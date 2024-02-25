use std::fmt::{Debug, Error};
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
pub trait ProjectRepo: Debug {
  async fn get(&self, app_name: &str) -> Result<Project, sqlx::Error>;
  async fn save(&self, project: &Project);
  async fn delete(&self, app_name: &str);
  async fn find(&self) -> Vec<Project>;
}

#[derive(Debug)]
pub struct ProjectManager<'a> {
  repo: &'a (dyn ProjectRepo + Send + Sync),
}

impl<'a> ProjectManager<'_> {
  pub fn new(repo: &(dyn ProjectRepo + Send + Sync)) -> ProjectManager {
    return ProjectManager {
      repo
    }
  }

  pub async fn next_build_number(&self, app_name: &str) -> Result<i64,Error> {
    return Ok(112);
  }
}

pub async fn next_build_number2(app_name: &str) -> Result<i64,Error> {
  return Ok(112);
}
