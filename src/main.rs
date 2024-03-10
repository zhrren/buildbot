#[macro_use]
extern crate lazy_static;

use chrono::{Local, Utc};
use std::any::{type_name, Any};
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

use di::{injectable, Injectable, Ref, ServiceCollection, ServiceProvider};
use log::{error, info, warn};
use log4rs;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ConnectionTrait, DatabaseConnection};

use crate::config::inject_config::get_it;
use crate::domain::entity::project;
use crate::domain::generator::Generator;
use crate::domain::project::{ProjectManager, ProjectRepo};
use crate::infra::db::DbClient;

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

  web::serve().await;
}
