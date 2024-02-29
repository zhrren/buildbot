#[macro_use]
extern crate lazy_static;

use std::rc::Rc;
use std::sync::Arc;

use di::{injectable, Injectable, ServiceCollection};
use log::{error, info, warn};
use log4rs;

mod domain;
mod kernel;
mod rest;
mod web;
mod infra;

#[tokio::main]
async fn main() {

  // let provider = Provider2 { shared: DepOne  { count: 1 } };
  // let facade1: Facade = provider.provide();
  // let facade2: Facade = provider.provide();

  // let one1 = MyStruct;
  // let one2 = MyStruct;
  // println!("{}, {}", &facade1.dep.count, &facade1.dep.count);

  let provider = ServiceCollection::new()
    .add(EnglishPhase::singleton())
    .add(Person::transient())
    .build_provider()
    .unwrap();

  let person = provider.get_required::<Person>();

  println!("{}", person.speak());

  dotenv::dotenv().ok();
  log4rs::init_file("buildbot.yaml", Default::default()).unwrap();

  info!("application starting...");
  web::serve().await;
}

//
// struct MyStruct;
//
// #[injectable]
// struct DepOne {
//   count: u32
// }
//
// #[injectable]
// struct DepTwo<'a> {
//   dep_two: &'a DepOne
// }
//
//
// #[injectable]
// struct Facade<'a> {
//   dep: &'a DepOne,
// }
//
// #[provider]
// struct Provider2 {
//   #[provide]
//   shared: DepOne,
// }
//
//

trait Phrase {
  fn salutation(&self) -> &str;
}

#[injectable(Phrase)]
struct EnglishPhase;

impl Phrase for EnglishPhase {
  fn salutation(&self) -> &str {
    "Hello world!"
  }
}

#[injectable]
struct Person {
  phase: Rc<dyn Phrase>,
}

impl Person {
  fn speak(&self) -> &str {
    self.phase.salutation()
  }
}
