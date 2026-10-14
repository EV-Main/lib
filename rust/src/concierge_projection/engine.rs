//! The pull loop: cursor → pull → decide → apply or park → advance.

use std::{
	collections::VecDeque,
	time::{Duration, SystemTime, UNIX_EPOCH},
};

use tracing::{debug, error, info, warn};

use super::{
	event::{ScopeEvent, ScopeEventBody, Stream, UserEvent, user_key},
	handler::Handler,
	source::{EventSource, Page, Pulled, SourceError, UnknownEvent},
	store::{ParkReason, Parked, ProjectionStore, StoreError},
};

const TARGET: &str = "ev_lib::concierge_projection";

/// concierge's own ceiling on `limit` (its `MAX_LIMIT`). A larger batch would come back
/// short every time and stop each drain after one page.
pub const MAX_BATCH: u32 = 500;

/// What to do with an event this build cannot read (docs/SCOPES-AND-APPROVALS.md §4.3 of
/// concierge).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum UnknownKind {
	/// Stop the stream's cursor on it until a build that can read it is deployed. For a
	/// consumer whose swallowed event costs money.
	Halt,
	/// Record its position, move on, and replay it by position on the next start. What a
	/// direction does: one unreadable row must not stop every suspension behind it.
	#[default]
	Park,
}

#[derive(Clone, Debug)]
pub struct ProjectionConfig {
	pub unknown_kind: UnknownKind,
	/// Events per pull, clamped to `1..=`[`MAX_BATCH`]. A page shorter than this is read as
	/// the stream's head, so it must not exceed what concierge will return.
	pub batch: u32,
	/// The wait between cycles that did not stall.
	pub poll_interval: Duration,
	/// The ceiling the wait doubles up to while cycles keep failing or stay halted.
	pub backoff_max: Duration,
}

impl Default for ProjectionConfig {
	fn default() -> Self {
		Self {
			unknown_kind: UnknownKind::Park,
			batch: 256,
			poll_interval: Duration::from_secs(5),
			backoff_max: Duration::from_secs(60),
		}
	}
}

#[derive(Debug, thiserror::Error)]
pub enum ProjectionError {
	#[error(transparent)]
	Source(#[from] SourceError),
	#[error(transparent)]
	Store(#[from] StoreError),
	#[error("projection handler failed on {stream:?} event {event_id}: {source}")]
	Handler {
		stream: Stream,
		event_id: String,
		#[source]
		source: Box<dyn std::error::Error + Send + Sync>,
	},
	/// concierge broke a rule of its own stream contract (a cursor that did not advance). The
	/// stream stops here rather than loop.
	#[error("concierge stream contract broken on {stream:?}: {detail}")]
	Protocol { stream: Stream, detail: String },
}

/// What one cycle did. `progressed` and `halted` drive the backoff; the counts are for the
/// caller's own reporting.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Cycle {
	/// Some cursor moved.
	pub progressed: bool,
	/// A stream stopped on an unreadable event under [`UnknownKind::Halt`].
	pub halted: bool,
	pub applied: u64,
	pub parked: u64,
	pub replayed: u64,
	/// Duplicates and events at or below their key's stored sequence.
	pub skipped: u64,
}

/// The engine. Generic over its three ports, so a test drives it with fakes.
pub struct Projection<St, H, Src> {
	store: St,
	handler: H,
	source: Src,
	config: ProjectionConfig,
}

#[derive(Clone, Debug)]
enum Event {
	User(UserEvent),
	Scope(ScopeEvent),
}

impl Event {
	fn event_id(&self) -> &str {
		match self {
			Self::User(e) => &e.event_id,
			Self::Scope(e) => &e.event_id,
		}
	}

	fn key(&self) -> String {
		match self {
			Self::User(e) => e.key(),
			Self::Scope(e) => e.key(),
		}
	}

	fn sequence(&self) -> u64 {
		match self {
			Self::User(e) => e.sequence,
			Self::Scope(e) => e.sequence,
		}
	}

	fn occurred_at(&self) -> i64 {
		match self {
			Self::User(e) => e.occurred_at,
			Self::Scope(e) => e.occurred_at,
		}
	}
}

fn widen_user(p: Pulled<UserEvent>) -> Pulled<Event> {
	match p {
		Pulled::Known(e) => Pulled::Known(Event::User(e)),
		Pulled::Unknown(u) => Pulled::Unknown(u),
	}
}

fn widen_scope(p: Pulled<ScopeEvent>) -> Pulled<Event> {
	match p {
		Pulled::Known(e) => Pulled::Known(Event::Scope(e)),
		Pulled::Unknown(u) => Pulled::Unknown(u),
	}
}

fn pulled_id(p: &Pulled<Event>) -> &str {
	match p {
		Pulled::Known(e) => e.event_id(),
		Pulled::Unknown(u) => &u.event_id,
	}
}

enum Decision {
	Apply,
	/// Already applied, or at/below the key's stored sequence.
	Skip {
		stale: bool,
	},
	Park(ParkReason),
	Halt,
}

/// How processing one event ended.
enum Step {
	/// Applied or skipped: the cursor may pass it.
	Done,
	/// Parked: the cursor may pass it too, the park row owns it now.
	Parked,
	/// It must be parked, but its position is unknown (the lifecycle stream's rows do not
	/// carry one): re-walk the batch one row at a time.
	NeedPosition,
	/// Replay only: it still cannot be applied and stays parked.
	StillParked,
	Halt,
}

impl<St, H, Src> Projection<St, H, Src>
where
	St: ProjectionStore,
	H: Handler,
	Src: EventSource,
{
	pub fn new(store: St, handler: H, source: Src, mut config: ProjectionConfig) -> Self {
		config.batch = config.batch.clamp(1, MAX_BATCH);
		Self { store, handler, source, config }
	}

	/// Run until the future is dropped. Replays what is parked first (a new build may now
	/// read it), then cycles: the scope stream before the lifecycle stream, so a user's
	/// MEMBER_JOINED is usually applied before their lifecycle row asks for it.
	///
	/// Dropping the future mid-cycle is safe: the cursor moves only after its events are
	/// done, so a cut-off event is re-pulled — re-applied if the handler's result was not
	/// recorded, or its MEMBER_JOINED follow-up re-run if only that was cut — never lost. Race it against the app's
	/// shutdown signal.
	pub async fn run(&self) {
		match self.replay_parked().await {
			Ok(replayed) if replayed > 0 => info!(target: TARGET, replayed, "concierge projection: replayed parked events"),
			Ok(_) => {}
			Err(e) => warn!(target: TARGET, error = %e, "concierge projection: replaying parked events failed; retrying next start"),
		}
		let ceiling = self.config.backoff_max.max(self.config.poll_interval);
		let mut backoff: Option<Duration> = None;
		loop {
			let mut cycle = Cycle::default();
			let failure = self.cycle(&mut cycle).await.err();
			// A cycle that moved a cursor is draining, even if a later pull failed; an idle one
			// is healthy. Only failing or halted with nothing committed is repeated work.
			let stalled = !cycle.progressed && (failure.is_some() || cycle.halted);
			let wait = if stalled {
				let next = backoff.map_or(self.config.poll_interval, |previous| previous.saturating_mul(2).min(ceiling));
				backoff = Some(next);
				next
			} else {
				backoff = None;
				self.config.poll_interval
			};
			if let Some(e) = failure {
				match e {
					ProjectionError::Protocol { .. } | ProjectionError::Source(SourceError::Refused(_) | SourceError::Config(_)) => {
						error!(target: TARGET, error = %e, retry_in = ?wait, "concierge projection: cycle failed and will not heal on its own")
					}
					ProjectionError::Source(_) | ProjectionError::Store(_) | ProjectionError::Handler { .. } => {
						warn!(target: TARGET, error = %e, retry_in = ?wait, progressed = cycle.progressed, "concierge projection: cycle failed, retrying")
					}
				}
			}
			tokio::time::sleep(wait).await;
		}
	}

	/// One pass over both streams, each drained to its head. `cycle` is filled in as it goes
	/// so it still says what was done when an error cut the pass short.
	pub async fn cycle(&self, cycle: &mut Cycle) -> Result<(), ProjectionError> {
		self.drain_scope(cycle).await?;
		self.drain_users(cycle).await?;
		Ok(())
	}

	/// Re-pull every parked event by position and apply what this build now can, key by key
	/// in position order. Returns how many left the park.
	pub async fn replay_parked(&self) -> Result<u64, ProjectionError> {
		let keys = self.store.parked_keys().await?;
		let mut cycle = Cycle::default();
		self.replay_keys(keys, &mut cycle).await?;
		Ok(cycle.replayed)
	}

	async fn drain_scope(&self, cycle: &mut Cycle) -> Result<(), ProjectionError> {
		let stream = Stream::Scope;
		let mut last_occurred = None;
		loop {
			let after = self.store.cursor(stream).await?;
			let page = self.source.pull_scope_events(after, self.config.batch).await?;
			let returned = page.events.len();
			check_advance(stream, after, &page)?;
			let mut done_through = after;
			for pulled in page.events {
				let position = match &pulled {
					Pulled::Known(e) => e.position,
					Pulled::Unknown(UnknownEvent { position: Some(p), .. }) => *p,
					Pulled::Unknown(UnknownEvent { position: None, event_id, .. }) => {
						return Err(ProjectionError::Protocol {
							stream,
							detail: format!("scope event {event_id} without a position"),
						});
					}
				};
				let pulled = widen_scope(pulled);
				if let Pulled::Known(e) = &pulled {
					last_occurred = Some(e.occurred_at());
				}
				let mut unblocked = Vec::new();
				let step = self.process(stream, pulled, Some(position), false, cycle, &mut unblocked).await?;
				self.replay_keys(unblocked, cycle).await?;
				match step {
					Step::Done | Step::Parked => done_through = position,
					Step::Halt => {
						self.advance(stream, after, done_through, cycle).await?;
						cycle.halted = true;
						return Ok(());
					}
					Step::NeedPosition | Step::StillParked => {
						return Err(ProjectionError::Protocol {
							stream,
							detail: "an event with a position asked for one".into(),
						});
					}
				}
			}
			self.advance(stream, after, page.next_position, cycle).await?;
			self.report(stream, page.next_position, last_occurred, cycle).await?;
			if !more(returned, self.config.batch, after, page.next_position) {
				return Ok(());
			}
		}
	}

	async fn drain_users(&self, cycle: &mut Cycle) -> Result<(), ProjectionError> {
		let stream = Stream::UserLifecycle;
		let mut last_occurred = None;
		loop {
			let after = self.store.cursor(stream).await?;
			let page = self.source.pull_user_events(after, self.config.batch).await?;
			let returned = page.events.len();
			check_advance(stream, after, &page)?;
			let mut walk = false;
			for pulled in page.events {
				let pulled = widen_user(pulled);
				if let Pulled::Known(e) = &pulled {
					last_occurred = Some(e.occurred_at());
				}
				// A lifecycle event never unblocks a key, so there is nothing to replay after it.
				match self.process(stream, pulled, None, false, cycle, &mut Vec::new()).await? {
					Step::Done | Step::Parked => {}
					// The cursor stays at `after`: what this batch applied re-applies as a
					// duplicate when an upgraded build pulls it again.
					Step::Halt => {
						cycle.halted = true;
						return Ok(());
					}
					Step::NeedPosition => {
						walk = true;
						break;
					}
					Step::StillParked => {
						return Err(ProjectionError::Protocol {
							stream,
							detail: "a live event reported as a replay".into(),
						});
					}
				}
			}
			if walk && self.walk_users(after, page.next_position, cycle).await? {
				cycle.halted = true;
				return Ok(());
			}
			self.advance(stream, after, page.next_position, cycle).await?;
			self.report(stream, page.next_position, last_occurred, cycle).await?;
			if !more(returned, self.config.batch, after, page.next_position) {
				return Ok(());
			}
		}
	}

	/// The lifecycle stream's rows carry no position, but parking needs one. Pull the batch
	/// again one row at a time: with `limit = 1` the answer's `next_position` is that row's
	/// position. What the first pass already applied is skipped as a duplicate. Returns
	/// whether the stream halted.
	async fn walk_users(&self, from: i64, until: i64, cycle: &mut Cycle) -> Result<bool, ProjectionError> {
		let stream = Stream::UserLifecycle;
		let mut cursor = from;
		while cursor < until {
			let page = self.source.pull_user_events(cursor, 1).await?;
			check_advance(stream, cursor, &page)?;
			let Some(pulled) = page.events.into_iter().next() else {
				if page.next_position > cursor {
					cursor = page.next_position;
					self.store.advance_cursor(stream, cursor).await?;
					continue;
				}
				break;
			};
			let position = page.next_position;
			match self.process(stream, widen_user(pulled), Some(position), false, cycle, &mut Vec::new()).await? {
				Step::Done | Step::Parked => {}
				Step::Halt => {
					self.store.advance_cursor(stream, cursor).await?;
					return Ok(true);
				}
				Step::NeedPosition | Step::StillParked => {
					return Err(ProjectionError::Protocol {
						stream,
						detail: "a walked event asked for a position it was given".into(),
					});
				}
			}
			self.store.advance_cursor(stream, position).await?;
			cycle.progressed = true;
			cursor = position;
		}
		Ok(false)
	}

	async fn decide(&self, pulled: &Pulled<Event>, replay: bool) -> Result<Decision, ProjectionError> {
		let event = match pulled {
			Pulled::Unknown(_) => {
				return Ok(match self.config.unknown_kind {
					UnknownKind::Halt => Decision::Halt,
					UnknownKind::Park => Decision::Park(ParkReason::UnknownKind),
				});
			}
			Pulled::Known(event) => event,
		};
		if self.store.is_applied(event.event_id()).await? {
			return Ok(Decision::Skip { stale: false });
		}
		let key = event.key();
		// A replay is the head of its key's queue; everything else queues behind what is
		// parked, or the sequence guard below would run past it.
		if !replay && self.store.has_parked(&key).await? {
			return Ok(Decision::Park(ParkReason::Behind));
		}
		let stored = self.store.sequence(&key).await?;
		// A user's lifecycle sequence row is written by their MEMBER_JOINED first, so its
		// absence means the projection has no row to apply this to yet.
		if matches!(event, Event::User(_)) && stored.is_none() {
			return Ok(Decision::Park(ParkReason::AwaitingMember));
		}
		if stored.is_some_and(|last| event.sequence() <= last) {
			return Ok(Decision::Skip { stale: true });
		}
		Ok(Decision::Apply)
	}

	/// Decide and act on one event. `position` is `None` only on the lifecycle stream's
	/// batched pull.
	async fn process(&self, stream: Stream, pulled: Pulled<Event>, position: Option<i64>, replay: bool, cycle: &mut Cycle, unblocked: &mut Vec<String>) -> Result<Step, ProjectionError> {
		match self.decide(&pulled, replay).await? {
			Decision::Halt => {
				if let Pulled::Unknown(u) = &pulled {
					error!(target: TARGET, stream = stream.as_str(), event_id = %u.event_id, kind = u.kind, why = u.why, "concierge projection: halted on an event this build cannot read; deploy a build that can (needs intervention)");
				}
				Ok(Step::Halt)
			}
			Decision::Park(_) if replay => Ok(Step::StillParked),
			Decision::Park(reason) => {
				let Some(position) = position else {
					return Ok(Step::NeedPosition);
				};
				let key = match &pulled {
					Pulled::Known(e) => e.key(),
					Pulled::Unknown(u) => u.key.clone(),
				};
				if let Pulled::Unknown(u) = &pulled {
					warn!(target: TARGET, stream = stream.as_str(), event_id = %u.event_id, position, kind = u.kind, why = u.why, "concierge projection: parked an event this build cannot read");
				}
				self.store
					.park(&Parked {
						stream,
						position,
						event_id: pulled_id(&pulled).to_owned(),
						key,
						reason,
					})
					.await?;
				cycle.parked += 1;
				Ok(Step::Parked)
			}
			Decision::Skip { stale } => {
				match (stale, &pulled) {
					// Recorded so a redelivery is caught by the cheaper id check next time.
					(true, Pulled::Known(e)) => self.store.record_applied(stream, e.event_id(), &e.key(), e.sequence()).await?,
					// A redelivered MEMBER_JOINED may be one whose follow-up never ran (the
					// cycle was cut between recording it and raising its user's floor);
					// without this, that user's lifecycle would stay parked for ever.
					(false, Pulled::Known(Event::Scope(e))) => self.after_member_joined(e, unblocked).await?,
					(false, Pulled::Known(Event::User(_))) | (_, Pulled::Unknown(_)) => {}
				}
				cycle.skipped += 1;
				Ok(Step::Done)
			}
			Decision::Apply => {
				let Pulled::Known(event) = pulled else {
					return Err(ProjectionError::Protocol {
						stream,
						detail: "an unreadable event decided as applicable".into(),
					});
				};
				self.apply(stream, &event, cycle, unblocked).await?;
				Ok(Step::Done)
			}
		}
	}

	/// A MEMBER_JOINED pushes its user key onto `unblocked` when lifecycle events wait on it;
	/// the caller replays them, so this never recurses into a replay.
	async fn apply(&self, stream: Stream, event: &Event, cycle: &mut Cycle, unblocked: &mut Vec<String>) -> Result<(), ProjectionError> {
		let handled = match event {
			Event::User(e) => self.handler.on_user_event(e).await,
			Event::Scope(e) => self.handler.on_scope_event(e).await,
		};
		handled.map_err(|source| ProjectionError::Handler {
			stream,
			event_id: event.event_id().to_owned(),
			source: Box::new(source),
		})?;
		self.store.record_applied(stream, event.event_id(), &event.key(), event.sequence()).await?;
		cycle.applied += 1;
		if let Event::Scope(e) = event {
			self.after_member_joined(e, unblocked).await?;
		}
		Ok(())
	}

	/// For a MEMBER_JOINED: raise its user's lifecycle floor to the snapshot's
	/// `lifecycle_sequence` (which is also what marks the user as seeded) and queue the
	/// user's parked lifecycle for replay. Idempotent, so it runs again on a redelivery —
	/// that is how a follow-up cut short after `record_applied` completes. A no-op for
	/// every other scope event.
	async fn after_member_joined(&self, event: &ScopeEvent, unblocked: &mut Vec<String>) -> Result<(), ProjectionError> {
		let ScopeEventBody::MemberJoined { auth_subject, member, .. } = &event.body else {
			return Ok(());
		};
		let key = user_key(auth_subject);
		self.store.raise_sequence(&key, member.lifecycle_sequence).await?;
		if self.store.has_parked(&key).await? {
			unblocked.push(key);
		}
		Ok(())
	}

	/// Replay each key's parked events in position order, stopping at the first that still
	/// cannot be applied. A work queue rather than recursion: a replayed MEMBER_JOINED
	/// unblocks its user's key in turn.
	async fn replay_keys(&self, keys: Vec<String>, cycle: &mut Cycle) -> Result<(), ProjectionError> {
		let mut queue: VecDeque<String> = keys.into();
		while let Some(key) = queue.pop_front() {
			for parked in self.store.parked(&key).await? {
				let pulled = match parked.stream {
					Stream::UserLifecycle => self.source.pull_user_events(parked.position - 1, 1).await?.events.into_iter().next().map(widen_user),
					Stream::Scope => self.source.pull_scope_events(parked.position - 1, 1).await?.events.into_iter().next().map(widen_scope),
				};
				let Some(pulled) = pulled else {
					error!(target: TARGET, key = %key, position = parked.position, event_id = %parked.event_id, "concierge projection: a parked position no longer resolves to any event (needs intervention)");
					break;
				};
				if pulled_id(&pulled) != parked.event_id {
					error!(target: TARGET, key = %key, position = parked.position, parked = %parked.event_id, pulled = %pulled_id(&pulled), "concierge projection: a parked position resolves to another event (needs intervention)");
					break;
				}
				let mut unblocked = Vec::new();
				let step = self.process(parked.stream, pulled, Some(parked.position), true, cycle, &mut unblocked).await?;
				queue.extend(unblocked);
				match step {
					Step::Done => {
						self.store.unpark(parked.stream, parked.position).await?;
						cycle.replayed += 1;
					}
					Step::StillParked | Step::Halt => break,
					Step::Parked | Step::NeedPosition => {
						return Err(ProjectionError::Protocol {
							stream: parked.stream,
							detail: "a replay parked an event again".into(),
						});
					}
				}
			}
		}
		Ok(())
	}

	async fn advance(&self, stream: Stream, from: i64, to: i64, cycle: &mut Cycle) -> Result<(), ProjectionError> {
		if to > from {
			self.store.advance_cursor(stream, to).await?;
			cycle.progressed = true;
		}
		Ok(())
	}

	/// The lag metric: how far behind the stream's head the projection is, in the time
	/// between an event happening and this build applying it.
	async fn report(&self, stream: Stream, cursor: i64, last_occurred: Option<i64>, cycle: &Cycle) -> Result<(), ProjectionError> {
		let parked_total = self.store.parked_count().await?;
		let now = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| i64::try_from(d.as_secs()).unwrap_or(i64::MAX));
		let lag_secs = last_occurred.map(|at| now.saturating_sub(at));
		if cycle.applied > 0 || cycle.parked > 0 {
			info!(target: TARGET, stream = stream.as_str(), cursor, applied = cycle.applied, parked = cycle.parked, skipped = cycle.skipped, parked_total, lag_secs, "concierge projection: progressed");
		} else {
			debug!(target: TARGET, stream = stream.as_str(), cursor, parked_total, lag_secs, "concierge projection: idle");
		}
		Ok(())
	}
}

/// A non-empty answer must move the cursor; otherwise the next pull is this one again and
/// the loop would hammer concierge with it.
fn check_advance<T>(stream: Stream, after: i64, page: &Page<T>) -> Result<(), ProjectionError> {
	if !page.events.is_empty() && page.next_position <= after {
		return Err(ProjectionError::Protocol {
			stream,
			detail: format!("{} events returned but next_position {} did not pass {after}", page.events.len(), page.next_position),
		});
	}
	Ok(())
}

/// Pull again while a full batch came back, or while an empty one still moved the cursor
/// (rows scanned that were not this caller's to see).
fn more(returned: usize, batch: u32, after: i64, next_position: i64) -> bool {
	let full = usize::try_from(batch).is_ok_and(|b| returned >= b);
	full || (returned == 0 && next_position > after)
}
