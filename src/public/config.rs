use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

use serde::Serialize;

use super::Machrc;

// #[derive(Clone, Debug)]
// pub enum WorkspaceKind {
//   Pnpm,
//   NpmOrYarn,
// }

#[derive(Clone, Debug, Serialize)]
pub struct Config {
  pub start_time: SystemTime,
  pub entry_point: PathBuf,
  pub dist_dir: PathBuf,
  pub workspace_root: Option<PathBuf>,
  pub workspace_kind: Option<()>, //Option<WorkspaceKind>,
  pub project_root: PathBuf,
  pub package_json: serde_json::Value,
  pub machrc: Machrc,
  pub threads: usize,
  pub node_workers: usize,
  pub optimize: bool,
  pub env: HashMap<String, String>,
}
