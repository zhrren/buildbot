use axum::Json;
use axum::response::IntoResponse;
use axum::routing::{get, MethodRouter};
use serde::{Deserialize, Serialize};
use sqlx::Executor;
use crate::domain::project::ProjectRepo;
use crate::repository::memory::{MemoryRepository};
use crate::repository::Repository;
use crate::rest::GET_IT;

pub fn auth() -> Vec<(&'static str, MethodRouter)> {
  return vec![("/auth/token", get(token))];
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Token {
  token: &'static str,
}

async fn token() -> impl IntoResponse {
  // let pool = &GET_IT.db_pool;
  // pool.execute_many("select now()");

  let repo = GET_IT.provide::<MemoryRepository>();
  let item = repo.get(1).unwrap();
  println!("{:?}", item);


  // let service = GET_IT.provide::<UserService>();
  // println!("{:?}", service.get(1));

  let result = Token {
    token: "1234567890",
  };
  Json(result)
}

