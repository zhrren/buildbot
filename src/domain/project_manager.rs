use anyhow::anyhow;
use crate::domain::entity::prelude::Project;
use crate::meta;
use crate::util::meta::{Meta};

pub struct ProjectManager {}

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
