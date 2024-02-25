use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use axum::routing::{get, MethodRouter, post};

use crate::domain::project::ProjectManager;
use crate::infra::db_project::ProjectRepoImpl;
use crate::rest::context::Context;

pub fn project() -> Vec<(&'static str, MethodRouter<Context>)> {
  return vec![("/project/next_build_number", get(next_build_number))];
}

async fn next_build_number(State(context): State<Context>) -> Result<Json<i64>, StatusCode> {
  // let PROJECT_REPO = ProjectRepoImpl::new();
  // let PROJECT_MANAGER = ProjectManager::new(&PROJECT_REPO);
  // let value = PROJECT_MANAGER.next_build_number("android-natural").await;
  Ok(Json(1))

  // Json(1)
}

