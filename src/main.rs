#[macro_use]
extern crate lazy_static;

use std::any::{Any, type_name};
use std::rc::Rc;
use std::sync::Arc;
use chrono::{Local, Utc};

use di::{injectable, Injectable, Ref, ServiceCollection, ServiceProvider};
use log::{error, info, warn};
use log4rs;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ConnectionTrait};

use crate::config::inject_config::{get_it};
use crate::domain::entity::project;
use crate::domain::project_manager::ProjectManager;
use crate::infra::db::create_pool;

mod kernel;
mod rest;
mod web;
mod infra;
mod config;
mod domain;
mod util;

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();
  log4rs::init_file("buildbot.yaml", Default::default()).unwrap();

  let db = create_pool().await;
  project::ActiveModel {
    id: Set(1),
    app_name: Set("app_name".to_string()),
    build_number: Set(1),
    created: Set(Utc::now().to_rfc3339()),
    updated: Set(Utc::now().to_rfc3339()),
    ..Default::default()
  }
    .save(&db)
    .await
    .expect("could not insert post");

  let project_manager = ProjectManager{};
  let result = project_manager.create_project("app_name".to_string(), 1).await;
  info!("result: {:?}", result.ok());
  info!("application starting...");

  web::serve().await;
}

