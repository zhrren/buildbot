use axum::Json;
use axum::response::IntoResponse;
use axum::routing::{get, MethodRouter};
use serde::{Deserialize, Serialize};

pub fn routes() -> Vec<(String, MethodRouter)> {
  return vec![("/auth/token".to_string(), get(token))];
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
