use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
  pub app_port: u16,
  pub database_url: String,
}
