use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use sqlx::{query_as, Pool, Sqlite};
use std::ops::Deref;
use std::str::FromStr;
use std::time::Duration;

use crate::domain::project::{Project, ProjectRepo};
use crate::infra::db::POOL;
use crate::kernel::settings::SETTINGS;

impl ProjectRepo for Project {
  async fn get(&self, app_name: &str) -> Option<Project> {
    let item = query_as!(Project, "select id,app_name,build_number from t_project where app_name = ?", app_name)
      .fetch_optional(POOL.deref())
      .await
      .unwrap();
    return item;
  }

  async fn save(&self, project: Project) {
    todo!()
  }

  async fn delete(&self, app_name: &str) {
    todo!()
  }

  async fn find(&self) -> Vec<Project> {
    todo!()
  }
}
