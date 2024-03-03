use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use crate::kernel::SETTINGS;

// pub async fn create_pool() -> Pool<Sqlite> {
//   let db_options = SqliteConnectOptions::from_str(SETTINGS.database_url.as_str())
//     .unwrap()
//     .journal_mode(SqliteJournalMode::Wal)
//     .statement_cache_capacity(53)
//     .busy_timeout(Duration::from_secs(30));
//
//   let pool = SqlitePoolOptions::new()
//     .max_connections(3)
//     .connect_with(db_options)
//     .await
//     .unwrap();
//
//   return pool;
// }

pub async  fn create_pool() -> DatabaseConnection {
  let mut opt = ConnectOptions::new(SETTINGS.database_url.as_str());
  opt.max_connections(100)
    .min_connections(5)
    .connect_timeout(Duration::from_secs(8))
    .acquire_timeout(Duration::from_secs(8))
    .idle_timeout(Duration::from_secs(8))
    .max_lifetime(Duration::from_secs(8))
    .sqlx_logging(true)
    .sqlx_logging_level(log::LevelFilter::Info)
    .set_schema_search_path("my_schema"); // Setting default PostgreSQL schema

  let db = Database::connect(opt).await.unwrap();
  return db;
}
