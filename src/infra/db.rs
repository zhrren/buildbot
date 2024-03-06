use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;
use migration::{Migrator, MigratorTrait};

use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbConn};
use crate::kernel::SETTINGS;


pub struct DbClient {
  connection: RefCell<Option<Arc<DatabaseConnection>>>
}

impl DbClient {

  pub fn new() -> DbClient {
    DbClient {
      connection: RefCell::new(None)
    }
  }

  pub async fn get_connection(&self) -> Arc<DatabaseConnection> {
    let connection_ref = &self.connection;
    if connection_ref.borrow().is_none() {
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

      let connection:DatabaseConnection = Database::connect(options).await.unwrap();
      Migrator::up(&connection, None).await.expect("Migrator up failed");

      connection_ref.borrow_mut().replace(Arc::new(connection));
    }
    return connection_ref.borrow().clone().unwrap();
  }
}
