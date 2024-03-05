use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
  pub name: String,
  pub message: String,
}

impl fmt::Display for Meta {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Meta: name={}, message={}", self.name, self.message)
  }
}

impl Error for Meta {
}

#[macro_export]
macro_rules! meta {
    ($name: expr) => {
        Meta { name: $name.to_string(), message: "".to_string() }.into()
    };
    ($name: expr, $message: expr) => {
        Meta { name: $name.to_string(), message: $message.to_string() }.into()
    };
}
