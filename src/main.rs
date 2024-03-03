#[macro_use]
extern crate lazy_static;

use std::any::{Any, type_name};
use std::rc::Rc;
use std::sync::Arc;

use di::{injectable, Injectable, Ref, ServiceCollection, ServiceProvider};
use log::{error, info, warn};
use log4rs;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ConnectionTrait};

use crate::config::inject_config::{get_it, Person, Phrase};
use crate::domain::post::post;
use crate::infra::db::create_pool;

mod domain;
mod kernel;
mod rest;
mod web;
mod infra;
mod config;

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();
  log4rs::init_file("buildbot.yaml", Default::default()).unwrap();

  let db = create_pool().await;
  post::ActiveModel {
    title: Set("title".to_string()),
    text: Set("text".to_string()),
    ..Default::default()
  }
    .save(&db)
    .await
    .expect("could not insert post");

  info!("application starting...");
  web::serve().await;
}

