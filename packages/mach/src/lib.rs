// #![deny(unused_crate_dependencies)]
#![allow(warnings)]

#[cfg(feature = "cli_parser")]
pub mod cli;
pub mod cmd;
pub mod kit;
pub mod mach;
pub mod platform;
pub mod plugins;
pub mod public;
pub mod rpc;

pub use self::cmd::BuildOptions;
pub use self::cmd::BuildResult;
pub use self::cmd::DevOptions;
pub use self::cmd::DevResult;
pub use self::cmd::VersionOptions;
pub use self::cmd::VersionResult;
pub use self::cmd::WatchOptions;
pub use self::cmd::WatchResult;
pub use self::mach::*;
