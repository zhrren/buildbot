use std::any::Any;

use di::{
  existing_as_self, Injectable, Ref, ServiceCollection, ServiceProvider,
};

use crate::domain::generator::Generator;
use crate::domain::project_manager::ProjectManager;
use crate::infra::db_client::DbClient;
use crate::infra::db_project::ProjectRepoImpl;

lazy_static! {
  pub static ref DI: ServiceProvider = {
    let provider = ServiceCollection::new()
    // .add(existing::<dyn Phrase,EnglishPhase>(Box::new(EnglishPhase)))
    .add(Generator::singleton())
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
