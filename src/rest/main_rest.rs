use axum::extract::Query;
use axum::routing::{get, MethodRouter};
use axum::Json;
use sea_orm::ActiveModelTrait;
use serde::{Deserialize, Serialize};

use crate::config::inject_config::get_it;
use crate::domain::project_manager::ProjectManager;
use crate::util::rest::RestError;

pub fn routes() -> Vec<(String, MethodRouter)> {
  return vec![("/gen_build_number".to_string(), get(gen_build_number))];
}

#[derive(Serialize, Deserialize, Debug)]
struct GenBuildNumberParams {
  app_name: String,
  format: Option<String>,
}
async fn gen_build_number(query: Query<GenBuildNumberParams>) -> Result<Json<i64>, RestError> {
  let params = query.0;
  let project_manager = get_it::<ProjectManager>();
  let build_number = project_manager.gen_build_number(params.app_name).await?;
  Ok(Json(build_number))
}
