use std::collections::HashMap;
use std::path::PathBuf;

use super::Machrc;

// #[derive(Clone, Debug)]
// pub enum WorkspaceKind {
//   Pnpm,
//   NpmOrYarn,
// }

#[derive(Clone, Debug)]
pub struct Config {
  pub entry_point: PathBuf,
  pub dist_dir: PathBuf,
  pub workspace_root: Option<PathBuf>,
  pub workspace_kind: Option<()>, //Option<WorkspaceKind>,
  pub project_root: PathBuf,
  pub package_json: serde_json::Value,
  pub mach_config: Option<Machrc>,
  pub threads: usize,
  pub node_workers: usize,
  pub optimize: bool,
  pub env: HashMap<String, String>,
}
