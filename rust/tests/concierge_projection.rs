//! `concierge_projection`: the pull engine on a fake concierge and a recording handler over
//! the real SQLite store (in memory); the SQLite store's contract and migration; and
//! `GrpcSource`'s reading of the wire, against an in-process fake of concierge's streams.
#![cfg(feature = "concierge_projection_sqlite")]

#[macro_use]
#[path = "support/projection_store.rs"]
mod projection_store;

use std::{
	collections::{BTreeMap, VecDeque},
	io::Write,
	sync::{Arc, Mutex},
	time::Duration,
};

use async_trait::async_trait;
use ev_lib::concierge_projection::{
	Cycle, EventSource, GrpcSource, Handler, Member, Page, ParkReason, Parked, Projection, ProjectionConfig, ProjectionError, ProjectionStore, Pulled, ScopeEvent, ScopeEventBody,
	ScopeState, SourceError, SqliteProjectionStore, StoreError, Stream, UnknownEvent, UnknownKind, UserEvent, UserEventKind, sqlite,
};
use evconcierge_contracts::concierge::v1 as wire;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use tokio::time::Instant;

const SCOPE: &str = "sa";
const OCCURRED_AT: i64 = 1_760_000_000;

// --- fixtures -----------------------------------------------------------------------------

async fn sqlite_pool() -> SqlitePool {
	// One connection: every `sqlite::memory:` connection is its own database.
	let pool = SqlitePoolOptions::new()
		.max_connections(1)
		.idle_timeout(None)
		.max_lifetime(None)
		.connect("sqlite::memory:")
		.await
		.unwrap();
	sqlite::migrate(&pool).await.unwrap();
	pool
}

async fn fresh_sqlite_store() -> Option<(SqliteProjectionStore, ())> {
	Some((SqliteProjectionStore::new(sqlite_pool().await), ()))
}

async fn no_teardown(_: ()) {}

fn user_event(event_id: &str, subject: &str, kind: UserEventKind, sequence: u64) -> UserEvent {
	UserEvent {
		event_id: event_id.to_owned(),
		kind,
		user_id: format!("uid-{subject}"),
		auth_subject: subject.to_owned(),
		sequence,
		occurred_at: OCCURRED_AT,
		kyc_level: 1,
		email: format!("{subject}@example.com"),
		email_verified: true,
		token_version: 1,
		role: "investor".to_owned(),
	}
}

fn grants(event_id: &str, position: i64, subject: &str, sequence: u64) -> ScopeEvent {
	ScopeEvent {
		event_id: event_id.to_owned(),
		position,
		scope: SCOPE.to_owned(),
		occurred_at: OCCURRED_AT,
		sequence,
		body: ScopeEventBody::GrantsChanged {
			user_id: format!("uid-{subject}"),
			auth_subject: subject.to_owned(),
			permissions: vec!["sa.leads.read".to_owned()],
		},
	}
}

fn joined(event_id: &str, position: i64, subject: &str, lifecycle_sequence: u64) -> ScopeEvent {
	ScopeEvent {
		event_id: event_id.to_owned(),
		position,
		scope: SCOPE.to_owned(),
		occurred_at: OCCURRED_AT,
		sequence: 1,
		body: ScopeEventBody::MemberJoined {
			user_id: format!("uid-{subject}"),
			auth_subject: subject.to_owned(),
			member: Member {
				email: format!("{subject}@example.com"),
				email_verified: true,
				username: String::new(),
				preferred_name: subject.to_owned(),
				status: "active".to_owned(),
				kyc_level: 1,
				token_version: 1,
				lifecycle_sequence,
			},
		},
	}
}

fn unknown_scope_event(event_id: &str, position: i64, subject: &str) -> UnknownEvent {
	UnknownEvent {
		event_id: event_id.to_owned(),
		position: Some(position),
		key: format!("scope:{SCOPE}:user:{subject}"),
		kind: 9,
		why: "a scope event kind this build cannot name",
	}
}

fn unknown_user_event(event_id: &str, subject: &str) -> UnknownEvent {
	UnknownEvent {
		event_id: event_id.to_owned(),
		position: None,
		key: format!("user:{subject}"),
		kind: 8,
		why: "a lifecycle kind this build cannot name",
	}
}

fn parked(stream: Stream, position: i64, event_id: &str, key: &str, reason: ParkReason) -> Parked {
	Parked {
		stream,
		position,
		event_id: event_id.to_owned(),
		key: key.to_owned(),
		reason,
	}
}

/// A stored row of the fake concierge: readable by this build, or of a kind only an
/// upgraded build can read.
#[derive(Clone)]
enum Row<T> {
	Known(T),
	FromTheFuture { today: UnknownEvent, upgraded: T },
}

impl<T: Clone> Row<T> {
	fn pulled(&self, upgraded: bool) -> Pulled<T> {
		match self {
			Self::Known(event) => Pulled::Known(event.clone()),
			Self::FromTheFuture { upgraded: event, .. } if upgraded => Pulled::Known(event.clone()),
			Self::FromTheFuture { today, .. } => Pulled::Unknown(today.clone()),
		}
	}
}

/// concierge's two streams, positions as concierge keeps them. `after` is exclusive; an
/// answer's `next_position` is the last row returned, or `after` when none was.
#[derive(Default)]
struct Streams {
	users: BTreeMap<i64, Row<UserEvent>>,
	scopes: BTreeMap<i64, Row<ScopeEvent>>,
	upgraded: bool,
	scripted_scope_pages: VecDeque<Page<Pulled<ScopeEvent>>>,
	failing_pulls: usize,
	scope_pulls_at: Vec<Instant>,
}

fn page<T: Clone>(rows: &BTreeMap<i64, Row<T>>, upgraded: bool, after: i64, limit: u32) -> Page<Pulled<T>> {
	let taken: Vec<(i64, Pulled<T>)> = rows.range(after + 1..).take(limit as usize).map(|(position, row)| (*position, row.pulled(upgraded))).collect();
	Page {
		next_position: taken.last().map_or(after, |(position, _)| *position),
		events: taken.into_iter().map(|(_, pulled)| pulled).collect(),
	}
}

#[derive(Clone, Default)]
struct FakeConcierge(Arc<Mutex<Streams>>);

impl FakeConcierge {
	fn user(&self, position: i64, row: Row<UserEvent>) {
		self.0.lock().unwrap().users.insert(position, row);
	}

	fn scope(&self, position: i64, row: Row<ScopeEvent>) {
		self.0.lock().unwrap().scopes.insert(position, row);
	}

	fn remove_scope(&self, position: i64) {
		self.0.lock().unwrap().scopes.remove(&position);
	}

	/// Deploy a build that reads every kind.
	fn upgrade(&self) {
		self.0.lock().unwrap().upgraded = true;
	}

	/// The next scope pull answers `page` instead of the stored rows.
	fn script_scope_page(&self, page: Page<Pulled<ScopeEvent>>) {
		self.0.lock().unwrap().scripted_scope_pages.push_back(page);
	}

	fn fail_pulls(&self, count: usize) {
		self.0.lock().unwrap().failing_pulls = count;
	}

	fn scope_pulls_since(&self, start: Instant) -> Vec<u64> {
		self.0.lock().unwrap().scope_pulls_at.iter().map(|at| at.duration_since(start).as_secs()).collect()
	}
}

#[async_trait]
impl EventSource for FakeConcierge {
	async fn pull_user_events(&self, after: i64, limit: u32) -> Result<Page<Pulled<UserEvent>>, SourceError> {
		let streams = self.0.lock().unwrap();
		Ok(page(&streams.users, streams.upgraded, after, limit))
	}

	async fn pull_scope_events(&self, after: i64, limit: u32) -> Result<Page<Pulled<ScopeEvent>>, SourceError> {
		let mut streams = self.0.lock().unwrap();
		streams.scope_pulls_at.push(Instant::now());
		if streams.failing_pulls > 0 {
			streams.failing_pulls -= 1;
			return Err(SourceError::Unavailable("concierge is restarting".into()));
		}
		if let Some(scripted) = streams.scripted_scope_pages.pop_front() {
			return Ok(scripted);
		}
		Ok(page(&streams.scopes, streams.upgraded, after, limit))
	}
}

/// The projection: records the event ids it applied, in order; fails on demand.
#[derive(Clone, Default)]
struct Recorder {
	applied: Arc<Mutex<Vec<String>>>,
	failing_on: Arc<Mutex<Option<String>>>,
}

impl Recorder {
	fn applied(&self) -> Vec<String> {
		self.applied.lock().unwrap().clone()
	}

	fn fail_on(&self, event_id: Option<&str>) {
		*self.failing_on.lock().unwrap() = event_id.map(str::to_owned);
	}

	fn handle(&self, event_id: &str) -> Result<(), std::io::Error> {
		if self.failing_on.lock().unwrap().as_deref() == Some(event_id) {
			return Err(std::io::Error::other("projection database is down"));
		}
		self.applied.lock().unwrap().push(event_id.to_owned());
		Ok(())
	}
}

#[async_trait]
impl Handler for Recorder {
	type Error = std::io::Error;

	async fn on_user_event(&self, event: &UserEvent) -> Result<(), Self::Error> {
		self.handle(&event.event_id)
	}

	async fn on_scope_event(&self, event: &ScopeEvent) -> Result<(), Self::Error> {
		self.handle(&event.event_id)
	}
}

struct Harness {
	projection: Projection<SqliteProjectionStore, Recorder, FakeConcierge>,
	pool: SqlitePool,
	store: SqliteProjectionStore,
	handler: Recorder,
	concierge: FakeConcierge,
}

impl Harness {
	async fn new(unknown_kind: UnknownKind) -> Self {
		Self::with_batch(unknown_kind, 256).await
	}

	async fn with_batch(unknown_kind: UnknownKind, batch: u32) -> Self {
		let pool = sqlite_pool().await;
		let store = SqliteProjectionStore::new(pool.clone());
		let handler = Recorder::default();
		let concierge = FakeConcierge::default();
		let config = ProjectionConfig {
			unknown_kind,
			batch,
			poll_interval: Duration::from_secs(5),
			backoff_max: Duration::from_secs(20),
		};
		Self {
			projection: Projection::new(store.clone(), handler.clone(), concierge.clone(), config),
			pool,
			store,
			handler,
			concierge,
		}
	}

	async fn cycle(&self) -> (Result<(), ProjectionError>, Cycle) {
		let mut cycle = Cycle::default();
		let result = self.projection.cycle(&mut cycle).await;
		(result, cycle)
	}
}

/// What the engine logged while the guard lives.
#[derive(Clone, Default)]
struct Log(Arc<Mutex<Vec<u8>>>);

impl Log {
	fn capture(&self) -> tracing::subscriber::DefaultGuard {
		let writer = self.clone();
		tracing::subscriber::set_default(tracing_subscriber::fmt().with_writer(move || writer.clone()).with_ansi(false).finish())
	}

	fn text(&self) -> String {
		String::from_utf8(self.0.lock().unwrap().clone()).unwrap()
	}
}

impl Write for Log {
	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
		self.0.lock().unwrap().write(buf)
	}

	fn flush(&mut self) -> std::io::Result<()> {
		Ok(())
	}
}

// --- dedupe and per-key ordering ----------------------------------------------------------

#[tokio::test]
async fn should_hand_a_redelivered_event_id_to_the_handler_once() {
	let h = Harness::new(UnknownKind::Park).await;
	h.concierge.scope(1, Row::Known(joined("j-ann", 1, "ann", 0)));
	h.concierge.user(1, Row::Known(user_event("u-1", "ann", UserEventKind::Suspended, 1)));
	h.concierge.user(2, Row::Known(user_event("u-1", "ann", UserEventKind::Suspended, 1)));

	let (result, cycle) = h.cycle().await;

	result.unwrap();
	assert_eq!(h.handler.applied(), ["j-ann", "u-1"]);
	assert_eq!(cycle.skipped, 1);
}

#[tokio::test]
async fn should_not_hand_over_an_event_at_the_key_stored_sequence() {
	let h = Harness::new(UnknownKind::Park).await;
	h.concierge.scope(1, Row::Known(joined("j-ann", 1, "ann", 0)));
	h.concierge.user(1, Row::Known(user_event("u-1", "ann", UserEventKind::Suspended, 4)));
	h.concierge.user(2, Row::Known(user_event("u-2", "ann", UserEventKind::KycChanged, 4)));

	let (result, _) = h.cycle().await;

	result.unwrap();
	assert_eq!(h.handler.applied(), ["j-ann", "u-1"]);
}

#[tokio::test]
async fn should_not_let_a_stale_reinstated_undo_a_later_suspended() {
	let h = Harness::new(UnknownKind::Park).await;
	h.concierge.scope(1, Row::Known(joined("j-ann", 1, "ann", 0)));
	h.concierge.user(1, Row::Known(user_event("u-suspended", "ann", UserEventKind::Suspended, 5)));
	h.concierge.user(2, Row::Known(user_event("u-reinstated", "ann", UserEventKind::Reinstated, 4)));

	let (result, _) = h.cycle().await;

	result.unwrap();
	assert_eq!(h.handler.applied(), ["j-ann", "u-suspended"]);
}

// --- lifecycle before MEMBER_JOINED -------------------------------------------------------

#[tokio::test]
async fn should_park_a_lifecycle_event_that_arrives_before_its_member_joined() {
	let h = Harness::new(UnknownKind::Park).await;
	h.concierge.user(1, Row::Known(user_event("u-1", "bob", UserEventKind::Suspended, 3)));

	let (result, cycle) = h.cycle().await;

	result.unwrap();
	assert_eq!(h.handler.applied(), Vec::<String>::new());
	assert_eq!(
		h.store.parked("user:bob").await.unwrap(),
		[parked(Stream::UserLifecycle, 1, "u-1", "user:bob", ParkReason::AwaitingMember)]
	);
	assert_eq!(h.store.cursor(Stream::UserLifecycle).await.unwrap(), 1);
	assert_eq!(cycle.parked, 1);
}

#[tokio::test]
async fn should_apply_a_parked_lifecycle_event_above_lifecycle_sequence_once_member_joined_lands() {
	let h = Harness::new(UnknownKind::Park).await;
	h.concierge.user(1, Row::Known(user_event("u-1", "bob", UserEventKind::Suspended, 3)));
	h.cycle().await.0.unwrap();
	h.concierge.scope(1, Row::Known(joined("j-bob", 1, "bob", 2)));

	let (result, cycle) = h.cycle().await;

	result.unwrap();
	assert_eq!(h.handler.applied(), ["j-bob", "u-1"]);
	assert_eq!(h.store.parked_count().await.unwrap(), 0);
	assert_eq!(cycle.replayed, 1);
}

#[tokio::test]
async fn should_drop_a_parked_lifecycle_event_at_lifecycle_sequence_once_member_joined_lands() {
	let h = Harness::new(UnknownKind::Park).await;
	h.concierge.user(1, Row::Known(user_event("u-1", "bob", UserEventKind::Suspended, 3)));
	h.cycle().await.0.unwrap();
	h.concierge.scope(1, Row::Known(joined("j-bob", 1, "bob", 3)));

	let (result, _) = h.cycle().await;

	result.unwrap();
	assert_eq!(h.handler.applied(), ["j-bob"]);
	assert_eq!(h.store.parked_count().await.unwrap(), 0);
}

#[tokio::test]
async fn should_apply_and_unblock_a_queued_lifecycle_key_in_order_once_member_joined_lands() {
	let h = Harness::new(UnknownKind::Park).await;
	h.concierge.user(1, Row::Known(user_event("u-1", "bob", UserEventKind::Suspended, 3)));
	h.concierge.user(2, Row::Known(user_event("u-2", "bob", UserEventKind::Reinstated, 4)));
	h.cycle().await.0.unwrap();
	h.concierge.scope(1, Row::Known(joined("j-bob", 1, "bob", 3)));

	let (result, _) = h.cycle().await;

	result.unwrap();
	assert_eq!(h.handler.applied(), ["j-bob", "u-2"]);
	assert_eq!(h.store.parked_count().await.unwrap(), 0);
}

/// A MEMBER_JOINED is recorded as applied before its user's lifecycle sequence is raised,
/// in two units of work. Cut between them (a database blip; `run()` dropped at shutdown,
/// which GUIDE.md calls safe) and the re-pulled MEMBER_JOINED is skipped as a duplicate,
/// so the raise never happens and every later lifecycle event of the user parks as
/// AwaitingMember — a SUSPENDED included.
#[tokio::test]
async fn should_apply_lifecycle_events_after_a_member_joined_whose_bookkeeping_was_cut_short() {
	let h = Harness::new(UnknownKind::Park).await;
	h.concierge.scope(1, Row::Known(joined("j-bob", 1, "bob", 0)));
	h.concierge.user(1, Row::Known(user_event("u-suspended", "bob", UserEventKind::Suspended, 1)));
	sqlx::query(
		"CREATE TRIGGER blip BEFORE INSERT ON concierge_projection_sequence WHEN NEW.key = 'user:bob'
		 BEGIN SELECT RAISE(ABORT, 'database blip'); END",
	)
	.execute(&h.pool)
	.await
	.unwrap();
	let cut = h.cycle().await.0.unwrap_err();
	assert!(matches!(cut, ProjectionError::Store(StoreError::Backend { .. })), "{cut:?}");
	sqlx::query("DROP TRIGGER blip").execute(&h.pool).await.unwrap();

	let (result, _) = h.cycle().await;

	result.unwrap();
	assert_eq!(
		h.handler.applied(),
		["j-bob", "u-suspended"],
		"bob's SUSPENDED was not applied; parked: {:?}",
		h.store.parked("user:bob").await.unwrap()
	);
}

// --- unknown kinds: Park ------------------------------------------------------------------

#[tokio::test]
async fn should_park_an_unknown_scope_kind_and_queue_its_key_behind_it_while_other_keys_flow() {
	let h = Harness::new(UnknownKind::Park).await;
	h.concierge.scope(
		1,
		Row::FromTheFuture {
			today: unknown_scope_event("s-new", 1, "ann"),
			upgraded: grants("s-new", 1, "ann", 1),
		},
	);
	h.concierge.scope(2, Row::Known(grants("g-ann", 2, "ann", 2)));
	h.concierge.scope(3, Row::Known(grants("g-carol", 3, "carol", 1)));

	let (result, cycle) = h.cycle().await;

	result.unwrap();
	assert_eq!(h.handler.applied(), ["g-carol"]);
	assert_eq!(
		h.store.parked("scope:sa:user:ann").await.unwrap(),
		[
			parked(Stream::Scope, 1, "s-new", "scope:sa:user:ann", ParkReason::UnknownKind),
			parked(Stream::Scope, 2, "g-ann", "scope:sa:user:ann", ParkReason::Behind),
		]
	);
	assert_eq!(h.store.cursor(Stream::Scope).await.unwrap(), 3);
	assert!(!cycle.halted);
}

#[tokio::test]
async fn should_apply_a_parked_scope_key_in_position_order_after_an_upgrade() {
	let h = Harness::new(UnknownKind::Park).await;
	h.concierge.scope(
		1,
		Row::FromTheFuture {
			today: unknown_scope_event("s-new", 1, "ann"),
			upgraded: grants("s-new", 1, "ann", 1),
		},
	);
	h.concierge.scope(2, Row::Known(grants("g-ann", 2, "ann", 2)));
	h.concierge.scope(3, Row::Known(grants("g-carol", 3, "carol", 1)));
	h.cycle().await.0.unwrap();
	h.concierge.upgrade();

	let replayed = h.projection.replay_parked().await.unwrap();

	assert_eq!(replayed, 2);
	assert_eq!(h.handler.applied(), ["g-carol", "s-new", "g-ann"]);
	assert_eq!(h.store.parked_count().await.unwrap(), 0);
}

#[tokio::test]
async fn should_park_an_unknown_lifecycle_kind_at_its_position_and_queue_its_key_behind_it() {
	let h = Harness::new(UnknownKind::Park).await;
	h.concierge.scope(1, Row::Known(joined("j-ann", 1, "ann", 0)));
	h.concierge.scope(2, Row::Known(joined("j-carol", 2, "carol", 0)));
	h.concierge.user(1, Row::Known(user_event("u-1", "ann", UserEventKind::Suspended, 1)));
	h.concierge.user(
		2,
		Row::FromTheFuture {
			today: unknown_user_event("u-new", "ann"),
			upgraded: user_event("u-new", "ann", UserEventKind::KycChanged, 2),
		},
	);
	h.concierge.user(3, Row::Known(user_event("u-3", "ann", UserEventKind::Reinstated, 3)));
	h.concierge.user(4, Row::Known(user_event("u-carol", "carol", UserEventKind::RoleChanged, 1)));

	let (result, _) = h.cycle().await;

	result.unwrap();
	assert_eq!(h.handler.applied(), ["j-ann", "j-carol", "u-1", "u-carol"]);
	assert_eq!(
		h.store.parked("user:ann").await.unwrap(),
		[
			parked(Stream::UserLifecycle, 2, "u-new", "user:ann", ParkReason::UnknownKind),
			parked(Stream::UserLifecycle, 3, "u-3", "user:ann", ParkReason::Behind),
		]
	);
	assert_eq!(h.store.cursor(Stream::UserLifecycle).await.unwrap(), 4);
}

#[tokio::test]
async fn should_apply_a_parked_lifecycle_key_in_position_order_after_an_upgrade() {
	let h = Harness::new(UnknownKind::Park).await;
	h.concierge.scope(1, Row::Known(joined("j-ann", 1, "ann", 0)));
	h.concierge.user(
		1,
		Row::FromTheFuture {
			today: unknown_user_event("u-new", "ann"),
			upgraded: user_event("u-new", "ann", UserEventKind::KycChanged, 1),
		},
	);
	h.concierge.user(2, Row::Known(user_event("u-2", "ann", UserEventKind::Suspended, 2)));
	h.cycle().await.0.unwrap();
	h.concierge.upgrade();

	let replayed = h.projection.replay_parked().await.unwrap();

	assert_eq!(replayed, 2);
	assert_eq!(h.handler.applied(), ["j-ann", "u-new", "u-2"]);
}

#[tokio::test]
async fn should_keep_an_unknown_kind_parked_when_the_build_still_cannot_read_it() {
	let h = Harness::new(UnknownKind::Park).await;
	h.concierge.scope(
		1,
		Row::FromTheFuture {
			today: unknown_scope_event("s-new", 1, "ann"),
			upgraded: grants("s-new", 1, "ann", 1),
		},
	);
	h.cycle().await.0.unwrap();

	let replayed = h.projection.replay_parked().await.unwrap();

	assert_eq!(replayed, 0);
	assert_eq!(h.store.parked_count().await.unwrap(), 1);
}

// --- the lifecycle stream's positions -----------------------------------------------------

#[tokio::test]
async fn should_park_a_lifecycle_event_at_the_position_a_single_row_pull_reports() {
	let h = Harness::new(UnknownKind::Park).await;
	h.concierge.scope(1, Row::Known(joined("j-ann", 1, "ann", 0)));
	h.concierge.user(4, Row::Known(user_event("u-ann-1", "ann", UserEventKind::Suspended, 1)));
	h.concierge.user(7, Row::Known(user_event("u-bob", "bob", UserEventKind::Suspended, 1)));
	h.concierge.user(9, Row::Known(user_event("u-ann-2", "ann", UserEventKind::Reinstated, 2)));

	let (result, _) = h.cycle().await;

	result.unwrap();
	assert_eq!(
		h.store.parked("user:bob").await.unwrap(),
		[parked(Stream::UserLifecycle, 7, "u-bob", "user:bob", ParkReason::AwaitingMember)]
	);
	assert_eq!(h.handler.applied(), ["j-ann", "u-ann-1", "u-ann-2"]);
	assert_eq!(h.store.cursor(Stream::UserLifecycle).await.unwrap(), 9);
}

#[tokio::test]
async fn should_park_by_position_in_a_later_batch_of_the_lifecycle_stream() {
	let h = Harness::with_batch(UnknownKind::Park, 2).await;
	h.concierge.scope(1, Row::Known(joined("j-ann", 1, "ann", 0)));
	h.concierge.user(1, Row::Known(user_event("u-1", "ann", UserEventKind::Suspended, 1)));
	h.concierge.user(2, Row::Known(user_event("u-2", "ann", UserEventKind::Reinstated, 2)));
	h.concierge.user(3, Row::Known(user_event("u-bob", "bob", UserEventKind::Suspended, 1)));
	h.concierge.user(4, Row::Known(user_event("u-4", "ann", UserEventKind::KycChanged, 3)));

	let (result, _) = h.cycle().await;

	result.unwrap();
	assert_eq!(
		h.store.parked("user:bob").await.unwrap(),
		[parked(Stream::UserLifecycle, 3, "u-bob", "user:bob", ParkReason::AwaitingMember)]
	);
	assert_eq!(h.handler.applied(), ["j-ann", "u-1", "u-2", "u-4"]);
	assert_eq!(h.store.cursor(Stream::UserLifecycle).await.unwrap(), 4);
}

#[tokio::test]
async fn should_drain_a_stream_longer_than_one_batch_in_one_cycle() {
	let h = Harness::with_batch(UnknownKind::Park, 2).await;
	h.concierge.scope(1, Row::Known(grants("g-1", 1, "ann", 1)));
	h.concierge.scope(2, Row::Known(grants("g-2", 2, "bob", 1)));
	h.concierge.scope(3, Row::Known(grants("g-3", 3, "carol", 1)));
	h.concierge.scope(4, Row::Known(grants("g-4", 4, "dave", 1)));
	h.concierge.scope(5, Row::Known(grants("g-5", 5, "erin", 1)));

	let (result, cycle) = h.cycle().await;

	result.unwrap();
	assert_eq!(h.handler.applied(), ["g-1", "g-2", "g-3", "g-4", "g-5"]);
	assert_eq!(h.store.cursor(Stream::Scope).await.unwrap(), 5);
	assert_eq!(cycle.applied, 5);
}

// --- unknown kinds: Halt ------------------------------------------------------------------

#[tokio::test]
async fn should_halt_the_scope_stream_with_its_cursor_on_the_event_before_an_unknown_kind() {
	let h = Harness::new(UnknownKind::Halt).await;
	h.concierge.scope(1, Row::Known(grants("g-carol", 1, "carol", 1)));
	h.concierge.scope(
		2,
		Row::FromTheFuture {
			today: unknown_scope_event("s-new", 2, "ann"),
			upgraded: grants("s-new", 2, "ann", 1),
		},
	);
	h.concierge.scope(3, Row::Known(grants("g-dave", 3, "dave", 1)));

	let (result, cycle) = h.cycle().await;

	result.unwrap();
	assert!(cycle.halted);
	assert_eq!(h.handler.applied(), ["g-carol"]);
	assert_eq!(h.store.cursor(Stream::Scope).await.unwrap(), 1);
	assert_eq!(h.store.parked_count().await.unwrap(), 0);
}

#[tokio::test]
async fn should_hold_the_lifecycle_cursor_on_an_unknown_kind_under_halt() {
	let h = Harness::new(UnknownKind::Halt).await;
	h.concierge.scope(1, Row::Known(joined("j-ann", 1, "ann", 0)));
	h.concierge.user(1, Row::Known(user_event("u-1", "ann", UserEventKind::Suspended, 1)));
	h.concierge.user(
		2,
		Row::FromTheFuture {
			today: unknown_user_event("u-new", "ann"),
			upgraded: user_event("u-new", "ann", UserEventKind::KycChanged, 2),
		},
	);
	h.concierge.user(3, Row::Known(user_event("u-3", "ann", UserEventKind::Reinstated, 3)));

	let (result, cycle) = h.cycle().await;

	result.unwrap();
	assert!(cycle.halted);
	assert_eq!(h.handler.applied(), ["j-ann", "u-1"]);
	assert_eq!(h.store.cursor(Stream::UserLifecycle).await.unwrap(), 0);
	assert_eq!(h.store.parked_count().await.unwrap(), 0);
}

#[tokio::test]
async fn should_park_what_precedes_an_unknown_lifecycle_kind_before_halting_on_it() {
	let h = Harness::new(UnknownKind::Halt).await;
	h.concierge.user(1, Row::Known(user_event("u-bob", "bob", UserEventKind::Suspended, 1)));
	h.concierge.user(
		2,
		Row::FromTheFuture {
			today: unknown_user_event("u-new", "ann"),
			upgraded: user_event("u-new", "ann", UserEventKind::KycChanged, 1),
		},
	);

	let (result, cycle) = h.cycle().await;

	result.unwrap();
	assert!(cycle.halted);
	assert_eq!(
		h.store.parked("user:bob").await.unwrap(),
		[parked(Stream::UserLifecycle, 1, "u-bob", "user:bob", ParkReason::AwaitingMember)]
	);
	assert_eq!(h.store.cursor(Stream::UserLifecycle).await.unwrap(), 1);
}

#[tokio::test]
async fn should_resume_a_halted_stream_in_order_once_the_build_can_read_the_event() {
	let h = Harness::new(UnknownKind::Halt).await;
	h.concierge.scope(1, Row::Known(grants("g-carol", 1, "carol", 1)));
	h.concierge.scope(
		2,
		Row::FromTheFuture {
			today: unknown_scope_event("s-new", 2, "ann"),
			upgraded: grants("s-new", 2, "ann", 1),
		},
	);
	h.concierge.scope(3, Row::Known(grants("g-dave", 3, "dave", 1)));
	h.cycle().await.0.unwrap();
	h.concierge.upgrade();

	let (result, cycle) = h.cycle().await;

	result.unwrap();
	assert!(!cycle.halted);
	assert_eq!(h.handler.applied(), ["g-carol", "s-new", "g-dave"]);
	assert_eq!(h.store.cursor(Stream::Scope).await.unwrap(), 3);
}

// --- replay that cannot resolve -----------------------------------------------------------

#[tokio::test]
async fn should_keep_a_key_parked_and_log_when_its_position_resolves_to_another_event() {
	let h = Harness::new(UnknownKind::Park).await;
	h.concierge.scope(
		1,
		Row::FromTheFuture {
			today: unknown_scope_event("s-new", 1, "ann"),
			upgraded: grants("s-new", 1, "ann", 1),
		},
	);
	h.cycle().await.0.unwrap();
	h.concierge.scope(1, Row::Known(grants("g-other", 1, "ann", 1)));
	let log = Log::default();
	let _guard = log.capture();

	let replayed = h.projection.replay_parked().await.unwrap();

	assert_eq!(replayed, 0);
	assert_eq!(
		h.store.parked("scope:sa:user:ann").await.unwrap(),
		[parked(Stream::Scope, 1, "s-new", "scope:sa:user:ann", ParkReason::UnknownKind)]
	);
	assert_eq!(h.handler.applied(), Vec::<String>::new());
	let text = log.text();
	assert!(text.contains("ERROR") && text.contains("resolves to another event (needs intervention)"), "{text}");
}

#[tokio::test]
async fn should_keep_a_key_parked_and_log_when_its_position_no_longer_resolves() {
	let h = Harness::new(UnknownKind::Park).await;
	h.concierge.scope(
		1,
		Row::FromTheFuture {
			today: unknown_scope_event("s-new", 1, "ann"),
			upgraded: grants("s-new", 1, "ann", 1),
		},
	);
	h.cycle().await.0.unwrap();
	h.concierge.remove_scope(1);
	h.concierge.upgrade();
	let log = Log::default();
	let _guard = log.capture();

	let replayed = h.projection.replay_parked().await.unwrap();

	assert_eq!(replayed, 0);
	assert_eq!(h.store.parked_count().await.unwrap(), 1);
	let text = log.text();
	assert!(text.contains("ERROR") && text.contains("no longer resolves to any event (needs intervention)"), "{text}");
}

// --- the stream contract ------------------------------------------------------------------

#[tokio::test]
async fn should_stop_with_a_protocol_error_when_a_non_empty_page_does_not_move_the_cursor() {
	let h = Harness::new(UnknownKind::Park).await;
	h.concierge.script_scope_page(Page {
		events: vec![Pulled::Known(grants("g-carol", 5, "carol", 1))],
		next_position: 0,
	});

	let (result, _) = h.cycle().await;

	let error = result.unwrap_err();
	assert!(matches!(error, ProjectionError::Protocol { stream: Stream::Scope, .. }), "{error:?}");
	assert_eq!(h.handler.applied(), Vec::<String>::new());
	assert_eq!(h.store.cursor(Stream::Scope).await.unwrap(), 0);
}

#[tokio::test]
async fn should_follow_an_empty_page_that_moves_the_cursor_and_pull_again() {
	let h = Harness::new(UnknownKind::Park).await;
	h.concierge.script_scope_page(Page { events: vec![], next_position: 10 });
	h.concierge.scope(12, Row::Known(grants("g-carol", 12, "carol", 1)));

	let (result, cycle) = h.cycle().await;

	result.unwrap();
	assert_eq!(h.handler.applied(), ["g-carol"]);
	assert_eq!(h.store.cursor(Stream::Scope).await.unwrap(), 12);
	assert!(cycle.progressed);
}

// --- a failing handler --------------------------------------------------------------------

#[tokio::test]
async fn should_neither_record_nor_pass_an_event_whose_handler_failed() {
	let h = Harness::new(UnknownKind::Park).await;
	h.concierge.scope(1, Row::Known(grants("g-carol", 1, "carol", 1)));
	h.handler.fail_on(Some("g-carol"));

	let (result, cycle) = h.cycle().await;

	let error = result.unwrap_err();
	assert!(matches!(&error, ProjectionError::Handler { event_id, .. } if event_id == "g-carol"), "{error:?}");
	assert!(!h.store.is_applied("g-carol").await.unwrap());
	assert_eq!(h.store.cursor(Stream::Scope).await.unwrap(), 0);
	assert!(!cycle.progressed);
}

#[tokio::test]
async fn should_apply_an_event_whose_handler_failed_on_the_next_cycle() {
	let h = Harness::new(UnknownKind::Park).await;
	h.concierge.scope(1, Row::Known(grants("g-carol", 1, "carol", 1)));
	h.handler.fail_on(Some("g-carol"));
	h.cycle().await.0.unwrap_err();
	h.handler.fail_on(None);

	let (result, _) = h.cycle().await;

	result.unwrap();
	assert_eq!(h.handler.applied(), ["g-carol"]);
	assert_eq!(h.store.cursor(Stream::Scope).await.unwrap(), 1);
}

// --- run(): pacing ------------------------------------------------------------------------

/// The bookkeeping in process memory. Only for `run()` on paused time: sqlx's SQLite answers
/// from a worker thread, and while the runtime waits on it the paused clock jumps to the
/// next timer. Held to the same contract as the real stores (`memory_store_contract`).
#[derive(Clone, Default)]
struct MemoryStore(Arc<Mutex<Book>>);

#[derive(Default)]
struct Book {
	cursors: BTreeMap<&'static str, i64>,
	applied: std::collections::BTreeSet<String>,
	sequences: BTreeMap<String, u64>,
	parked: BTreeMap<(&'static str, i64), Parked>,
}

fn check_fits(value: u64) -> Result<(), StoreError> {
	i64::try_from(value).map(|_| ()).map_err(|_| StoreError::OutOfRange { what: "sequence", value })
}

#[async_trait]
impl ProjectionStore for MemoryStore {
	async fn cursor(&self, stream: Stream) -> Result<i64, StoreError> {
		Ok(self.0.lock().unwrap().cursors.get(stream.as_str()).copied().unwrap_or(0))
	}

	async fn advance_cursor(&self, stream: Stream, to: i64) -> Result<(), StoreError> {
		let mut book = self.0.lock().unwrap();
		let cursor = book.cursors.entry(stream.as_str()).or_insert(to);
		*cursor = (*cursor).max(to);
		Ok(())
	}

	async fn is_applied(&self, event_id: &str) -> Result<bool, StoreError> {
		Ok(self.0.lock().unwrap().applied.contains(event_id))
	}

	async fn sequence(&self, key: &str) -> Result<Option<u64>, StoreError> {
		Ok(self.0.lock().unwrap().sequences.get(key).copied())
	}

	async fn record_applied(&self, _: Stream, event_id: &str, key: &str, sequence: u64) -> Result<(), StoreError> {
		check_fits(sequence)?;
		self.0.lock().unwrap().applied.insert(event_id.to_owned());
		self.raise_sequence(key, sequence).await
	}

	async fn raise_sequence(&self, key: &str, sequence: u64) -> Result<(), StoreError> {
		check_fits(sequence)?;
		let mut book = self.0.lock().unwrap();
		let stored = book.sequences.entry(key.to_owned()).or_insert(sequence);
		*stored = (*stored).max(sequence);
		Ok(())
	}

	async fn park(&self, parked: &Parked) -> Result<(), StoreError> {
		self.0.lock().unwrap().parked.entry((parked.stream.as_str(), parked.position)).or_insert_with(|| parked.clone());
		Ok(())
	}

	async fn has_parked(&self, key: &str) -> Result<bool, StoreError> {
		Ok(self.0.lock().unwrap().parked.values().any(|p| p.key == key))
	}

	async fn parked(&self, key: &str) -> Result<Vec<Parked>, StoreError> {
		let mut rows: Vec<Parked> = self.0.lock().unwrap().parked.values().filter(|p| p.key == key).cloned().collect();
		rows.sort_by_key(|p| p.position);
		Ok(rows)
	}

	async fn parked_keys(&self) -> Result<Vec<String>, StoreError> {
		let keys: std::collections::BTreeSet<String> = self.0.lock().unwrap().parked.values().map(|p| p.key.clone()).collect();
		Ok(keys.into_iter().collect())
	}

	async fn parked_count(&self) -> Result<u64, StoreError> {
		Ok(self.0.lock().unwrap().parked.len() as u64)
	}

	async fn unpark(&self, stream: Stream, position: i64) -> Result<(), StoreError> {
		self.0.lock().unwrap().parked.remove(&(stream.as_str(), position));
		Ok(())
	}
}

async fn fresh_memory_store() -> Option<(MemoryStore, ())> {
	Some((MemoryStore::default(), ()))
}

mod memory_store_contract {
	use super::{fresh_memory_store, no_teardown, projection_store};
	all_store_contract_tests!(fresh_memory_store, no_teardown);
}

fn paced(concierge: &FakeConcierge, handler: &Recorder) -> Projection<MemoryStore, Recorder, FakeConcierge> {
	let config = ProjectionConfig {
		unknown_kind: UnknownKind::Park,
		batch: 256,
		poll_interval: Duration::from_secs(5),
		backoff_max: Duration::from_secs(20),
	};
	Projection::new(MemoryStore::default(), handler.clone(), concierge.clone(), config)
}

#[tokio::test(start_paused = true)]
async fn run_should_double_the_wait_between_failing_cycles_up_to_the_ceiling() {
	let concierge = FakeConcierge::default();
	concierge.fail_pulls(usize::MAX);
	let projection = paced(&concierge, &Recorder::default());
	let start = Instant::now();

	tokio::select! {
		() = projection.run() => unreachable!("run returned"),
		() = tokio::time::sleep(Duration::from_secs(100)) => {}
	}

	// poll 5 s, then 10, 20, and 20 for ever: the ceiling is 20 s.
	assert_eq!(concierge.scope_pulls_since(start), [0, 5, 15, 35, 55, 75, 95]);
}

#[tokio::test(start_paused = true)]
async fn run_should_poll_at_the_interval_while_idle_and_after_recovering() {
	let concierge = FakeConcierge::default();
	concierge.fail_pulls(2);
	concierge.scope(1, Row::Known(grants("g-carol", 1, "carol", 1)));
	let handler = Recorder::default();
	let projection = paced(&concierge, &handler);
	let start = Instant::now();

	tokio::select! {
		() = projection.run() => unreachable!("run returned"),
		() = tokio::time::sleep(Duration::from_secs(31)) => {}
	}

	// two failures (waits 5, 10), then a cycle that progressed and idle ones: 5 s apart.
	assert_eq!(concierge.scope_pulls_since(start), [0, 5, 15, 20, 25, 30]);
	assert_eq!(handler.applied(), ["g-carol"]);
}

// --- the SQLite store ---------------------------------------------------------------------

mod sqlite_store_contract {
	use super::{fresh_sqlite_store, no_teardown, projection_store};
	all_store_contract_tests!(fresh_sqlite_store, no_teardown);
}

async fn schema_version(pool: &SqlitePool) -> i64 {
	sqlx::query_scalar("SELECT version FROM concierge_projection_schema").fetch_one(pool).await.unwrap()
}

#[tokio::test]
async fn sqlite_migrate_should_be_idempotent() {
	let pool = sqlite_pool().await;
	let store = SqliteProjectionStore::new(pool.clone());
	store.advance_cursor(Stream::Scope, 7).await.unwrap();

	sqlite::migrate(&pool).await.unwrap();
	sqlite::migrate(&pool).await.unwrap();

	assert_eq!(schema_version(&pool).await, 1);
	assert_eq!(store.cursor(Stream::Scope).await.unwrap(), 7);
}

#[tokio::test]
async fn sqlite_migrate_should_refuse_a_schema_newer_than_the_build() {
	let pool = sqlite_pool().await;
	sqlx::query("UPDATE concierge_projection_schema SET version = 2").execute(&pool).await.unwrap();

	let error = sqlite::migrate(&pool).await.unwrap_err();

	assert!(matches!(&error, StoreError::Corrupt(m) if m.contains("version 2")), "{error:?}");
}

#[tokio::test]
async fn sqlite_record_applied_should_leave_no_trace_when_raising_the_sequence_fails() {
	let pool = sqlite_pool().await;
	sqlx::query(
		"CREATE TRIGGER refuse_poisoned_key BEFORE INSERT ON concierge_projection_sequence WHEN NEW.key = 'user:poisoned'
		 BEGIN SELECT RAISE(ABORT, 'refused by the test'); END",
	)
	.execute(&pool)
	.await
	.unwrap();
	let store = SqliteProjectionStore::new(pool);

	let error = store.record_applied(Stream::UserLifecycle, "u-1", "user:poisoned", 1).await.unwrap_err();

	assert!(matches!(error, StoreError::Backend { .. }), "{error:?}");
	assert!(!store.is_applied("u-1").await.unwrap(), "the event id was recorded without its sequence");
}

#[tokio::test]
async fn sqlite_parked_should_report_a_row_with_an_unknown_reason_as_corrupt() {
	let pool = sqlite_pool().await;
	sqlx::query("INSERT INTO concierge_projection_parked (stream, position, event_id, key, reason) VALUES ('scope', 1, 'e-1', 'scope:sa', 'from_the_future')")
		.execute(&pool)
		.await
		.unwrap();

	let error = SqliteProjectionStore::new(pool).parked("scope:sa").await.unwrap_err();

	assert!(matches!(error, StoreError::Corrupt(_)), "{error:?}");
}

// --- GrpcSource: reading the wire ---------------------------------------------------------

use tonic_concierge::{
	Request, Response, Status,
	transport::{Server, server::TcpIncoming},
};
use wire::{
	PullScopeEventsRequest, PullScopeEventsResponse, PullUserLifecycleRequest, PullUserLifecycleResponse,
	scope_events_server::{ScopeEvents, ScopeEventsServer},
	user_events_server::{UserEvents, UserEventsServer},
};

/// A pull's `x-ev-client-id` and `authorization` headers.
type Credential = (Option<String>, Option<String>);

/// concierge's two pull RPCs, answering canned wire rows and keeping the credential each
/// pull presented.
#[derive(Clone, Default)]
struct WireConcierge {
	users: Arc<Mutex<Vec<wire::UserLifecycleEvent>>>,
	scopes: Arc<Mutex<Vec<wire::ScopeEvent>>>,
	refuse: bool,
	credentials: Arc<Mutex<Vec<Credential>>>,
}

impl WireConcierge {
	fn seen<T>(&self, request: &Request<T>) -> Result<(), Status> {
		let header = |name: &str| request.metadata().get(name).map(|v| v.to_str().unwrap().to_owned());
		self.credentials.lock().unwrap().push((header("x-ev-client-id"), header("authorization")));
		if self.refuse {
			return Err(Status::permission_denied("client disabled"));
		}
		Ok(())
	}
}

#[async_trait]
impl UserEvents for WireConcierge {
	async fn pull_user_lifecycle(&self, request: Request<PullUserLifecycleRequest>) -> Result<Response<PullUserLifecycleResponse>, Status> {
		self.seen(&request)?;
		Ok(Response::new(PullUserLifecycleResponse {
			events: self.users.lock().unwrap().clone(),
			next_position: 41,
		}))
	}
}

#[async_trait]
impl ScopeEvents for WireConcierge {
	async fn pull_scope_events(&self, request: Request<PullScopeEventsRequest>) -> Result<Response<PullScopeEventsResponse>, Status> {
		self.seen(&request)?;
		Ok(Response::new(PullScopeEventsResponse {
			events: self.scopes.lock().unwrap().clone(),
			next_position: 42,
		}))
	}
}

async fn grpc_source(concierge: WireConcierge) -> GrpcSource {
	let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
	let addr = listener.local_addr().unwrap();
	tokio::spawn(
		Server::builder()
			.add_service(UserEventsServer::new(concierge.clone()))
			.add_service(ScopeEventsServer::new(concierge))
			.serve_with_incoming(TcpIncoming::from(listener)),
	);
	GrpcSource::connect_lazy(&format!("http://{addr}"), "sa", "client-SECRET", Duration::from_secs(5)).unwrap()
}

fn wire_user_event(kind: i32, auth_subject: &str) -> wire::UserLifecycleEvent {
	wire::UserLifecycleEvent {
		event_id: "u-9".into(),
		user_id: "uid-ann".into(),
		kind,
		auth_subject: auth_subject.into(),
		sequence: 9,
		occurred_at: OCCURRED_AT,
		..Default::default()
	}
}

fn wire_scope_event(kind: wire::scope_event::Kind) -> wire::ScopeEvent {
	wire::ScopeEvent {
		event_id: "s-9".into(),
		position: 9,
		scope: SCOPE.into(),
		kind: kind.into(),
		occurred_at: OCCURRED_AT,
		sequence: 3,
		user_id: "uid-ann".into(),
		auth_subject: "ann".into(),
		..Default::default()
	}
}

#[tokio::test]
async fn grpc_source_should_present_the_client_credential_as_headers() {
	let concierge = WireConcierge::default();
	let source = grpc_source(concierge.clone()).await;

	let page = source.pull_scope_events(0, 10).await.unwrap();

	assert_eq!(page.next_position, 42);
	assert_eq!(
		concierge.credentials.lock().unwrap().as_slice(),
		[(Some("sa".to_owned()), Some("Bearer client-SECRET".to_owned()))]
	);
}

#[tokio::test]
async fn grpc_source_should_report_a_refused_credential_as_refused() {
	let source = grpc_source(WireConcierge {
		refuse: true,
		..WireConcierge::default()
	})
	.await;

	let error = source.pull_user_events(0, 10).await.unwrap_err();

	assert!(matches!(error, SourceError::Refused(tonic_concierge::Code::PermissionDenied)), "{error:?}");
}

#[tokio::test]
async fn grpc_source_should_read_a_lifecycle_kind_it_cannot_name_as_unknown_without_a_position() {
	let concierge = WireConcierge::default();
	concierge.users.lock().unwrap().push(wire_user_event(99, "ann"));
	let source = grpc_source(concierge).await;

	let page = source.pull_user_events(0, 10).await.unwrap();

	let [Pulled::Unknown(unknown)] = page.events.as_slice() else {
		panic!("expected one unknown event, got {:?}", page.events);
	};
	assert_eq!((unknown.event_id.as_str(), unknown.position, unknown.key.as_str(), unknown.kind), ("u-9", None, "user:ann", 99));
}

#[tokio::test]
async fn grpc_source_should_read_a_lifecycle_event_without_its_auth_subject_as_unknown() {
	let concierge = WireConcierge::default();
	concierge.users.lock().unwrap().push(wire_user_event(wire::user_lifecycle_event::Kind::Suspended.into(), ""));
	let source = grpc_source(concierge).await;

	let page = source.pull_user_events(0, 10).await.unwrap();

	assert!(matches!(page.events.as_slice(), [Pulled::Unknown(_)]), "{:?}", page.events);
}

#[tokio::test]
async fn grpc_source_should_read_a_known_lifecycle_kind_into_its_snapshot() {
	let concierge = WireConcierge::default();
	concierge.users.lock().unwrap().push(wire_user_event(wire::user_lifecycle_event::Kind::Suspended.into(), "ann"));
	let source = grpc_source(concierge).await;

	let page = source.pull_user_events(0, 10).await.unwrap();

	let [Pulled::Known(event)] = page.events.as_slice() else {
		panic!("expected one known event, got {:?}", page.events);
	};
	assert_eq!((event.kind, event.auth_subject.as_str(), event.sequence), (UserEventKind::Suspended, "ann", 9));
	assert_eq!(page.next_position, 41);
}

#[tokio::test]
async fn grpc_source_should_read_a_grants_changed_without_grants_as_unknown_keyed_by_scope_and_subject() {
	let concierge = WireConcierge::default();
	concierge.scopes.lock().unwrap().push(wire_scope_event(wire::scope_event::Kind::GrantsChanged));
	let source = grpc_source(concierge).await;

	let page = source.pull_scope_events(0, 10).await.unwrap();

	let [Pulled::Unknown(unknown)] = page.events.as_slice() else {
		panic!("expected one unknown event, got {:?}", page.events);
	};
	assert_eq!((unknown.event_id.as_str(), unknown.position, unknown.key.as_str()), ("s-9", Some(9), "scope:sa:user:ann"));
}

#[tokio::test]
async fn grpc_source_should_read_a_member_joined_into_its_member_snapshot() {
	let concierge = WireConcierge::default();
	concierge.scopes.lock().unwrap().push(wire::ScopeEvent {
		member: Some(wire::ScopeMember {
			email: "ann@example.com".into(),
			email_verified: true,
			preferred_name: "Ann".into(),
			status: "active".into(),
			kyc_level: 2,
			token_version: 4,
			lifecycle_sequence: 17,
			..Default::default()
		}),
		..wire_scope_event(wire::scope_event::Kind::MemberJoined)
	});
	let source = grpc_source(concierge).await;

	let page = source.pull_scope_events(0, 10).await.unwrap();

	let [Pulled::Known(event)] = page.events.as_slice() else {
		panic!("expected one known event, got {:?}", page.events);
	};
	assert_eq!(
		event.body,
		ScopeEventBody::MemberJoined {
			user_id: "uid-ann".into(),
			auth_subject: "ann".into(),
			member: Member {
				email: "ann@example.com".into(),
				email_verified: true,
				username: String::new(),
				preferred_name: "Ann".into(),
				status: "active".into(),
				kyc_level: 2,
				token_version: 4,
				lifecycle_sequence: 17,
			},
		}
	);
}

#[tokio::test]
async fn grpc_source_should_read_a_scope_state_it_cannot_name_as_unrecognized() {
	let concierge = WireConcierge::default();
	concierge.scopes.lock().unwrap().push(wire::ScopeEvent {
		scope_snapshot: Some(wire::Scope {
			id: SCOPE.into(),
			state: 7,
			..Default::default()
		}),
		auth_subject: String::new(),
		..wire_scope_event(wire::scope_event::Kind::ScopeChanged)
	});
	let source = grpc_source(concierge).await;

	let page = source.pull_scope_events(0, 10).await.unwrap();

	let [
		Pulled::Known(ScopeEvent {
			body: ScopeEventBody::ScopeChanged(scope),
			..
		}),
	] = page.events.as_slice()
	else {
		panic!("expected one SCOPE_CHANGED, got {:?}", page.events);
	};
	assert_eq!(scope.state, ScopeState::Unrecognized(7));
}
