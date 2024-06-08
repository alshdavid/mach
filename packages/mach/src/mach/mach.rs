use super::build;
use super::dev;
use super::version;
use super::watch;
use crate::BuildOptions;
use crate::BuildResult;
use crate::DevOptions;
use crate::DevResult;
use crate::VersionOptions;
use crate::VersionResult;
use crate::WatchOptions;
use crate::WatchResult;

pub struct Mach {}

impl Mach {
  pub fn new() -> Self {
    Self {}
  }

  pub fn build(
    &self,
    options: BuildOptions,
  ) -> Result<BuildResult, String> {
    build::build(options)
  }

  pub fn watch(
    &self,
    options: WatchOptions,
  ) -> Result<WatchResult, String> {
    watch::watch(options)
  }

  pub fn dev(
    &self,
    options: DevOptions,
  ) -> Result<DevResult, String> {
    dev::dev(options)
  }

  pub fn version(
    &self,
    options: VersionOptions,
  ) -> VersionResult {
    version::version(options)
  }
}
