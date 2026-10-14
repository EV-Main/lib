// The `ProjectionStore` contract, run against both adapters on a real database: every
// runtime query in `pg.rs` and `sqlite.rs` is executed by one of these. `#[path]`-included
// by tests/concierge_projection.rs (SQLite) and tests/concierge_projection_pg.rs; each
// wraps the cases in `store_contract_tests!` with its own fixture.

use ev_lib::concierge_projection::{ParkReason, Parked, ProjectionStore, StoreError, Stream};

fn parked(stream: Stream, position: i64, event_id: &str, key: &str, reason: ParkReason) -> Parked {
	Parked {
		stream,
		position,
		event_id: event_id.to_owned(),
		key: key.to_owned(),
		reason,
	}
}

pub async fn cursor_should_start_at_zero(store: &impl ProjectionStore) {
	assert_eq!(store.cursor(Stream::Scope).await.unwrap(), 0);
	assert_eq!(store.cursor(Stream::UserLifecycle).await.unwrap(), 0);
}

pub async fn advance_cursor_should_never_move_backwards(store: &impl ProjectionStore) {
	store.advance_cursor(Stream::Scope, 10).await.unwrap();
	store.advance_cursor(Stream::Scope, 4).await.unwrap();
	assert_eq!(store.cursor(Stream::Scope).await.unwrap(), 10, "a lower `to` moved the cursor back");

	store.advance_cursor(Stream::Scope, 12).await.unwrap();
	assert_eq!(store.cursor(Stream::Scope).await.unwrap(), 12);
}

pub async fn cursors_of_the_two_streams_should_be_independent(store: &impl ProjectionStore) {
	store.advance_cursor(Stream::Scope, 7).await.unwrap();

	assert_eq!(store.cursor(Stream::UserLifecycle).await.unwrap(), 0);
}

pub async fn record_applied_should_mark_the_event_and_raise_the_key_sequence(store: &impl ProjectionStore) {
	store.record_applied(Stream::UserLifecycle, "e-1", "user:ann", 3).await.unwrap();

	assert!(store.is_applied("e-1").await.unwrap());
	assert!(!store.is_applied("e-2").await.unwrap());
	assert_eq!(store.sequence("user:ann").await.unwrap(), Some(3));
	assert_eq!(store.sequence("user:bob").await.unwrap(), None);
}

pub async fn record_applied_should_be_idempotent(store: &impl ProjectionStore) {
	store.record_applied(Stream::Scope, "e-1", "scope:sa", 3).await.unwrap();
	store.record_applied(Stream::Scope, "e-1", "scope:sa", 3).await.unwrap();

	assert!(store.is_applied("e-1").await.unwrap());
	assert_eq!(store.sequence("scope:sa").await.unwrap(), Some(3));
}

pub async fn record_applied_should_not_lower_the_key_sequence(store: &impl ProjectionStore) {
	store.raise_sequence("user:ann", 9).await.unwrap();

	store.record_applied(Stream::UserLifecycle, "e-1", "user:ann", 4).await.unwrap();

	assert_eq!(store.sequence("user:ann").await.unwrap(), Some(9));
}

pub async fn raise_sequence_should_keep_the_highest(store: &impl ProjectionStore) {
	store.raise_sequence("user:ann", 5).await.unwrap();
	store.raise_sequence("user:ann", 3).await.unwrap();
	assert_eq!(store.sequence("user:ann").await.unwrap(), Some(5), "a lower raise lowered the sequence");

	store.raise_sequence("user:ann", 8).await.unwrap();
	assert_eq!(store.sequence("user:ann").await.unwrap(), Some(8));
}

pub async fn a_sequence_past_i64_should_be_out_of_range(store: &impl ProjectionStore) {
	let recorded = store.record_applied(Stream::UserLifecycle, "e-1", "user:ann", u64::MAX).await.unwrap_err();
	let raised = store.raise_sequence("user:ann", 1 << 63).await.unwrap_err();

	assert!(matches!(recorded, StoreError::OutOfRange { value: u64::MAX, .. }), "{recorded:?}");
	assert!(matches!(raised, StoreError::OutOfRange { .. }), "{raised:?}");
	assert!(!store.is_applied("e-1").await.unwrap());
}

pub async fn park_should_be_idempotent_on_stream_and_position(store: &impl ProjectionStore) {
	store.park(&parked(Stream::Scope, 5, "e-5", "scope:sa:user:ann", ParkReason::UnknownKind)).await.unwrap();
	store.park(&parked(Stream::Scope, 5, "e-5", "scope:sa:user:ann", ParkReason::UnknownKind)).await.unwrap();
	assert_eq!(store.parked_count().await.unwrap(), 1);

	store.park(&parked(Stream::UserLifecycle, 5, "u-5", "user:ann", ParkReason::AwaitingMember)).await.unwrap();
	assert_eq!(store.parked_count().await.unwrap(), 2);
}

pub async fn parked_should_list_a_key_oldest_position_first(store: &impl ProjectionStore) {
	store.park(&parked(Stream::Scope, 9, "e-9", "scope:sa:user:ann", ParkReason::Behind)).await.unwrap();
	store.park(&parked(Stream::Scope, 3, "e-3", "scope:sa:user:ann", ParkReason::UnknownKind)).await.unwrap();
	store.park(&parked(Stream::Scope, 5, "e-5", "scope:sa:user:bob", ParkReason::UnknownKind)).await.unwrap();

	assert_eq!(
		store.parked("scope:sa:user:ann").await.unwrap(),
		vec![
			parked(Stream::Scope, 3, "e-3", "scope:sa:user:ann", ParkReason::UnknownKind),
			parked(Stream::Scope, 9, "e-9", "scope:sa:user:ann", ParkReason::Behind),
		]
	);
}

pub async fn parked_should_read_back_every_stream_and_reason(store: &impl ProjectionStore) {
	store.park(&parked(Stream::UserLifecycle, 1, "u-1", "user:ann", ParkReason::AwaitingMember)).await.unwrap();
	store.park(&parked(Stream::UserLifecycle, 2, "u-2", "user:ann", ParkReason::Behind)).await.unwrap();
	store.park(&parked(Stream::UserLifecycle, 3, "u-3", "user:ann", ParkReason::UnknownKind)).await.unwrap();

	assert_eq!(
		store.parked("user:ann").await.unwrap(),
		vec![
			parked(Stream::UserLifecycle, 1, "u-1", "user:ann", ParkReason::AwaitingMember),
			parked(Stream::UserLifecycle, 2, "u-2", "user:ann", ParkReason::Behind),
			parked(Stream::UserLifecycle, 3, "u-3", "user:ann", ParkReason::UnknownKind),
		]
	);
}

pub async fn parked_keys_should_list_each_key_once(store: &impl ProjectionStore) {
	store.park(&parked(Stream::Scope, 1, "e-1", "scope:sa:user:bob", ParkReason::UnknownKind)).await.unwrap();
	store.park(&parked(Stream::Scope, 2, "e-2", "scope:sa:user:bob", ParkReason::Behind)).await.unwrap();
	store.park(&parked(Stream::UserLifecycle, 1, "u-1", "user:ann", ParkReason::AwaitingMember)).await.unwrap();

	assert_eq!(store.parked_keys().await.unwrap(), vec!["scope:sa:user:bob".to_owned(), "user:ann".to_owned()]);
	assert!(store.has_parked("user:ann").await.unwrap());
	assert!(!store.has_parked("user:carol").await.unwrap());
}

pub async fn unpark_should_remove_only_that_stream_and_position(store: &impl ProjectionStore) {
	store.park(&parked(Stream::Scope, 1, "e-1", "scope:sa:user:bob", ParkReason::UnknownKind)).await.unwrap();
	store.park(&parked(Stream::Scope, 2, "e-2", "scope:sa:user:bob", ParkReason::Behind)).await.unwrap();
	store.park(&parked(Stream::UserLifecycle, 1, "u-1", "user:ann", ParkReason::AwaitingMember)).await.unwrap();

	store.unpark(Stream::Scope, 1).await.unwrap();

	assert_eq!(
		store.parked("scope:sa:user:bob").await.unwrap(),
		vec![parked(Stream::Scope, 2, "e-2", "scope:sa:user:bob", ParkReason::Behind)]
	);
	assert!(store.has_parked("user:ann").await.unwrap());
	assert_eq!(store.parked_count().await.unwrap(), 2);
}

/// One `#[tokio::test]` per contract case. `$fresh` is an async fn returning
/// `Option<(store, fixture)>` (`None`: the backend is not configured, the test says SKIPPED
/// and returns); `$finish` takes the fixture back and tears it down.
macro_rules! store_contract_tests {
	($fresh:path, $finish:path; $($case:ident),+ $(,)?) => {
		$(
			#[tokio::test]
			async fn $case() {
				let Some((store, fixture)) = $fresh().await else { return };
				projection_store::$case(&store).await;
				$finish(fixture).await;
			}
		)+
	};
}

macro_rules! all_store_contract_tests {
	($fresh:path, $finish:path) => {
		store_contract_tests!(
			$fresh, $finish;
			cursor_should_start_at_zero,
			advance_cursor_should_never_move_backwards,
			cursors_of_the_two_streams_should_be_independent,
			record_applied_should_mark_the_event_and_raise_the_key_sequence,
			record_applied_should_be_idempotent,
			record_applied_should_not_lower_the_key_sequence,
			raise_sequence_should_keep_the_highest,
			a_sequence_past_i64_should_be_out_of_range,
			park_should_be_idempotent_on_stream_and_position,
			parked_should_list_a_key_oldest_position_first,
			parked_should_read_back_every_stream_and_reason,
			parked_keys_should_list_each_key_once,
			unpark_should_remove_only_that_stream_and_position,
		);
	};
}
