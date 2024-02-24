use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
  pub name: String,
  pub message: String
}

impl fmt::Display for Meta {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "Meta: name={}, message={}", self.name, self.message)
  }
}
