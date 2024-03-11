use std::time::Duration;
use axum::http::StatusCode;
use axum::Json;
use axum::routing::{get, MethodRouter};
use di::injectable;
use sea_orm::{ActiveModelTrait, ConnectOptions, Database, DatabaseConnection, EntityTrait, IntoActiveModel};
use serde::{Deserialize, Serialize};
use crate::domain::entity::prelude::Project;
use crate::domain::entity::project;
use crate::kernel::SETTINGS;
use axum::{
  async_trait,
};
use crate::config::inject_config::get_it;
use crate::domain::project_manager::ProjectManager;

pub fn routes() -> Vec<(String, MethodRouter)> {
  return vec![("/gen_build_number".to_string(), get(gen_build_number))];
}

#[derive(Serialize, Deserialize, Debug)]
struct GenBuildNumberParams {
  app_name: String,
  format: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GenBuildNumberResult {
  build_number: i64,
}


// async fn gen_build_number2() -> Json<i64> {
//   let build_number = gen_build_number().await.unwrap();
//   Json(build_number)
// }

async fn gen_build_number() -> Result<Json<i64>, StatusCode> {
  let project_manager = get_it::<ProjectManager>();
  let dd = project_manager.gen_build_number("test".to_string()).await.unwrap();
  Ok(Json(dd))
}

type ResultResponse<T> = std::result::Result<T, ResponseError>;

struct ResponseError(anyhow::Error);

impl From<anyhow::Error> for ResponseError {
  fn from(error: anyhow::Error) -> Self {
    Self(error)
  }
}

impl axum::response::IntoResponse for ResponseError {
  fn into_response(self) -> axum::response::Response {
    // let status_code = if let Some(error) = self.0.downcast_ref::<ApiError>() {
    //   match error {
    //     ApiError::NotFound => axum::http::StatusCode::NOT_FOUND,
    //     ApiError::BadRequest => axum::http::StatusCode::BAD_REQUEST,
    //   }
    // } else {
    //   axum::http::StatusCode::INTERNAL_SERVER_ERROR
    // };
    let status_code = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
    return (status_code, self.0.to_string()).into_response();
  }
}
