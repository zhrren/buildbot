use std::fmt::Error;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::kernel::meta::Meta;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Project {
  pub id: i64,
  pub app_name: String,
  pub build_number: i64,
}

#[async_trait]
pub trait ProjectRepo {
  // async fn get(&self, app_name: &str) -> Result<Project, Error>;
  // async fn save(&self, project: &Project);
  // async fn delete(&self, app_name: &str);
  // async fn find(&self) -> Vec<Project>;
}

pub struct ProjectManager<'a> {
  repo: &'a Box<dyn ProjectRepo>,
}

impl ProjectManager<'_> {
  pub fn new(repo: &Box<dyn ProjectRepo>) -> ProjectManager {
    return ProjectManager {
      repo,
    };
  }

  pub async fn next_build_number(&self, app_name: &str) -> Result<i64,Error> {
    return Ok(112);
  }
}
