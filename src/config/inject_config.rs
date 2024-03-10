use std::any::Any;
use std::sync::Arc;

use di::{existing, existing_as_self, injectable, Injectable, Ref, ServiceCollection, ServiceProvider};
use crate::domain::project::ProjectManager;
use crate::infra::db::DbClient;
use crate::infra::db_project::ProjectRepoImpl;

lazy_static! {
  static ref DI: ServiceProvider = {
    // let db = create_pool().await;

    let provider = ServiceCollection::new()
    // .add(existing::<dyn Phrase,EnglishPhase>(Box::new(EnglishPhase)))
    // .add(Person::transient())
    // .add(ProjectRepoImpl::singleton())
    .add(existing_as_self(DbClient::new()))
    .add(ProjectRepoImpl::singleton())
    .add(ProjectManager::singleton())
    .build_provider()
    .unwrap();
    return provider;
  };
}

pub fn get_it<T: Any + ?Sized>() -> Ref<T> {
  return DI.get_required::<T>();
}
