use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use di::injectable;
use crate::domain::entity::prelude::Project;
use crate::domain::entity::project;

use crate::domain::project::{ProjectRepo};

#[injectable(ProjectRepo)]
pub struct ProjectRepoImpl {

}

#[async_trait]
impl ProjectRepo for ProjectRepoImpl {
  fn create(&self, app_name: String, build_number: i64) -> anyhow::Result<project::Model> {
    todo!()
  }

  fn update(&self, id: i64, app_name: String, build_number: i64) -> anyhow::Result<project::Model> {
    todo!()
  }

  fn get(&self, id: i64) -> anyhow::Result<project::Model> {
    todo!()
  }

  fn find(&self) -> anyhow::Result<Vec<project::Model>> {
    todo!()
  }
}
