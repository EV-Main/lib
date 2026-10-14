//! [`ProjectionStore`] on Postgres. Runtime queries only, so the consumer's build never
//! needs a live database or a `.sqlx` cache for this library.

use async_trait::async_trait;
use sqlx::{PgPool, Row};

use super::{
	event::Stream,
	store::{Parked, ProjectionStore, StoreError, backend, parked_row, to_i64, to_u64},
};

/// The bookkeeping tables, one step per schema version. Applied in order by [`migrate`];
/// a shipped step is never edited, only followed by a new one.
const STEPS: &[&[&str]] = &[&[
	"CREATE TABLE IF NOT EXISTS concierge_projection_cursor (
		stream TEXT PRIMARY KEY,
		position BIGINT NOT NULL,
		updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
	)",
	"CREATE TABLE IF NOT EXISTS concierge_projection_applied (
		event_id TEXT PRIMARY KEY,
		stream TEXT NOT NULL,
		applied_at TIMESTAMPTZ NOT NULL DEFAULT now()
	)",
	"CREATE TABLE IF NOT EXISTS concierge_projection_sequence (
		key TEXT PRIMARY KEY,
		sequence BIGINT NOT NULL
	)",
	"CREATE TABLE IF NOT EXISTS concierge_projection_parked (
		stream TEXT NOT NULL,
		position BIGINT NOT NULL,
		event_id TEXT NOT NULL,
		key TEXT NOT NULL,
		reason TEXT NOT NULL,
		parked_at TIMESTAMPTZ NOT NULL DEFAULT now(),
		PRIMARY KEY (stream, position)
	)",
	// A plain (not CONCURRENTLY) index is safe only because the table was created empty in
	// this same step.
	"CREATE INDEX IF NOT EXISTS concierge_projection_parked_key ON concierge_projection_parked (key, position)",
]];

/// Create or bring up to date the engine's tables, `concierge_projection_*`, in the pool's
/// schema. Idempotent, and safe for replicas booting together: the steps run under a
/// transaction-scoped advisory lock. Kept apart from the consumer's own `sqlx::migrate!`
/// history on purpose — its version table is `concierge_projection_schema`, so neither
/// side's numbering can collide with the other's.
pub async fn migrate(pool: &PgPool) -> Result<(), StoreError> {
	let mut tx = pool.begin().await.map_err(backend("begin migration"))?;
	// The tables are new and nobody else writes them, so a wait here is another replica
	// migrating; anything longer than this is worth failing the boot over.
	sqlx::query("SET LOCAL lock_timeout = '10s'").execute(&mut *tx).await.map_err(backend("set lock_timeout"))?;
	sqlx::query("SELECT pg_advisory_xact_lock(hashtext('ev_lib.concierge_projection'))")
		.execute(&mut *tx)
		.await
		.map_err(backend("take the migration lock"))?;
	sqlx::query("CREATE TABLE IF NOT EXISTS concierge_projection_schema (id BOOLEAN PRIMARY KEY DEFAULT TRUE CHECK (id), version INTEGER NOT NULL)")
		.execute(&mut *tx)
		.await
		.map_err(backend("create the schema version table"))?;
	let current: Option<i32> = sqlx::query_scalar("SELECT version FROM concierge_projection_schema WHERE id")
		.fetch_optional(&mut *tx)
		.await
		.map_err(backend("read the schema version"))?;
	let current = usize::try_from(current.unwrap_or(0)).map_err(|_| StoreError::Corrupt("negative concierge_projection_schema.version".into()))?;
	if current > STEPS.len() {
		return Err(StoreError::Corrupt(format!(
			"concierge_projection schema is at version {current}, newer than this build's {}; roll forward, not back",
			STEPS.len()
		)));
	}
	for step in &STEPS[current..] {
		for statement in *step {
			sqlx::query(*statement).execute(&mut *tx).await.map_err(backend("apply a schema step"))?;
		}
	}
	let version = i32::try_from(STEPS.len()).map_err(|_| StoreError::Corrupt("more schema steps than an INTEGER holds".into()))?;
	sqlx::query("INSERT INTO concierge_projection_schema (id, version) VALUES (TRUE, $1) ON CONFLICT (id) DO UPDATE SET version = EXCLUDED.version")
		.bind(version)
		.execute(&mut *tx)
		.await
		.map_err(backend("record the schema version"))?;
	tx.commit().await.map_err(backend("commit migration"))
}

#[derive(Clone, Debug)]
pub struct PgProjectionStore {
	pool: PgPool,
}

impl PgProjectionStore {
	/// Over a pool whose schema [`migrate`] has already brought up to date.
	pub fn new(pool: PgPool) -> Self {
		Self { pool }
	}
}

#[async_trait]
impl ProjectionStore for PgProjectionStore {
	async fn cursor(&self, stream: Stream) -> Result<i64, StoreError> {
		let position: Option<i64> = sqlx::query_scalar("SELECT position FROM concierge_projection_cursor WHERE stream = $1")
			.bind(stream.as_str())
			.fetch_optional(&self.pool)
			.await
			.map_err(backend("read cursor"))?;
		Ok(position.unwrap_or(0))
	}

	async fn advance_cursor(&self, stream: Stream, to: i64) -> Result<(), StoreError> {
		sqlx::query(
			"INSERT INTO concierge_projection_cursor (stream, position) VALUES ($1, $2)
			 ON CONFLICT (stream) DO UPDATE SET position = GREATEST(concierge_projection_cursor.position, EXCLUDED.position), updated_at = now()",
		)
		.bind(stream.as_str())
		.bind(to)
		.execute(&self.pool)
		.await
		.map_err(backend("advance cursor"))?;
		Ok(())
	}

	async fn is_applied(&self, event_id: &str) -> Result<bool, StoreError> {
		sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM concierge_projection_applied WHERE event_id = $1)")
			.bind(event_id)
			.fetch_one(&self.pool)
			.await
			.map_err(backend("check applied"))
	}

	async fn sequence(&self, key: &str) -> Result<Option<u64>, StoreError> {
		let sequence: Option<i64> = sqlx::query_scalar("SELECT sequence FROM concierge_projection_sequence WHERE key = $1")
			.bind(key)
			.fetch_optional(&self.pool)
			.await
			.map_err(backend("read sequence"))?;
		sequence.map(|s| to_u64("sequence", s)).transpose()
	}

	async fn record_applied(&self, stream: Stream, event_id: &str, key: &str, sequence: u64) -> Result<(), StoreError> {
		let sequence = to_i64("sequence", sequence)?;
		let mut tx = self.pool.begin().await.map_err(backend("begin record_applied"))?;
		sqlx::query("INSERT INTO concierge_projection_applied (event_id, stream) VALUES ($1, $2) ON CONFLICT (event_id) DO NOTHING")
			.bind(event_id)
			.bind(stream.as_str())
			.execute(&mut *tx)
			.await
			.map_err(backend("record applied"))?;
		raise(&mut *tx, key, sequence).await?;
		tx.commit().await.map_err(backend("commit record_applied"))
	}

	async fn raise_sequence(&self, key: &str, sequence: u64) -> Result<(), StoreError> {
		raise(&self.pool, key, to_i64("sequence", sequence)?).await
	}

	async fn park(&self, parked: &Parked) -> Result<(), StoreError> {
		sqlx::query(
			"INSERT INTO concierge_projection_parked (stream, position, event_id, key, reason) VALUES ($1, $2, $3, $4, $5)
			 ON CONFLICT (stream, position) DO NOTHING",
		)
		.bind(parked.stream.as_str())
		.bind(parked.position)
		.bind(&parked.event_id)
		.bind(&parked.key)
		.bind(parked.reason.as_str())
		.execute(&self.pool)
		.await
		.map_err(backend("park"))?;
		Ok(())
	}

	async fn has_parked(&self, key: &str) -> Result<bool, StoreError> {
		sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM concierge_projection_parked WHERE key = $1)")
			.bind(key)
			.fetch_one(&self.pool)
			.await
			.map_err(backend("check parked"))
	}

	async fn parked(&self, key: &str) -> Result<Vec<Parked>, StoreError> {
		let rows = sqlx::query("SELECT stream, position, event_id, key, reason FROM concierge_projection_parked WHERE key = $1 ORDER BY position")
			.bind(key)
			.fetch_all(&self.pool)
			.await
			.map_err(backend("list parked"))?;
		rows.into_iter()
			.map(|row| {
				let stream: String = row.try_get("stream").map_err(backend("decode parked.stream"))?;
				let reason: String = row.try_get("reason").map_err(backend("decode parked.reason"))?;
				parked_row(
					&stream,
					row.try_get("position").map_err(backend("decode parked.position"))?,
					row.try_get("event_id").map_err(backend("decode parked.event_id"))?,
					row.try_get("key").map_err(backend("decode parked.key"))?,
					&reason,
				)
			})
			.collect()
	}

	async fn parked_keys(&self) -> Result<Vec<String>, StoreError> {
		sqlx::query_scalar("SELECT DISTINCT key FROM concierge_projection_parked ORDER BY key")
			.fetch_all(&self.pool)
			.await
			.map_err(backend("list parked keys"))
	}

	async fn parked_count(&self) -> Result<u64, StoreError> {
		let count: i64 = sqlx::query_scalar("SELECT count(*) FROM concierge_projection_parked")
			.fetch_one(&self.pool)
			.await
			.map_err(backend("count parked"))?;
		to_u64("parked count", count)
	}

	async fn unpark(&self, stream: Stream, position: i64) -> Result<(), StoreError> {
		sqlx::query("DELETE FROM concierge_projection_parked WHERE stream = $1 AND position = $2")
			.bind(stream.as_str())
			.bind(position)
			.execute(&self.pool)
			.await
			.map_err(backend("unpark"))?;
		Ok(())
	}
}

async fn raise<'e>(executor: impl sqlx::PgExecutor<'e>, key: &str, sequence: i64) -> Result<(), StoreError> {
	sqlx::query(
		"INSERT INTO concierge_projection_sequence (key, sequence) VALUES ($1, $2)
		 ON CONFLICT (key) DO UPDATE SET sequence = GREATEST(concierge_projection_sequence.sequence, EXCLUDED.sequence)",
	)
	.bind(key)
	.bind(sequence)
	.execute(executor)
	.await
	.map_err(backend("raise sequence"))?;
	Ok(())
}
