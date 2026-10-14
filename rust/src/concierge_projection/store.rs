//! The engine's bookkeeping port: cursors, applied event ids, per-key sequences, parked
//! positions. Not the projection itself — that is the [`Handler`](super::Handler)'s, in the
//! consumer's own tables.

use async_trait::async_trait;

use super::event::Stream;

/// Why an event is parked rather than applied. Persisted by [`as_str`](Self::as_str).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ParkReason {
	/// A kind (or a shape) this build cannot read. Unblocked by an upgrade; replayed on the
	/// next start.
	UnknownKind,
	/// A lifecycle event for a subject whose MEMBER_JOINED has not been applied yet.
	/// Unblocked by that MEMBER_JOINED.
	AwaitingMember,
	/// Its key already has a parked event; it waits behind it so the per-key sequence guard
	/// never runs past an event still to apply.
	Behind,
}

impl ParkReason {
	pub fn as_str(self) -> &'static str {
		match self {
			Self::UnknownKind => "unknown_kind",
			Self::AwaitingMember => "awaiting_member",
			Self::Behind => "behind",
		}
	}

	pub fn parse(raw: &str) -> Option<Self> {
		match raw {
			"unknown_kind" => Some(Self::UnknownKind),
			"awaiting_member" => Some(Self::AwaitingMember),
			"behind" => Some(Self::Behind),
			_ => None,
		}
	}
}

/// A parked event: its POSITION, not its decode. concierge keeps every row at full fidelity
/// for ever, and a re-pull at `after = position - 1, limit = 1` is how it comes back.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Parked {
	pub stream: Stream,
	pub position: i64,
	pub event_id: String,
	pub key: String,
	pub reason: ParkReason,
}

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
	#[error("projection store: {context}: {source}")]
	Backend {
		context: &'static str,
		#[source]
		source: Box<dyn std::error::Error + Send + Sync>,
	},
	/// A `u64` sequence past what the store's signed 64-bit column holds.
	#[error("projection store: {what} {value} does not fit a signed 64-bit column")]
	OutOfRange { what: &'static str, value: u64 },
	/// A row this build cannot read back (a stream or reason name it does not know).
	#[error("projection store: unreadable row: {0}")]
	Corrupt(String),
}

/// Bookkeeping for the pull engine. Each method is its own unit of work; the engine orders
/// them so that a crash between two of them re-applies an event rather than losing one.
#[async_trait]
pub trait ProjectionStore: Send + Sync {
	/// The stream's cursor; 0 before the first advance.
	async fn cursor(&self, stream: Stream) -> Result<i64, StoreError>;
	/// Move the cursor forward to `to`. Never backwards: a lower `to` is a no-op.
	async fn advance_cursor(&self, stream: Stream, to: i64) -> Result<(), StoreError>;

	async fn is_applied(&self, event_id: &str) -> Result<bool, StoreError>;
	/// The highest sequence applied for `key`, if any.
	async fn sequence(&self, key: &str) -> Result<Option<u64>, StoreError>;
	/// Record `event_id` as done and raise `key`'s sequence to at least `sequence`,
	/// atomically. Idempotent.
	async fn record_applied(&self, stream: Stream, event_id: &str, key: &str, sequence: u64) -> Result<(), StoreError>;
	/// Raise `key`'s sequence to at least `sequence` without an event of its own — how a
	/// MEMBER_JOINED marks the lifecycle it already reflects.
	async fn raise_sequence(&self, key: &str, sequence: u64) -> Result<(), StoreError>;

	/// Idempotent on `(stream, position)`.
	async fn park(&self, parked: &Parked) -> Result<(), StoreError>;
	async fn has_parked(&self, key: &str) -> Result<bool, StoreError>;
	/// `key`'s parked events, oldest position first.
	async fn parked(&self, key: &str) -> Result<Vec<Parked>, StoreError>;
	/// Every key with a parked event.
	async fn parked_keys(&self) -> Result<Vec<String>, StoreError>;
	async fn parked_count(&self) -> Result<u64, StoreError>;
	async fn unpark(&self, stream: Stream, position: i64) -> Result<(), StoreError>;
}

#[cfg(any(feature = "concierge_projection_pg", feature = "concierge_projection_sqlite"))]
pub(crate) fn backend(context: &'static str) -> impl FnOnce(sqlx::Error) -> StoreError {
	move |e| StoreError::Backend { context, source: Box::new(e) }
}

#[cfg(any(feature = "concierge_projection_pg", feature = "concierge_projection_sqlite"))]
pub(crate) fn to_i64(what: &'static str, value: u64) -> Result<i64, StoreError> {
	i64::try_from(value).map_err(|_| StoreError::OutOfRange { what, value })
}

#[cfg(any(feature = "concierge_projection_pg", feature = "concierge_projection_sqlite"))]
pub(crate) fn to_u64(what: &'static str, value: i64) -> Result<u64, StoreError> {
	u64::try_from(value).map_err(|_| StoreError::Corrupt(format!("{what} {value} is negative")))
}

#[cfg(any(feature = "concierge_projection_pg", feature = "concierge_projection_sqlite"))]
pub(crate) fn parked_row(stream: &str, position: i64, event_id: String, key: String, reason: &str) -> Result<Parked, StoreError> {
	Ok(Parked {
		stream: Stream::parse(stream).ok_or_else(|| StoreError::Corrupt(format!("unknown stream {stream:?}")))?,
		position,
		event_id,
		key,
		reason: ParkReason::parse(reason).ok_or_else(|| StoreError::Corrupt(format!("unknown park reason {reason:?}")))?,
	})
}
