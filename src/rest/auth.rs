use axum::Json;
use axum::response::IntoResponse;
use axum::routing::{get, MethodRouter};
use serde::{Deserialize, Serialize};

pub fn auth() -> Vec<(&'static str, MethodRouter)> {
  return vec![("/auth/get_token", get(get_token))];
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Token {
  token: &'static str,
}

async fn get_token() -> impl IntoResponse {
  let result = Token {
    token: "1234567890",
  };
  Json(result)
}

