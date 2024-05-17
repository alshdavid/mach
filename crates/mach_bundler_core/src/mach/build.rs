use std::path::PathBuf;

use super::mach::Mach;

#[cfg(feature = "cli_parser")]
use clap::Parser;

#[cfg_attr(feature = "cli_parser", derive(Parser))]
#[derive(Debug)]
pub struct BuildOptions {
  /// Input file to build
  pub entries: Option<Vec<PathBuf>>,

  /// Output folder
  #[cfg_attr(feature = "cli_parser", arg(short = 'o', long = "dist", default_value = "dist"))]
  pub out_folder: PathBuf,

  /// Delete output folder before emitting files
  #[cfg_attr(feature = "cli_parser", arg(short = 'c', long = "clean"))]
  pub clean: bool,

  /// Disable optimizations
  #[cfg_attr(feature = "cli_parser", arg(long = "no-optimize"))]
  pub no_optimize: bool,

  /// Enable bundle splitting (experimental)
  #[cfg_attr(feature = "cli_parser", arg(long = "bundle-splitting"))]
  pub bundle_splitting: bool,

  /// How many threads to use for compilation
  #[cfg_attr(feature = "cli_parser", arg(short = 't', long = "threads", env = "MACH_THREADS"))]
  pub threads: Option<usize>,

  /// How many Node.js workers to spawn for plugins
  #[cfg_attr(feature = "cli_parser", arg(long = "node-workers", env = "MACH_NODE_WORKERS"))]
  pub node_workers: Option<usize>,

  /// Enable logging debug info
  #[cfg_attr(feature = "cli_parser", arg(long = "debug"))]
  pub debug: bool,
}

pub struct BuildResult {

}

impl Mach {
  pub fn build(&self, options: BuildOptions) -> Result<BuildResult, String> {
    todo!();
  }
}
