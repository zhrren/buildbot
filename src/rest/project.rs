use axum::http::StatusCode;
use axum::Json;
use axum::routing::{get, MethodRouter};

pub fn project() -> Vec<(&'static str, MethodRouter)> {
  return vec![("/project/next_build_number", get(next_build_number))];
}

async fn next_build_number() -> Result<Json<i64>, StatusCode> {
  // let PROJECT_REPO = ProjectRepoImpl::new();
  // let PROJECT_MANAGER = ProjectManager::new(&PROJECT_REPO);
  // let value = PROJECT_MANAGER.next_build_number("android-natural").await;
  Ok(Json(1))

  // Json(1)
}

