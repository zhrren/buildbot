use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Project {
  pub id: i64,
  pub app_name: String,
  pub build_number: i64,
}
pub trait ProjectRepo {
  async fn get(&self, app_name: &str) -> Option<Project>;
  async fn save(&self, project: Project);
  async fn delete(&self, app_name: &str);
  async fn find(&self) -> Vec<Project>;
}

pub struct ProjectManager<'a> {
  repo: ProjectRepo + 'a,
}

impl ProjectManager {
  pub fn new(repo: impl ProjectRepo + 'static) -> ProjectManager {
    return ProjectManager {
      repo,
    }
   }
}

pub mod manager {
  pub async fn next_build_number(app_name: &str) -> i64 {

    return 1;
  }
}
