use std::time::Duration;
use migration::{Migrator, MigratorTrait};

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use crate::kernel::SETTINGS;

pub async  fn create_pool() -> DatabaseConnection {
  let mut options = ConnectOptions::new(SETTINGS.database_url.as_str());
  options.max_connections(100)
    .min_connections(5)
    .connect_timeout(Duration::from_secs(8))
    .acquire_timeout(Duration::from_secs(8))
    .idle_timeout(Duration::from_secs(8))
    .max_lifetime(Duration::from_secs(8))
    .sqlx_logging(true)
    .sqlx_logging_level(log::LevelFilter::Info)
    .set_schema_search_path("my_schema"); // Setting default PostgreSQL schema

  let db = Database::connect(options).await.unwrap();
  Migrator::up(&db, None).await;
  return db;
}
