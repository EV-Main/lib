//! `concierge_projection`'s Postgres store and migration against a real server.
//!
//! Needs `EV_LIB_TEST_DATABASE_URL` (a role that may `CREATE DATABASE`); each test runs in
//! a database of its own, dropped at the end. Unset, every test prints SKIPPED and passes
//! vacuously — a dedicated variable rather than `DATABASE_URL`, so a shell pointed at a
//! real database never has throwaway databases created on it.
#![cfg(feature = "concierge_projection_pg")]

#[macro_use]
#[path = "support/projection_store.rs"]
mod projection_store;

use ev_lib::concierge_projection::{PgProjectionStore, ProjectionStore, StoreError, Stream, pg};
use sqlx::{
	AssertSqlSafe, PgPool,
	postgres::{PgConnectOptions, PgPoolOptions},
};
use tokio::task::JoinSet;

const URL_VAR: &str = "EV_LIB_TEST_DATABASE_URL";

/// A database of its own on the test server.
struct Database {
	admin: PgPool,
	name: String,
	options: PgConnectOptions,
	pool: PgPool,
}

impl Database {
	async fn create() -> Option<Self> {
		let Ok(url) = std::env::var(URL_VAR) else {
			eprintln!("SKIPPED: {URL_VAR} is unset, the Postgres store was not tested");
			return None;
		};
		let admin = PgPoolOptions::new().max_connections(1).connect(&url).await.unwrap();
		let name = format!("ev_lib_test_{}", uuid::Uuid::new_v4().simple());
		sqlx::query(AssertSqlSafe(format!("CREATE DATABASE {name}"))).execute(&admin).await.unwrap();
		let options = url.parse::<PgConnectOptions>().unwrap().database(&name);
		let pool = PgPoolOptions::new().max_connections(4).connect_with(options.clone()).await.unwrap();
		Some(Self { admin, name, options, pool })
	}

	/// A pool of its own, as another replica of the service would have.
	async fn replica(&self) -> PgPool {
		PgPoolOptions::new().max_connections(1).connect_with(self.options.clone()).await.unwrap()
	}

	async fn drop(self) {
		self.pool.close().await;
		sqlx::query(AssertSqlSafe(format!("DROP DATABASE {} WITH (FORCE)", self.name)))
			.execute(&self.admin)
			.await
			.unwrap();
	}
}

async fn fresh_pg_store() -> Option<(PgProjectionStore, Database)> {
	let database = Database::create().await?;
	pg::migrate(&database.pool).await.unwrap();
	Some((PgProjectionStore::new(database.pool.clone()), database))
}

all_store_contract_tests!(fresh_pg_store, Database::drop);

async fn schema_version(pool: &PgPool) -> i32 {
	sqlx::query_scalar("SELECT version FROM concierge_projection_schema").fetch_one(pool).await.unwrap()
}

#[tokio::test]
async fn pg_migrate_should_be_idempotent() {
	let Some((store, database)) = fresh_pg_store().await else { return };
	store.advance_cursor(Stream::Scope, 7).await.unwrap();

	pg::migrate(&database.pool).await.unwrap();
	pg::migrate(&database.pool).await.unwrap();

	assert_eq!(schema_version(&database.pool).await, 1);
	assert_eq!(store.cursor(Stream::Scope).await.unwrap(), 7);
	database.drop().await;
}

#[tokio::test]
async fn pg_migrate_should_refuse_a_schema_newer_than_the_build() {
	let Some((_, database)) = fresh_pg_store().await else { return };
	sqlx::query("UPDATE concierge_projection_schema SET version = 2").execute(&database.pool).await.unwrap();

	let error = pg::migrate(&database.pool).await.unwrap_err();

	assert!(matches!(&error, StoreError::Corrupt(m) if m.contains("version 2")), "{error:?}");
	database.drop().await;
}

#[tokio::test]
async fn pg_replicas_booting_together_should_all_migrate() {
	let Some(database) = Database::create().await else { return };
	let mut replicas = JoinSet::new();
	for _ in 0..8 {
		let pool = database.replica().await;
		replicas.spawn(async move { pg::migrate(&pool).await });
	}

	let outcomes = replicas.join_all().await;

	assert!(outcomes.iter().all(Result::is_ok), "{outcomes:?}");
	assert_eq!(schema_version(&database.pool).await, 1);
	database.drop().await;
}

#[tokio::test]
async fn pg_record_applied_should_leave_no_trace_when_raising_the_sequence_fails() {
	let Some((store, database)) = fresh_pg_store().await else { return };
	sqlx::query("ALTER TABLE concierge_projection_sequence ADD CONSTRAINT refuse_poisoned_key CHECK (key <> 'user:poisoned')")
		.execute(&database.pool)
		.await
		.unwrap();

	let error = store.record_applied(Stream::UserLifecycle, "u-1", "user:poisoned", 1).await.unwrap_err();

	assert!(matches!(error, StoreError::Backend { .. }), "{error:?}");
	assert!(!store.is_applied("u-1").await.unwrap(), "the event id was recorded without its sequence");
	database.drop().await;
}

#[tokio::test]
async fn pg_parked_should_report_a_row_with_an_unknown_reason_as_corrupt() {
	let Some((store, database)) = fresh_pg_store().await else { return };
	sqlx::query("INSERT INTO concierge_projection_parked (stream, position, event_id, key, reason) VALUES ('scope', 1, 'e-1', 'scope:sa', 'from_the_future')")
		.execute(&database.pool)
		.await
		.unwrap();

	let error = store.parked("scope:sa").await.unwrap_err();

	assert!(matches!(error, StoreError::Corrupt(_)), "{error:?}");
	database.drop().await;
}
