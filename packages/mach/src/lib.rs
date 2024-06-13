// #![deny(unused_crate_dependencies)]
#![allow(warnings)]

#[cfg(feature = "cli_parser")]
pub mod cli;
pub mod cmd;
pub mod kit;
pub mod platform;
pub mod plugins;
pub mod public;
pub mod rpc;

//
// Mach Lib API
//
pub use self::cmd::BuildOptions;
pub use self::cmd::BuildResult;
pub use self::cmd::DevOptions;
pub use self::cmd::DevResult;
pub use self::cmd::VersionOptions;
pub use self::cmd::VersionResult;
pub use self::cmd::WatchOptions;
pub use self::cmd::WatchResult;
pub use self::cmd::MachOptions;

pub struct Mach {
  options: cmd::MachOptions,
}

impl Mach {
  pub fn new(options: cmd::MachOptions) -> Self {
    Self { options }
  }

  pub fn build(
    &self,
    options: cmd::BuildOptions,
  ) -> anyhow::Result<cmd::BuildResult> {
    cmd::build(self.options.clone(), options)
  }

  pub fn watch(
    &self,
    options: cmd::WatchOptions,
  ) -> Result<cmd::WatchResult, String> {
    cmd::watch(options)
  }

  pub fn dev(
    &self,
    options: cmd::DevOptions,
  ) -> Result<cmd::DevResult, String> {
    cmd::dev(options)
  }

  pub fn version(
    &self,
    options: cmd::VersionOptions,
  ) -> cmd::VersionResult {
    cmd::version(options)
  }
}
