use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use di::injectable;
use log::info;

use crate::domain::entity::prelude::Project;
use crate::domain::entity::project;
use crate::domain::generator::Generator;
use crate::meta;
use crate::util::build_util::BuildUtil;
use crate::util::meta::Meta;

#[async_trait]
pub trait ProjectRepo {
  async fn create(&self, model: project::Model) -> anyhow::Result<project::Model>;
  async fn update(&self, model: project::Model) -> anyhow::Result<project::Model>;
  async fn delete(&self, model: project::Model) -> anyhow::Result<u64>;
  async fn get(&self, app_name: String) -> anyhow::Result<Option<project::Model>>;
  async fn find(&self) -> anyhow::Result<Vec<project::Model>>;
}

#[injectable]
pub struct ProjectManager {
  generator: Arc<Generator>,
  project_repo: Arc<dyn ProjectRepo>,
}

impl ProjectManager {
  pub async fn gen_build_number(&self, app_name: String) -> anyhow::Result<i64> {
    let mut result = self.project_repo.get(app_name.clone()).await?;
    let mut project_model: project::Model;
    if (result.is_some()) {
      project_model = result.unwrap();
      project_model.updated = self.generator.now();
    } else {
      project_model = project::Model {
        id: self.generator.unique_id(),
        app_name: app_name.clone(),
        build_number: 0,
        created: self.generator.now(),
        updated: self.generator.now(),
      };
      self.project_repo.create(project_model.clone()).await?;
    }
    project_model.build_number = BuildUtil::gen_build_number(project_model.build_number);
    let result = self.project_repo.update(project_model.clone()).await;

    info!(
      "[{}] generate build number: {}",
      app_name, project_model.build_number
    );
    return Ok(result?.build_number);
  }
}
