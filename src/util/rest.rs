use axum::http::StatusCode;
use axum::Json;
use axum::routing::MethodRouter;

pub trait Rest {
  fn configure(self) -> Vec<(String, MethodRouter)>;
}
