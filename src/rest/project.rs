use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use axum::routing::{MethodRouter, post};

use crate::domain::project::ProjectManager;
use crate::infra::db_project::ProjectRepoImpl;

pub fn project() -> Vec<(&'static str, MethodRouter)> {
  return vec![("/project/next_build_number", post(next_build_number))];
}

async fn next_build_number() -> impl IntoResponse {
  // let project_repo = ProjectRepoImpl::new();
  // let project_manager = ProjectManager::new(&project_repo);
  // let value = project_manager.next_build_number("android-natural").await.unwrap();
  // Json(value)
  Json(1)
}
