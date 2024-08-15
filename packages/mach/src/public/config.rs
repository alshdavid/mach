use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use serde::Deserialize;

pub type MachConfigSync = Arc<MachConfig>;

#[derive(Clone, Debug, Deserialize)]
pub struct MachConfig {
  pub threads: usize,
  pub entries: Vec<PathBuf>,
  pub project_root: PathBuf,
  pub env: HashMap<String, String>,
  pub out_folder: PathBuf,
}
