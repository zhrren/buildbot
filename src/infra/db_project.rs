use std::sync::Arc;

use crate::domain::entity::prelude::Project;
use async_trait::async_trait;
use chrono::Utc;
use di::injectable;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QueryOrder};

use crate::domain::entity::project;
use crate::domain::generator::Generator;
use crate::domain::project::ProjectRepo;
use crate::infra::db::DbClient;

#[injectable(ProjectRepo)]
pub struct ProjectRepoImpl {
  client: Arc<DbClient>,
  generator: Arc<Generator>,
}

#[async_trait]
impl ProjectRepo for ProjectRepoImpl {
  async fn create(&self, app_name: String, build_number: i64) -> anyhow::Result<project::Model> {
    let conn = self.client.get().await;
    let result = project::ActiveModel {
      id: Set(self.generator.unique_id()),
      app_name: Set(app_name),
      build_number: Set(0),
      created: Set(Utc::now().to_rfc3339()),
      updated: Set(Utc::now().to_rfc3339()),
      ..Default::default()
    }
    .insert(conn.as_ref())
    .await;
    return Ok(result?);
  }

  async fn update(
    &self,
    id: i64,
    app_name: String,
    build_number: i64,
  ) -> anyhow::Result<project::Model> {
    let conn = self.client.get().await;
    let mut post: project::ActiveModel = Project::find_by_id(id)
      .one(conn.as_ref())
      .await?
      .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
      .map(Into::into)?;

    post.app_name = Set(app_name);
    post.build_number = Set(build_number);
    post.updated = Set(Utc::now().to_rfc3339());
    let result = post.update(conn.as_ref()).await;
    return Ok(result?);
  }

  async fn get(&self, id: i64) -> anyhow::Result<Option<project::Model>> {
    let conn = self.client.get().await;
    let result = Project::find_by_id(id).one(conn.as_ref()).await;
    return Ok(result?);
  }

  async fn find(&self) -> anyhow::Result<Vec<project::Model>> {
    let conn = self.client.get().await;
    let result = Project::find().all(conn.as_ref()).await;
    return Ok(result?);
  }
}
