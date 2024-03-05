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
  pub clean_dist_dir: bool,
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

impl Config {
  pub fn log_details(&self) {
    println!("Entry:         {}", self.entry_point.to_str().unwrap());
    println!("Root:          {}", self.project_root.to_str().unwrap());
    if !&self.machrc.is_default {
      println!("Mach Config:   {}", self.machrc.file_path.to_str().unwrap());
    } else {
      println!("Mach Config:   Default");
    }
    println!("Out Dir:       {}", self.dist_dir.to_str().unwrap());
    println!("Optimize:      {}", self.optimize);
    println!("Threads:       {}", self.threads);
    println!("Node Workers:  {}", self.node_workers);
  }

  pub fn time_elapsed(&self) -> f64 {
    self.start_time.elapsed().unwrap().as_nanos() as f64 / 1_000_000 as f64 / 1000 as f64
  }
}
