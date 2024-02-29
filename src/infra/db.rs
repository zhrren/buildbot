use std::cell::RefCell;
use std::str::FromStr;
use std::time::Duration;
use lazy_static::lazy::Lazy;

use sqlx::{Connection, ConnectOptions, Executor, Pool, Sqlite, SqlitePool};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};

use crate::kernel::SETTINGS;

const INITIAL_SQL: &str = r"
create table if not exists t_project (id integer primary key NOT NULL, app_name text NOT NULL, build_number integer NOT NULL);
";

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

  pool.execute_many(INITIAL_SQL);

  return pool;
}

pub struct DbClient {
  pool: RefCell<Option<Pool<Sqlite>>>,
}

impl DbClient {
  pub async fn get_pool() -> Pool<Sqlite> {
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

    pool.execute_many(INITIAL_SQL);
    return pool;
  }
}
