use std::any::Any;
use std::sync::Arc;

use crate::domain::generator::Generator;
use crate::domain::project_manager::ProjectManager;
use crate::infra::db_client::DbClient;
use crate::infra::db_project::ProjectRepoImpl;
use di::{
  existing, existing_as_self, injectable, Injectable, Ref, ServiceCollection, ServiceProvider,
};
use log::info;
use crate::util::rest::Rest;

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
