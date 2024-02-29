use std::any::Any;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use async_once::AsyncOnce;
use derivative::Derivative;

use di::{existing, existing_as_self, Injectable, injectable, Ref, ServiceCollection, ServiceProvider};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use sqlx::{Executor, Pool, Sqlite, SqlitePool};
use crate::infra::db_project::ProjectRepoImpl;
use crate::kernel::SETTINGS;

lazy_static! {
  static ref DI: ServiceProvider = {
    let provider = ServiceCollection::new()
    .add(existing::<dyn Phrase,EnglishPhase>(Box::new(EnglishPhase)))
    .add(Person::transient())
    // .add(ProjectRepoImpl::singleton())
    .build_provider()
    .unwrap();
    return provider;
  };
}

pub fn get_it<T: Any + ?Sized>() -> Ref<T> {
  return DI.get_required::<T>()
}

pub trait Phrase {
  fn salutation(&self) -> &str;
}

#[injectable(Phrase)]
pub struct EnglishPhase;

impl Phrase for EnglishPhase {
  fn salutation(&self) -> &str {
    "Hello world!"
  }
}

#[injectable]
pub struct Person {
  pub phase: Arc<dyn Phrase>,
}

impl Person {
  pub fn speak(&self) -> &str {
    self.phase.salutation()
  }
}
