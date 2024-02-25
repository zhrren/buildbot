use axum::extract::State;
use axum::http::{Request, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, MethodRouter};
use axum::Json;
use serde::{Deserialize, Serialize};
use crate::domain::project::ProjectManager;
use crate::rest::context::Context;

pub fn auth() -> Vec<(&'static str, MethodRouter<Context>)> {
    return vec![("/auth/get_token", get(get_token))];
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Token {
    token: &'static str,
}
async fn get_token(State(context): State<Context>) -> impl IntoResponse {
    let result = Token {
        token: "1234567890",
    };
    Json(result)
}

