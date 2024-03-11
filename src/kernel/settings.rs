use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
  pub app_port: u16,
  pub database_url: String,
}

lazy_static! {
  pub static ref SETTINGS: Settings = {
    pub fn get_str_default(key: &str, default: &str) -> String {
      return env::var(key).unwrap_or(default.to_string());
    }
    pub fn get_str(key: &str) -> String {
      return env::var(key).expect(&format!("缺少环境变量: {}", key));
    }
    Settings {
      app_port: get_str_default("app_port", "10940").parse().unwrap(),
      database_url: get_str("DATABASE_URL"),
    }
  };
}
