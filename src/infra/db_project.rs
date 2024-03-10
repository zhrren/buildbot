use std::sync::Arc;

use async_trait::async_trait;
use di::injectable;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter};
use sea_orm::ActiveValue::Set;

use crate::domain::entity::prelude::Project;
use crate::domain::entity::project;
use crate::domain::generator::Generator;
use crate::domain::project_manager::ProjectRepo;
use crate::infra::db_client::DbClient;

#[injectable(ProjectRepo)]
pub struct ProjectRepoImpl {
  client: Arc<DbClient>,
  generator: Arc<Generator>,
}

#[async_trait]
impl ProjectRepo for ProjectRepoImpl {
  async fn create(&self, model: project::Model) -> anyhow::Result<project::Model> {
    let conn = self.client.get().await;
    let result = model.into_active_model().insert(conn.as_ref()).await;
    return Ok(result?);
  }

  async fn update(&self, model: project::Model) -> anyhow::Result<project::Model> {
    let conn = self.client.get().await;
    let mut am = model.clone().into_active_model();
    am.build_number = Set(model.build_number);
    am.updated = Set(model.updated);
    let result = am.update(conn.as_ref()).await;
    return Ok(result?);
  }

  async fn delete(&self, model: project::Model) -> anyhow::Result<u64> {
    let conn = self.client.get().await;
    let result = model.into_active_model().delete(conn.as_ref()).await;
    return Ok(result?.rows_affected);
  }

  async fn get(&self, app_name: String) -> anyhow::Result<Option<project::Model>> {
    let conn = self.client.get().await;
    let result = Project::find()
      .filter(project::Column::AppName.eq(app_name))
      .one(conn.as_ref())
      .await;
    return Ok(result?);
  }

  async fn find(&self) -> anyhow::Result<Vec<project::Model>> {
    let conn = self.client.get().await;
    let result = Project::find().all(conn.as_ref()).await;
    return Ok(result?);
  }
}
