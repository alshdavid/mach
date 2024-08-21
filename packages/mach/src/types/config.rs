use std::collections::HashMap;
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct MachConfig {
  pub threads: usize,
  pub entries: Vec<PathBuf>,
  pub project_root: PathBuf,
  pub env: HashMap<String, String>,
  pub out_folder: PathBuf,
}

impl Default for MachConfig {
  fn default() -> Self {
    Self {
      threads: num_cpus::get(),
      entries: Default::default(),
      project_root: Default::default(),
      env: Default::default(),
      out_folder: Default::default(),
    }
  }
}
