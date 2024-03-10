#[macro_use]
extern crate lazy_static;

use chrono::{Local, Utc};
use std::any::{type_name, Any};
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;
use std::vec;

use di::{injectable, Injectable, Ref, ServiceCollection, ServiceProvider};
use log::{error, info, warn};
use log4rs;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ConnectionTrait, DatabaseConnection};

use crate::config::inject_config::{DI, get_it};
use crate::domain::entity::project;
use crate::domain::generator::Generator;
use crate::domain::project_manager::{ProjectManager, ProjectRepo};
use crate::infra::db_client::DbClient;
use crate::rest::{auth_rest, main_rest};
use crate::util::build_util::{BuildUtil};
use crate::util::rest::Rest;

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
