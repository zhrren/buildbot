use std::ops::Deref;

use crate::domain::project::ProjectRepo;

pub mod auth;
pub mod project;


// lazy_static! {
//   pub static ref GET_IT: &'static Provider = {
//     let db_pool: SqlitePool = tokio::runtime::Runtime::new()
//       .unwrap()
//       .block_on(async { create_pool().await });
//
//     let provider = Provider{  };
//     // let pool = provider.provide::<SqlitePool>();
//     // println!("{}", pool.size());
//
//     let get_it: &'static Provider = Box::leak(Box::new(provider));
//     return get_it;
//   };
// }
