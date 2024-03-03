#[macro_use]
extern crate lazy_static;

use std::any::{Any, type_name};
use std::rc::Rc;
use std::sync::Arc;

use di::{injectable, Injectable, Ref, ServiceCollection, ServiceProvider};
use log::{error, info, warn};
use log4rs;

use crate::config::inject_config::{get_it, Person, Phrase};

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

  info!("application starting...");
  web::serve().await;
}

