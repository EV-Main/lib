use async_trait::async_trait;

use super::event::{ScopeEvent, UserEvent};

/// The projection itself: what a direction does with each event, in its own tables.
///
/// Called at least once per event. The engine has already dropped duplicates and anything
/// at or below the key's stored sequence, but a crash between the handler returning and
/// the engine recording it hands the same event over again — so write snapshots (upsert
/// the row to what the event says), never deltas.
///
/// An `Err` stops the cycle before the event is recorded; the engine retries it from the
/// same cursor after a backoff. Return one only for what a retry can fix (the database is
/// down); an event the projection has no use for is `Ok(())`.
#[async_trait]
pub trait Handler: Send + Sync {
	type Error: std::error::Error + Send + Sync + 'static;

	async fn on_user_event(&self, event: &UserEvent) -> Result<(), Self::Error>;
	async fn on_scope_event(&self, event: &ScopeEvent) -> Result<(), Self::Error>;
}
