#[macro_use]
extern crate lazy_static;

use std::any::Any;
use std::ops::Deref;

use di::Injectable;
use log4rs;
use sea_orm::{ActiveModelTrait, ConnectionTrait};

use crate::config::inject_config::get_it;
use crate::domain::generator::Generator;
use crate::domain::project_manager::ProjectRepo;
use crate::infra::db_client::DbClient;
use crate::rest::{auth_rest, main_rest};

mod config;
mod domain;
mod infra;
mod kernel;
mod rest;
mod util;
mod web;

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();
  log4rs::init_file("buildbot.yaml", Default::default()).unwrap();

  get_it::<Generator>().configure();
  get_it::<DbClient>().configure().await;

  let routes = [auth_rest::routes(), main_rest::routes()].concat();
  web::serve(routes).await;
}
