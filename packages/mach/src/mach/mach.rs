/*
  This is a nice wrapper for Mach's public API
*/

use anyhow;

use crate::cmd::build;
use crate::cmd::dev;
use crate::cmd::version;
use crate::cmd::watch;
use crate::cmd::BuildOptions;
use crate::cmd::BuildResult;
use crate::cmd::DevOptions;
use crate::cmd::DevResult;
use crate::cmd::VersionOptions;
use crate::cmd::VersionResult;
use crate::cmd::WatchOptions;
use crate::cmd::WatchResult;

pub struct Mach {}

impl Mach {
  pub fn new() -> Self {
    Self {}
  }

  pub fn build(
    &self,
    options: BuildOptions,
  ) -> anyhow::Result<BuildResult> {
    build(options)
  }

  pub fn watch(
    &self,
    options: WatchOptions,
  ) -> Result<WatchResult, String> {
    watch(options)
  }

  pub fn dev(
    &self,
    options: DevOptions,
  ) -> Result<DevResult, String> {
    dev(options)
  }

  pub fn version(
    &self,
    options: VersionOptions,
  ) -> VersionResult {
    version(options)
  }
}
