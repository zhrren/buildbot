use axum::response::IntoResponse;
use axum::routing::{get, MethodRouter};
use axum::Json;
use serde::{Deserialize, Serialize};

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
