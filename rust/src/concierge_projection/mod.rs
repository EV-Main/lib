//! `concierge_projection` — the engine a direction (invest, admin, the Service-Arb panel)
//! keeps its own copy of concierge's users with.
//!
//! concierge is the identity plane and never calls a direction; a direction pulls. Two
//! streams, each with its own cursor:
//!
//! - `UserEvents.PullUserLifecycle` — suspensions, KYC level, role, token revocations,
//!   filtered to the users this client may see and redacted for it;
//! - `ScopeEvents.PullScopeEvents` — the scopes themselves, grants inside them, and
//!   MEMBER_JOINED, which seeds a user's row.
//!
//! The engine owns the bookkeeping — cursor, dedupe by `event_id`, per-key ordering by
//! `sequence`, parking — behind the [`ProjectionStore`] port; the [`Handler`] owns the
//! projection. A lifecycle event that arrives before its user's MEMBER_JOINED is parked by
//! `auth_subject` and replayed when it lands; an event this build cannot read takes the
//! [`UnknownKind`] policy. What to keep, what never to copy, and how to rebuild: GUIDE.md.
//!
//! ```ignore
//! pg::migrate(&pool).await?;
//! let projection = Projection::new(
//!     PgProjectionStore::new(pool.clone()),
//!     MyUsers::new(pool),
//!     GrpcSource::connect_lazy(&concierge_grpc, "invest", &client_secret, Duration::from_secs(10))?,
//!     ProjectionConfig::default(),
//! );
//! tokio::select! { () = projection.run() => {}, () = shutdown.cancelled() => {} }
//! ```

mod engine;
mod event;
mod handler;
mod source;
mod store;

#[cfg(feature = "concierge_projection_pg")]
pub mod pg;
#[cfg(feature = "concierge_projection_sqlite")]
pub mod sqlite;

pub use engine::{Cycle, Projection, ProjectionConfig, ProjectionError, UnknownKind};
pub use event::{Member, Scope, ScopeClient, ScopeEvent, ScopeEventBody, ScopeState, Stream, UserEvent, UserEventKind};
pub use handler::Handler;
#[cfg(feature = "concierge_projection_pg")]
pub use pg::PgProjectionStore;
pub use source::{EventSource, GrpcSource, Page, Pulled, SourceError, UnknownEvent};
#[cfg(feature = "concierge_projection_sqlite")]
pub use sqlite::SqliteProjectionStore;
pub use store::{ParkReason, Parked, ProjectionStore, StoreError};
