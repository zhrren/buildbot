use std::env;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub app_port: u16,
    pub database_url: String,
}

fn get_str_default(key: &str, default: &str) -> String {
    return env::var(key).unwrap_or(default.to_string());
}

fn get_str(key: &str) -> String {
    return env::var(key).expect(&format!("缺少环境变量: {}", key));
}

lazy_static! {
    pub static ref SETTINGS: Settings = {
        Settings {
            app_port: get_str_default("app_port", "10243").parse().unwrap(),
            database_url: get_str("DATABASE_URL"),
        }
    };
}
