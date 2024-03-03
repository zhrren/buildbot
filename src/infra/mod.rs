use std::env;
use async_once::AsyncOnce;
use crate::infra::db::create_pool;

pub mod db;
pub mod db_project;

// lazy_static! {
//   pub static ref DB_POOL: AsyncOnce<Pool<Sqlite>> = AsyncOnce::new( {
//     create_pool()
//   });
// }
