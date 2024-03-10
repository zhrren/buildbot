use std::ops::Deref;

pub mod auth_rest;
pub mod main_rest;

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
