use std::{collections::HashMap, path::PathBuf};

use crate::{public::Machrc, rpc::RpcHosts};

#[derive(Clone, Debug)]
pub struct MachOptions {
  pub rpc_hosts: RpcHosts,
  pub threads: usize,
  pub entries: Vec<PathBuf>,
  pub config: Machrc,
  pub env: HashMap<String, String>,
  pub out_folder: PathBuf,
  pub project_root: PathBuf,
}

impl Default for MachOptions {
  fn default() -> Self {
    Self {
      rpc_hosts: Default::default(),
      threads: num_cpus::get(),
      entries: Default::default(),
      config: Default::default(),
      env: Default::default(),
      out_folder: Default::default(),
      project_root: Default::default(),      
    }
  }
}
