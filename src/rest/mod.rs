use nject::{injectable, module, provider};
use sqlx::{Pool, Sqlite};

use crate::infra::db::create_pool;
use crate::repository;
use crate::repository::Repository;

pub mod auth;
pub mod project;

lazy_static! {
    static ref POOL: Pool<Sqlite> = {
      tokio::runtime::Runtime::new().unwrap().block_on(async {
        create_pool().await
      })
    };

    pub static ref GET_IT: &'static Provider = {
      #[provider]
      struct InitProvider;

      let get_it: &'static Provider = Box::leak(Box::new(InitProvider.provide()));
      return get_it
    };
}

#[provider]
#[injectable]
pub struct Provider(#[import] Module);

#[module]
#[injectable]
pub struct Module {
  #[export(dyn Repository)]
  repository: repository::memory::MemoryRepository,
}
