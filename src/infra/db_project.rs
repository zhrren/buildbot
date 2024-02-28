use std::ops::Deref;

use async_trait::async_trait;
use nject::injectable;
use sqlx::{Pool, query_as, Sqlite, SqlitePool};

use crate::domain::project::{Project, ProjectRepo};

#[injectable]
#[derive(Debug)]
pub struct ProjectRepoImpl {
  #[inject(SqlitePool::connect_lazy("file::memory:?cache=shared").expect("Invalid database URL"))]
  pool: SqlitePool,

}

#[async_trait]
impl ProjectRepo for ProjectRepoImpl {
  async fn get(&self, app_name: &str) -> Result<Project, sqlx::Error> {
    // let item = query_as!(Project, "select id,app_name,build_number from t_project where app_name = ?", app_name)
    //   .fetch_one(self.pool.deref())
    //   .await;
    // return item;
    todo!()
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
