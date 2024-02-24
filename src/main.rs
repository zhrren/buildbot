#[macro_use]
extern crate lazy_static;

use log::{error, info, warn};
use log4rs;

mod domain;
mod kernel;
mod rest;
mod web;
mod infra;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    log4rs::init_file("buildbot.yaml", Default::default()).unwrap();

    info!("application starting...");
    web::serve().await;
}
