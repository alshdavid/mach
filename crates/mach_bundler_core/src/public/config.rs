use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;

use super::Machrc;

// #[derive(Clone, Debug)]
// pub enum WorkspaceKind {
//   Pnpm,
//   NpmOrYarn,
// }

pub type MachConfigSync = Arc<MachConfig>;

#[derive(Clone, Debug)]
pub struct MachConfig {
  pub start_time: SystemTime,
  pub entries: Vec<PathBuf>,
  pub dist_dir: PathBuf,
  pub clean_dist_dir: bool,
  pub project_root: PathBuf,
  pub machrc: Machrc,
  pub threads: usize,
  pub node_workers: usize,
  pub optimize: bool,
  pub bundle_splitting: bool,
  pub env: HashMap<String, String>,
}

impl MachConfig {
  pub fn time_elapsed(&self) -> f64 {
    self.start_time.elapsed().unwrap().as_nanos() as f64 / 1_000_000 as f64 / 1000 as f64
  }
}
