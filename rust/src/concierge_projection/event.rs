//! The events a [`Handler`](super::Handler) is given: this library's own types, not the
//! wire's, so a contract bump in concierge is a change in [`source`](super::source) and not
//! in every projection.

/// The two concierge streams a projection reads. Each has its own cursor.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Stream {
	/// `UserEvents.PullUserLifecycle`.
	UserLifecycle,
	/// `ScopeEvents.PullScopeEvents`.
	Scope,
}

impl Stream {
	/// The name a store keys this stream's rows by. Stable: it is persisted.
	pub fn as_str(self) -> &'static str {
		match self {
			Self::UserLifecycle => "user_lifecycle",
			Self::Scope => "scope",
		}
	}

	pub fn parse(raw: &str) -> Option<Self> {
		match raw {
			"user_lifecycle" => Some(Self::UserLifecycle),
			"scope" => Some(Self::Scope),
			_ => None,
		}
	}
}

/// A lifecycle row as a relying party receives it: redacted by concierge (no seat
/// permissions), keyed by `auth_subject`. Every field but `kind` is a snapshot of the user
/// after the change, on every kind.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserEvent {
	pub event_id: String,
	pub kind: UserEventKind,
	/// concierge's own user id. Correlate on [`auth_subject`](Self::auth_subject) instead.
	pub user_id: String,
	/// Opaque and immutable; the one user key every plane shares. Never parse it.
	pub auth_subject: String,
	/// Per-user, strictly increasing. The engine applies an event only above what it has
	/// stored for the user.
	pub sequence: u64,
	/// Unix seconds.
	pub occurred_at: i64,
	pub kyc_level: u32,
	pub email: String,
	pub email_verified: bool,
	pub token_version: u64,
	/// `investor` / `operator` / `admin` / `owner` — the platform role, not the direction's.
	pub role: String,
}

/// The lifecycle kinds this build can name. A kind it cannot is never one of these: it
/// takes the [`UnknownKind`](super::UnknownKind) policy instead.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum UserEventKind {
	Created,
	Suspended,
	Reinstated,
	KycChanged,
	SessionsRevoked,
	RoleChanged,
	/// Money-plane seat permissions moved. A relying party's row carries none of them, so
	/// for a direction this is a snapshot refresh and nothing more.
	PermissionsChanged,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScopeEvent {
	pub event_id: String,
	pub position: i64,
	/// The scope the event is about.
	pub scope: String,
	/// Unix seconds.
	pub occurred_at: i64,
	/// Per scope for [`ScopeEventBody::ScopeChanged`] (the scope's `version`); per (scope,
	/// user) for the other two.
	pub sequence: u64,
	pub body: ScopeEventBody,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ScopeEventBody {
	/// The whole scope after the change, `state` included.
	ScopeChanged(Scope),
	/// The user's whole concrete permission set in the scope's namespace after the change.
	/// Empty means "holds nothing here", and must be applied as such.
	GrantsChanged { user_id: String, auth_subject: String, permissions: Vec<String> },
	/// The user became visible to the scope. Seed the projection's row from `member`;
	/// lifecycle events at or below `member.lifecycle_sequence` are already in it, and the
	/// engine will not hand them over.
	MemberJoined { user_id: String, auth_subject: String, member: Member },
}

/// What a direction may hold about a person. Never identity-document fields.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Member {
	pub email: String,
	pub email_verified: bool,
	pub username: String,
	pub preferred_name: String,
	pub status: String,
	pub kyc_level: u32,
	pub token_version: u64,
	/// The lifecycle `sequence` this snapshot is as of.
	pub lifecycle_sequence: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Scope {
	pub id: String,
	/// Empty for `main`.
	pub namespace: String,
	/// Empty for `main`.
	pub parent: String,
	pub display_name: String,
	pub origins: Vec<String>,
	pub return_paths: Vec<String>,
	pub clients: Vec<ScopeClient>,
	/// Client ids that may also read this scope's streams. Read access only.
	pub observers: Vec<String>,
	/// Unix seconds.
	pub created_at: i64,
	pub version: u64,
	pub state: ScopeState,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScopeClient {
	pub client_id: String,
	pub audience: String,
	pub disabled: bool,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScopeState {
	Active,
	/// Reversible stop: nothing new opens in it; sign-in and the streams go on.
	Frozen,
	/// Terminal.
	Closed,
	/// A state newer than this build. Treat it as not [`Active`](Self::Active): the safe
	/// reading of a state one cannot name.
	Unrecognized(i32),
}

/// The ordering key of an event: what its `sequence` counts within, and what parking
/// chains on. Persisted, so the format is fixed.
pub(crate) fn user_key(auth_subject: &str) -> String {
	format!("user:{auth_subject}")
}

pub(crate) fn scope_key(scope: &str) -> String {
	format!("scope:{scope}")
}

pub(crate) fn scope_user_key(scope: &str, auth_subject: &str) -> String {
	format!("scope:{scope}:user:{auth_subject}")
}

impl UserEvent {
	pub(crate) fn key(&self) -> String {
		user_key(&self.auth_subject)
	}
}

impl ScopeEvent {
	pub(crate) fn key(&self) -> String {
		match &self.body {
			ScopeEventBody::ScopeChanged(_) => scope_key(&self.scope),
			ScopeEventBody::GrantsChanged { auth_subject, .. } | ScopeEventBody::MemberJoined { auth_subject, .. } => scope_user_key(&self.scope, auth_subject),
		}
	}
}
