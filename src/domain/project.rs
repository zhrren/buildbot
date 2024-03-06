use anyhow::anyhow;
use crate::domain::entity::prelude::Project;
use crate::domain::entity::project;
use crate::meta;
use crate::util::meta::{Meta};

pub trait ProjectRepo {
  fn create(&self, app_name: String, build_number: i64) -> anyhow::Result<project::Model>;
  fn update(&self, id: i64, app_name: String, build_number: i64) -> anyhow::Result<project::Model>;
  fn get(&self, id: i64) -> anyhow::Result<project::Model>;
  fn find(&self) -> anyhow::Result<Vec<project::Model>>;
}

pub struct ProjectManager {
  repo: Box<dyn ProjectRepo>,
}

impl ProjectManager {
  pub async fn create_project(&self, app_name: String, build_number: i64) -> anyhow::Result<Project> {
    // let project = Project::find_first().await?;
    // if project.is_none() {
    //   let project = Project::create().set(app_name).set(build_number).exec().await;
    //   return Ok(project);
    // }
    return Err(meta!("project_already_exists"));
  }
}
