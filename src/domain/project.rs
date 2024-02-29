use std::fmt::{Debug, Error};
use std::rc::Rc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Project {
  pub id: i64,
  pub app_name: String,
  pub build_number: i64,
}

#[async_trait]
pub trait ProjectRepo {
  async fn get(&self, app_name: &str) -> Result<Project, sqlx::Error>;
  async fn save(&self, project: &Project);
  async fn delete(&self, app_name: &str);
  async fn find(&self) -> Vec<Project>;
}

pub struct ProjectManager {
  repo: Rc<dyn ProjectRepo>,
}

impl ProjectManager {
  pub async fn next_build_number(&self, app_name: &str) -> Result<i64, Error> {
    return Ok(112);
  }
}

pub async fn next_build_number2(app_name: &str) -> Result<i64, Error> {
  return Ok(112);
}
