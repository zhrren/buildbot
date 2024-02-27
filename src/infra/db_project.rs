use std::ops::Deref;

use async_trait::async_trait;
use sqlx::query_as;

use crate::domain::project::{Project, ProjectRepo};

#[derive(Debug)]
pub struct ProjectRepoImpl {
  pool: &'static sqlx::Pool<sqlx::Sqlite>,
}

impl ProjectRepoImpl {
  pub fn new(pool: &'static sqlx::Pool<sqlx::Sqlite>) -> Self {
    Self { pool }
  }
}

#[async_trait]
impl ProjectRepo for ProjectRepoImpl {
  async fn get(&self, app_name: &str) -> Result<Project, sqlx::Error> {
    let item = query_as!(Project, "select id,app_name,build_number from t_project where app_name = ?", app_name)
      .fetch_one(self.pool.deref())
      .await;
    return item;
  }

  async fn save(&self, project: &Project) {
    todo!()
  }

  async fn delete(&self, app_name: &str) {
    todo!()
  }

  async fn find(&self) -> Vec<Project> {
    vec![]
  }
}
