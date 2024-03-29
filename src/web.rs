use std::ops::Deref;

use axum::body::Body;
use axum::extract::State;
use axum::http::{HeaderValue, Request};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, MethodRouter};
use axum::{Error, Json, RequestExt, Router};
use log::info;

use crate::kernel::settings::SETTINGS;

pub async fn serve(routes: Vec<(String, MethodRouter)>) {
  info!(
    "web server starting on port http://127.0.0.1:{}",
    SETTINGS.app_port
  );

  let app = create_router(routes)
    .layer(axum::middleware::from_fn(logging))
    .layer(axum::middleware::from_fn(validation));
  let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", SETTINGS.app_port))
    .await
    .unwrap();
  axum::serve(listener, app).await.unwrap();
}

fn create_router(routes: Vec<(String, MethodRouter)>) -> Router {
  let mut app = Router::new();
  for route in routes {
    app = app.route(route.0.as_str(), route.1)
  }
  return app;
}

async fn logging(request: Request<Body>, next: Next) -> Response {
  let uri = request.uri().clone();
  info!("request {}", uri.to_string());
  next.run(request).await
}

async fn validation(request: Request<Body>, next: Next) -> Response {
  let path = request.uri().path();
  if ANONYMOUS_ROUTES.contains(&path) {
    return next.run(request).await;
  }
  let authorization = request.headers().get("Authorization");
  if authorization.is_none() {
    return Response::builder()
      .status(401)
      .body(Body::from("Unauthorized"))
      .unwrap();
  }

  let token = authorization.unwrap().to_str().unwrap().trim();
  if token.is_empty() {
    return Response::builder()
      .status(401)
      .body(Body::from("Unauthorized"))
      .unwrap();
  }

  next.run(request).await
}

lazy_static! {
  static ref ANONYMOUS_ROUTES: Vec<&'static str> = vec!["/auth/token", "/gen_build_number"];
}
