use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct BuildCommand {
  /// Input file to build
  pub entries: Option<Vec<String>>,

  /// Output folder
  #[arg(short = 'o', long = "dist", default_value = "dist")]
  pub out_folder: PathBuf,

  /// Output folder
  #[arg(long = "project-root")]
  pub project_root: Option<PathBuf>,

  /// Delete output folder before emitting files
  #[arg(short = 'c', long = "clean")]
  pub clean: bool,

  /// Disable optimizations
  #[arg(long = "no-optimize")]
  pub no_optimize: bool,

  /// Enable bundle splitting (experimental)
  #[arg(long = "bundle-splitting")]
  pub bundle_splitting: bool,

  /// How many threads to use for compilation
  #[arg(short = 't', long = "threads", env = "MACH_THREADS")]
  pub threads: Option<usize>,

  /// How many Node.js workers to spawn for plugins
  #[arg(long = "node-workers", env = "MACH_NODE_WORKERS")]
  pub node_workers: Option<usize>,

  /// Enable logging debug info
  #[arg(long = "debug")]
  pub debug: bool,
}
