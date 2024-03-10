use crate::domain::entity::prelude::Project;
use crate::domain::entity::project;
use crate::meta;
use crate::util::meta::Meta;
use anyhow::anyhow;
use async_trait::async_trait;
use di::injectable;
use log::info;
use std::future::Future;
use std::rc::Rc;
use std::sync::Arc;

#[async_trait]
pub trait ProjectRepo {
  async fn create(&self, app_name: String, build_number: i64) -> anyhow::Result<project::Model>;
  async fn update(
    &self,
    id: i64,
    app_name: String,
    build_number: i64,
  ) -> anyhow::Result<project::Model>;
  async fn get(&self, id: i64) -> anyhow::Result<Option<project::Model>>;
  async fn find(&self) -> anyhow::Result<Vec<project::Model>>;
}

#[injectable]
pub struct ProjectManager {
  repo: Arc<dyn ProjectRepo>,
}

impl ProjectManager {
  pub async fn create_project(
    &self,
    app_name: String,
    build_number: i64,
  ) -> anyhow::Result<Project> {
    // let project = Project::find_first().await?;
    // if project.is_none() {
    //   let project = Project::create().set(app_name).set(build_number).exec().await;
    //   return Ok(project);
    // }
    self.repo.create(app_name, build_number).await;
    info!("----------------------------------- create_project");
    return Err(meta!("project_already_exists"));
  }
}
