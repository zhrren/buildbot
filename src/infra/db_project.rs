use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use di::injectable;
//
// use crate::domain::project::{Project, ProjectRepo};
//
// #[injectable(ProjectRepo)]
// pub struct ProjectRepoImpl {
// }
//
// #[async_trait]
// impl ProjectRepo for ProjectRepoImpl {
//   async fn get(&self, app_name: &str) -> anyhow::Result<Option<Project>> {
//     // let item = query_as!(Project, "select id,app_name,build_number from t_project where app_name = ?", app_name)
//     //   .fetch_one(self.pool.deref())
//     //   .await;
//     // return item;
//     todo!()
//   }
//
//   async fn save(&self, project: &Project) {
//     todo!()
//   }
//
//   async fn delete(&self, app_name: &str) {
//     todo!()
//   }
//
//   async fn find(&self) -> Vec<Project> {
//     vec![]
//   }
// }
