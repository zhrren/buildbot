use axum::http::StatusCode;
use axum::routing::{get, MethodRouter};
use axum::Json;
use crate::util::rest::Rest;

pub fn routes() -> Vec<(String, MethodRouter)> {
  return vec![("/gen_build_number".to_string(), get(gen_build_number))];
}

async fn gen_build_number() -> Result<Json<i64>, StatusCode> {
  Ok(Json(1))
}
