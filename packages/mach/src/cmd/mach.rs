use std::collections::HashMap;
use std::path::PathBuf;

use crate::types::Machrc;
use crate::rpc::RpcHosts;

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

pub struct Mach {
  options: MachOptions,
}

impl Mach {
  pub fn new(options: MachOptions) -> Self {
    Self { options }
  }

  pub fn build(
    &self,
    options: super::BuildOptions,
  ) -> anyhow::Result<super::BuildResult> {
    super::build(self.options.clone(), options)
  }

  pub fn watch(
    &self,
    options: super::WatchOptions,
  ) -> Result<super::WatchResult, String> {
    super::watch(options)
  }

  pub fn dev(
    &self,
    options: super::DevOptions,
  ) -> Result<super::DevResult, String> {
    super::dev(options)
  }

  pub fn version(
    &self,
    options: super::VersionOptions,
  ) -> super::VersionResult {
    super::version(options)
  }
}
