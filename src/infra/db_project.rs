use std::fmt::Error;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use sqlx::{query_as, Pool, Sqlite};
use std::ops::Deref;

use crate::domain::project::{Project, ProjectRepo};
use crate::infra::db::POOL;
use crate::kernel::meta::Meta;

pub struct ProjectRepoImpl { }

impl ProjectRepoImpl {
  pub fn new() -> Self {
    Self {}
  }
}

impl ProjectRepo for ProjectRepoImpl {
  // async fn get(&self, app_name: &str) -> Result<Project, Error> {
  //   let item = query_as!(Project, "select id,app_name,build_number from t_project where app_name = ?", app_name)
  //     .fetch_one(POOL.deref())
  //     .await
  //     .into();
  //   return item;
  // }

  // async fn save(&self, project: &Project) {
  //   todo!()
  // }
  //
  // async fn delete(&self, app_name: & str) {
  //   todo!()
  // }
  //
  // async fn find(&self) -> Vec<Project> {
  //   todo!()
  // }
}
