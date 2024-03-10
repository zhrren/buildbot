use axum::http::StatusCode;
use axum::Json;
use axum::routing::{get, MethodRouter};

pub fn routes() -> Vec<(String, MethodRouter)> {
  return vec![("/gen_build_number".to_string(), get(gen_build_number))];
}

async fn gen_build_number() -> Result<Json<i64>, StatusCode> {
  Ok(Json(1))
}
