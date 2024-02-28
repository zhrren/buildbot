use std::sync::Mutex;

use nject::{inject, injectable};
use serde::{Deserialize, Serialize};
use crate::repository::Repository;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
  pub id: usize,
  pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUser {
  pub name: String,
}

#[inject(Self { users: Mutex::new(Vec::new()) })]
pub struct MemoryRepository {
  users: Mutex<Vec<User>>,
}

impl Repository for MemoryRepository {
  fn create(&self, user: CreateUser) -> User {
    let mut users = self.users.lock().unwrap();
    let new_user = User {
      id: users.len(),
      name: user.name,
    };
    users.push(new_user.to_owned());
    new_user
  }

  fn get(&self, user_id: usize) -> Option<User> {
    let users = self.users.lock().unwrap();
    if let Some(user) = users.get(user_id) {
      Some(user.to_owned())
    } else {
      None
    }
  }
}
//
// #[injectable]
// pub struct UserService<'a> {
//   repository: &'a dyn Repository,
// }
//
// impl<'a> UserService<'a> {
//   pub fn create(&self, user: CreateUser) -> User {
//     self.repository.create(user)
//   }
//
//   pub fn get(&self, user_id: usize) -> Option<User> {
//     self.repository.get(user_id)
//   }
// }
