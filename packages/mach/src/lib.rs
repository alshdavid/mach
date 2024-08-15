// #![deny(unused_crate_dependencies)]
#[cfg(feature = "cli_parser")]
pub mod cli;
pub mod cmd;
pub mod core;
pub mod kit;
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
pub use self::cmd::Mach;
pub use self::cmd::MachOptions;
pub use self::cmd::VersionOptions;
pub use self::cmd::VersionResult;
pub use self::cmd::WatchOptions;
pub use self::cmd::WatchResult;
