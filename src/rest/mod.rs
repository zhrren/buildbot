use nject::{injectable, module, provider};
use sqlx::{Pool, Sqlite, SqlitePool};
use std::ops::Deref;

use crate::domain::project::ProjectRepo;
use crate::infra::db::create_pool;
use crate::infra::db_project::ProjectRepoImpl;
use crate::repository;
use crate::repository::Repository;

pub mod auth;
pub mod project;

#[module]
#[injectable]
pub struct Module {
  #[export(dyn Repository)]
  repository: repository::memory::MemoryRepository,

  #[export(dyn ProjectRepo)]
  project_repo: ProjectRepoImpl,
}

#[provider]
#[injectable]
pub struct ModuleProvider {
  #[import]
  module: Module,

  #[inject(SqlitePool::connect_lazy("file::memory:?cache=shared").expect("Invalid database URL"))]
  pool: SqlitePool,
}

#[provider]
pub struct Provider {
  // #[provide]
  // db_pool: Pool<Sqlite>,
}

// #[provider]
// pub struct Provider;

lazy_static! {
  pub static ref GET_IT: &'static Provider = {
    let db_pool: SqlitePool = tokio::runtime::Runtime::new()
      .unwrap()
      .block_on(async { create_pool().await });

    let provider = Provider{  };
    // let pool = provider.provide::<SqlitePool>();
    // println!("{}", pool.size());

    let get_it: &'static Provider = Box::leak(Box::new(provider));
    return get_it;
  };
}
