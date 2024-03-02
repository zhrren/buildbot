use std::cell::{Cell, RefCell};
use std::str::FromStr;
use std::time::Duration;

use di::injectable;
use sqlx::{Connection, ConnectOptions, Executor, Pool, Sqlite};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};

use crate::kernel::SETTINGS;

pub async fn create_pool() -> Pool<Sqlite> {
  let db_options = SqliteConnectOptions::from_str(SETTINGS.database_url.as_str())
    .unwrap()
    .journal_mode(SqliteJournalMode::Wal)
    .statement_cache_capacity(53)
    .busy_timeout(Duration::from_secs(30));

  let pool = SqlitePoolOptions::new()
    .max_connections(3)
    .connect_with(db_options)
    .await
    .unwrap();

  return pool;
}
