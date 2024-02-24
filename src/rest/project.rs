use axum::http::{Request, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, MethodRouter, post};
use axum::Json;
use serde::{Deserialize, Serialize};

pub fn project() -> Vec<(&'static str, MethodRouter)> {
    return vec![("/project/next_build_number", post(next_build_number))];
}

async fn next_build_number() -> impl IntoResponse {
    let value = crate::domain::project::manager::next_build_number().await;
    return Json(value);
}
