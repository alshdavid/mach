use std::path::PathBuf;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Input file to build
    pub entry: Vec<PathBuf>,

    /// Output folder
    #[arg(short = 'o', long = "dist",  default_value = "dist")]
    pub out_folder: Option<PathBuf>,

    /// Enable optimization
    #[arg(short = 'z', long = "optimize", env = "MACH_OPTIMIZE", default_value = "true")]
    pub optimize: Option<bool>,

    /// How many threads to use for compilation
    #[arg(short = 't', long = "threads", env = "MACH_THREADS")]
    pub threads: Option<usize>,

    /// How many Node.js instances to spawn for plugins
    #[arg(long = "node-workers", env = "MACH_NODE_WORKERS", default_value = "4")]
    pub node_workers: Option<usize>,
}
