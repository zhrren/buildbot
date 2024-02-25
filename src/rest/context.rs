use serde::{Deserialize, Serialize};
use crate::domain::project::ProjectManager;

#[derive(Debug, Clone)]
pub struct Context {
  pub project_manager: &'static ProjectManager<'static>,
}

