use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub enum WorkspaceKind {
  Pnpm,
  NpmOrYarn,
  None,
}

#[derive(Clone, Debug)]
pub struct Config {
  pub entry_point: PathBuf,
  pub dist_dir: PathBuf,
  pub workspace_root: Option<PathBuf>,
  pub workspace_kind: WorkspaceKind,
  pub project_root: PathBuf,
  pub threads: usize,
  pub optimize: bool,
  pub env: HashMap<String, String>,
}
