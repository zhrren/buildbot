#[macro_use]
extern crate lazy_static;

use log::{error, info, warn};
use log4rs;
use nject::{injectable, module, provider};

use crate::repository::memory::MemoryRepository;
use crate::repository::Repository;

mod domain;
mod kernel;
mod rest;
mod web;
mod infra;
mod repository;

#[tokio::main]
async fn main() {

  let provider = Provider2 { shared: DepOne  { count: 1 } };
  let facade1: Facade = provider.provide();
  let facade2: Facade = provider.provide();

  // let one1 = MyStruct;
  // let one2 = MyStruct;
  println!("{}, {}", &facade1.dep.count, &facade1.dep.count);

  dotenv::dotenv().ok();
  log4rs::init_file("buildbot.yaml", Default::default()).unwrap();

  info!("application starting...");
  web::serve().await;
}


struct MyStruct;

#[injectable]
struct DepOne {
  count: u32
}

#[injectable]
struct DepTwo<'a> {
  dep_two: &'a DepOne
}


#[injectable]
struct Facade<'a> {
  dep: &'a DepOne,
}

#[provider]
struct Provider2 {
  #[provide]
  shared: DepOne,
}
