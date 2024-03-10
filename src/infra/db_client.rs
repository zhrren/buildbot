use idgenerator_thin::{IdGeneratorOptions, YitIdHelper};
use migration::{Migrator, MigratorTrait};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use crate::kernel::SETTINGS;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbConn};

pub struct DbClient {
  connection: RwLock<Option<Arc<DatabaseConnection>>>,
}

impl DbClient {
  pub fn new() -> DbClient {
    DbClient {
      connection: RwLock::new(None),
    }
  }

  pub async fn configure(&self) {
    let mut options = ConnectOptions::new(SETTINGS.database_url.as_str());
    options
      .max_connections(100)
      .min_connections(5)
      .connect_timeout(Duration::from_secs(8))
      .acquire_timeout(Duration::from_secs(8))
      .idle_timeout(Duration::from_secs(8))
      .max_lifetime(Duration::from_secs(8))
      .sqlx_logging(true)
      .sqlx_logging_level(log::LevelFilter::Info)
      .set_schema_search_path("my_schema"); // Setting default PostgreSQL schema

    let connection: DatabaseConnection = Database::connect(options).await.unwrap();
    Migrator::up(&connection, None)
      .await
      .expect("Migrator up failed");

    let connection_ref = &self.connection;
    let mut connection_ref_write = connection_ref.write().unwrap();
    connection_ref_write.replace(Arc::new(connection));
  }

  pub async fn get(&self) -> Arc<DatabaseConnection> {
    let connection_ref = &self.connection;
    return connection_ref.read().unwrap().clone().unwrap();
  }
}
