use crate::domain::entity::prelude::Project;

pub struct ProjectManager {}

impl ProjectManager {
  pub async fn create_project(app_name: String, build_number: i64) -> Result<Project, DbErr> {
    let project = Project::find_first().await?;
    if project.is_none() {
      let project = Project::create().set(app_name).set(build_number).exec().await?;
      return Ok(project);
    }
    return Err(DbErr::AlreadyExists);
  }
}
