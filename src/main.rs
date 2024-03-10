#[macro_use]
extern crate lazy_static;

use std::any::{Any, type_name};
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;
use chrono::{Local, Utc};

use di::{injectable, Injectable, Ref, ServiceCollection, ServiceProvider};
use log::{error, info, warn};
use log4rs;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ConnectionTrait, DatabaseConnection};

use crate::config::inject_config::{get_it};
use crate::domain::entity::project;
use crate::domain::project::ProjectManager;
use crate::infra::db::DbClient;

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

  // let db_client = DbClient::new();
  // let conn = db_client.get_connection().await;
  //
  // project::ActiveModel {
  //   id: Set(1),
  //   app_name: Set("app_name".to_string()),
  //   build_number: Set(1),
  //   created: Set(Utc::now().to_rfc3339()),
  //   updated: Set(Utc::now().to_rfc3339()),
  //   ..Default::default()
  // }
  //   .update(conn.deref())
  //   .await
  //   .expect("could not insert post");

  // let project_manager = ProjectManager{};
  // let result = project_manager.create_project("app_name".to_string(), 1).await;
  // info!("result: {:?}", result.ok());
  // info!("application starting...");

  let project_manager = get_it::<ProjectManager>();
  project_manager.create_project("app_name".to_string(), 1).await;

  let db_client = get_it::<DbClient>();
  let conn = db_client.get_connection().await;
  project::ActiveModel {
    id: Set(1),
    app_name: Set("app_name".to_string()),
    build_number: Set(1),
    created: Set(Utc::now().to_rfc3339()),
    updated: Set(Utc::now().to_rfc3339()),
    ..Default::default()
  }
    .save(conn.as_ref())
    .await
    .expect("could not insert post");


  web::serve().await;
}
