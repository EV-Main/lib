//! Where events come from: the [`EventSource`] port, and [`GrpcSource`], its adapter over
//! concierge's two pull RPCs. The conversion from the wire lives here and nowhere else.

use std::time::Duration;

use async_trait::async_trait;
use evconcierge_contracts::concierge::v1 as proto;
use proto::{
	PullScopeEventsRequest, PullUserLifecycleRequest, scope_event::Kind as WireScopeKind, scope_events_client::ScopeEventsClient, user_events_client::UserEventsClient,
	user_lifecycle_event::Kind as WireUserKind,
};
use tonic_concierge::{
	Code, Request, Status,
	metadata::{AsciiMetadataValue, MetadataValue},
	transport::{Channel, Endpoint},
};

use super::event::{Member, Scope, ScopeClient, ScopeEvent, ScopeEventBody, ScopeState, UserEvent, UserEventKind, scope_key, scope_user_key, user_key};

/// One pull's answer.
#[derive(Clone, Debug)]
pub struct Page<T> {
	/// Oldest first.
	pub events: Vec<T>,
	/// The cursor to pass as the next `after`: the maximum position concierge scanned for
	/// this caller, which may lie past the last event returned.
	pub next_position: i64,
}

/// An event as pulled: one this build can read, or one it cannot.
#[derive(Clone, Debug)]
pub enum Pulled<T> {
	Known(T),
	Unknown(UnknownEvent),
}

/// A row this build cannot read: a kind it cannot name (prost decodes it as unspecified),
/// or a known kind missing what it must carry. Only what is needed to park it by position
/// is kept — never the decode, which already lost what the new kind carried.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnknownEvent {
	pub event_id: String,
	/// `None` on the lifecycle stream, whose rows do not carry their position.
	pub position: Option<i64>,
	/// The ordering key it would have had, so later events of the same key queue behind it.
	pub key: String,
	/// The raw wire kind.
	pub kind: i32,
	pub why: &'static str,
}

#[derive(Debug, thiserror::Error)]
pub enum SourceError {
	/// The client credential was refused, or the client is disabled. Nothing will change
	/// until someone fixes the configuration.
	#[error("concierge refused the stream credential: {0:?}")]
	Refused(Code),
	/// Not reachable or too slow; the engine retries with backoff.
	#[error("concierge is unavailable: {0}")]
	Unavailable(String),
	#[error("misconfigured event source: {0}")]
	Config(String),
	#[error("concierge failed ({code:?}): {message}")]
	Failed { code: Code, message: String },
}

impl From<Status> for SourceError {
	fn from(status: Status) -> Self {
		let code = status.code();
		match code {
			Code::Unauthenticated | Code::PermissionDenied => Self::Refused(code),
			Code::Unavailable | Code::DeadlineExceeded | Code::Cancelled => Self::Unavailable(status.message().to_owned()),
			Code::Ok
			| Code::Unknown
			| Code::InvalidArgument
			| Code::NotFound
			| Code::AlreadyExists
			| Code::ResourceExhausted
			| Code::FailedPrecondition
			| Code::Aborted
			| Code::OutOfRange
			| Code::Unimplemented
			| Code::Internal
			| Code::DataLoss => Self::Failed {
				code,
				message: status.message().to_owned(),
			},
		}
	}
}

/// The two pull streams. `after` is exclusive; the server caps `limit` at its own ceiling.
/// A fake of this trait is how the engine is tested without concierge.
#[async_trait]
pub trait EventSource: Send + Sync {
	async fn pull_user_events(&self, after: i64, limit: u32) -> Result<Page<Pulled<UserEvent>>, SourceError>;
	async fn pull_scope_events(&self, after: i64, limit: u32) -> Result<Page<Pulled<ScopeEvent>>, SourceError>;
}

/// [`EventSource`] over concierge's gRPC, authenticated with the relying party's client
/// credential (`x-ev-client-id` + `authorization: Bearer <secret>`). concierge filters both
/// streams by what that client may see and redacts each lifecycle row for it.
#[derive(Clone)]
pub struct GrpcSource {
	users: UserEventsClient<Channel>,
	scopes: ScopeEventsClient<Channel>,
	client_id: AsciiMetadataValue,
	bearer: AsciiMetadataValue,
}

impl std::fmt::Debug for GrpcSource {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("GrpcSource").field("client_id", &self.client_id).finish_non_exhaustive()
	}
}

impl GrpcSource {
	/// Connects on first pull, so the service boots while concierge is down.
	pub fn connect_lazy(endpoint: &str, client_id: &str, client_secret: &str, timeout: Duration) -> Result<Self, SourceError> {
		let channel = Endpoint::from_shared(endpoint.to_owned())
			.map_err(|e| SourceError::Config(format!("concierge endpoint {endpoint:?}: {e}")))?
			.connect_timeout(timeout)
			.timeout(timeout)
			.connect_lazy();
		Self::from_channel(channel, client_id, client_secret)
	}

	pub fn from_channel(channel: Channel, client_id: &str, client_secret: &str) -> Result<Self, SourceError> {
		Ok(Self {
			users: UserEventsClient::new(channel.clone()),
			scopes: ScopeEventsClient::new(channel),
			client_id: MetadataValue::try_from(client_id).map_err(|_| SourceError::Config("the client id is not a valid header value".into()))?,
			bearer: MetadataValue::try_from(format!("Bearer {client_secret}")).map_err(|_| SourceError::Config("the client secret is not a valid header value".into()))?,
		})
	}

	fn authorized<T>(&self, message: T) -> Request<T> {
		let mut request = Request::new(message);
		let metadata = request.metadata_mut();
		metadata.insert("x-ev-client-id", self.client_id.clone());
		metadata.insert("authorization", self.bearer.clone());
		request
	}
}

#[async_trait]
impl EventSource for GrpcSource {
	async fn pull_user_events(&self, after: i64, limit: u32) -> Result<Page<Pulled<UserEvent>>, SourceError> {
		let request = self.authorized(PullUserLifecycleRequest { after_position: after, limit });
		let answer = self.users.clone().pull_user_lifecycle(request).await?.into_inner();
		Ok(Page {
			events: answer.events.into_iter().map(user_event).collect(),
			next_position: answer.next_position,
		})
	}

	async fn pull_scope_events(&self, after: i64, limit: u32) -> Result<Page<Pulled<ScopeEvent>>, SourceError> {
		let request = self.authorized(PullScopeEventsRequest { after_position: after, limit });
		let answer = self.scopes.clone().pull_scope_events(request).await?.into_inner();
		Ok(Page {
			events: answer.events.into_iter().map(scope_event).collect(),
			next_position: answer.next_position,
		})
	}
}

fn user_event(e: proto::UserLifecycleEvent) -> Pulled<UserEvent> {
	let kind = match WireUserKind::try_from(e.kind) {
		Ok(WireUserKind::Created) => UserEventKind::Created,
		Ok(WireUserKind::Suspended) => UserEventKind::Suspended,
		Ok(WireUserKind::Reinstated) => UserEventKind::Reinstated,
		Ok(WireUserKind::KycChanged) => UserEventKind::KycChanged,
		Ok(WireUserKind::SessionsRevoked) => UserEventKind::SessionsRevoked,
		Ok(WireUserKind::RoleChanged) => UserEventKind::RoleChanged,
		Ok(WireUserKind::PermissionsChanged) => UserEventKind::PermissionsChanged,
		Ok(WireUserKind::Unspecified) | Err(_) => {
			return Pulled::Unknown(UnknownEvent {
				key: user_key(&e.auth_subject),
				event_id: e.event_id,
				position: None,
				kind: e.kind,
				why: "a lifecycle kind this build cannot name",
			});
		}
	};
	if e.auth_subject.is_empty() {
		return Pulled::Unknown(UnknownEvent {
			key: user_key(&e.auth_subject),
			event_id: e.event_id,
			position: None,
			kind: e.kind,
			why: "a lifecycle event without its auth_subject",
		});
	}
	Pulled::Known(UserEvent {
		event_id: e.event_id,
		kind,
		user_id: e.user_id,
		auth_subject: e.auth_subject,
		sequence: e.sequence,
		occurred_at: e.occurred_at,
		kyc_level: e.kyc_level,
		email: e.email,
		email_verified: e.email_verified,
		token_version: e.token_version,
		role: e.role,
	})
}

fn scope_event(e: proto::ScopeEvent) -> Pulled<ScopeEvent> {
	let unknown = |e: &proto::ScopeEvent, why: &'static str| {
		Pulled::Unknown(UnknownEvent {
			event_id: e.event_id.clone(),
			position: Some(e.position),
			key: if e.auth_subject.is_empty() {
				scope_key(&e.scope)
			} else {
				scope_user_key(&e.scope, &e.auth_subject)
			},
			kind: e.kind,
			why,
		})
	};
	let body = match WireScopeKind::try_from(e.kind) {
		Ok(WireScopeKind::ScopeChanged) => match &e.scope_snapshot {
			Some(snapshot) => ScopeEventBody::ScopeChanged(scope(snapshot.clone())),
			None => return unknown(&e, "SCOPE_CHANGED without its scope snapshot"),
		},
		Ok(WireScopeKind::GrantsChanged) => match &e.grants {
			Some(grants) => ScopeEventBody::GrantsChanged {
				user_id: e.user_id.clone(),
				auth_subject: e.auth_subject.clone(),
				permissions: grants.permissions.clone(),
			},
			None => return unknown(&e, "GRANTS_CHANGED without its grants"),
		},
		Ok(WireScopeKind::MemberJoined) => match &e.member {
			Some(m) => ScopeEventBody::MemberJoined {
				user_id: e.user_id.clone(),
				auth_subject: e.auth_subject.clone(),
				member: Member {
					email: m.email.clone(),
					email_verified: m.email_verified,
					username: m.username.clone(),
					preferred_name: m.preferred_name.clone(),
					status: m.status.clone(),
					kyc_level: m.kyc_level,
					token_version: m.token_version,
					lifecycle_sequence: m.lifecycle_sequence,
				},
			},
			None => return unknown(&e, "MEMBER_JOINED without its member snapshot"),
		},
		Ok(WireScopeKind::Unspecified) | Err(_) => return unknown(&e, "a scope event kind this build cannot name"),
	};
	if !matches!(body, ScopeEventBody::ScopeChanged(_)) && e.auth_subject.is_empty() {
		return unknown(&e, "a user event without its auth_subject");
	}
	Pulled::Known(ScopeEvent {
		event_id: e.event_id,
		position: e.position,
		scope: e.scope,
		occurred_at: e.occurred_at,
		sequence: e.sequence,
		body,
	})
}

fn scope(s: proto::Scope) -> Scope {
	let state = match proto::ScopeState::try_from(s.state) {
		Ok(proto::ScopeState::Active) => ScopeState::Active,
		Ok(proto::ScopeState::Frozen) => ScopeState::Frozen,
		Ok(proto::ScopeState::Closed) => ScopeState::Closed,
		Ok(proto::ScopeState::Unspecified) | Err(_) => ScopeState::Unrecognized(s.state),
	};
	Scope {
		id: s.id,
		namespace: s.namespace,
		parent: s.parent,
		display_name: s.display_name,
		origins: s.origins,
		return_paths: s.return_paths,
		clients: s
			.clients
			.into_iter()
			.map(|c| ScopeClient {
				client_id: c.client_id,
				audience: c.audience,
				disabled: c.disabled,
			})
			.collect(),
		observers: s.observers,
		created_at: s.created_at,
		version: s.version,
		state,
	}
}
