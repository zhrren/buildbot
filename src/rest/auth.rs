use axum::Json;
use axum::response::IntoResponse;
use axum::routing::{get, MethodRouter};
use serde::{Deserialize, Serialize};

use crate::domain::project::ProjectRepo;

pub fn auth() -> Vec<(&'static str, MethodRouter)> {
  return vec![("/auth/token", get(token))];
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Token {
  token: &'static str,
}

async fn token() -> impl IntoResponse {
  let result = Token {
    token: "1234567890",
  };
  Json(result)
}

