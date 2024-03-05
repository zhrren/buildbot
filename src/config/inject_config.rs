use std::any::Any;
use std::sync::Arc;

use di::{existing, injectable, Injectable, Ref, ServiceCollection, ServiceProvider};

lazy_static! {
  static ref DI: ServiceProvider = {
    let provider = ServiceCollection::new()
    // .add(existing::<dyn Phrase,EnglishPhase>(Box::new(EnglishPhase)))
    // .add(Person::transient())
    // .add(ProjectRepoImpl::singleton())
    .build_provider()
    .unwrap();
    return provider;
  };
}

pub fn get_it<T: Any + ?Sized>() -> Ref<T> {
  return DI.get_required::<T>();
}
