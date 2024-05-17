use std::path::PathBuf;

use crate::platform::adapters::nodejs::NodejsAdapter;
use crate::platform::adapters::nodejs::NodejsAdapterOptions;
use crate::platform::bundling::bundle;
use crate::platform::config::load_plugins;
use crate::platform::emit::emit;
use crate::platform::packaging::package;
use crate::platform::transformation::resolve_and_transform;
use crate::public::programmatic::ProgrammaticReporter;
use crate::public::AssetGraphSync;
use crate::public::AssetMapSync;
use crate::public::BundleGraphSync;
use crate::public::BundleManifestSync;
use crate::public::BundleMapSync;
use crate::public::DependencyMapSync;
use crate::public::OutputsSync;
use super::build_app_reporter::AppReporter;
use super::build_parse_config::parse_config;
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
    let config = parse_config(options)?;

    /*
      This is the bundler state. It is passed into
      the bundling phases with read or write permissions
      depending on how that phase uses them
    */
    let asset_map = AssetMapSync::default();
    let dependency_map = DependencyMapSync::default();
    let asset_graph = AssetGraphSync::default();
    let bundles = BundleMapSync::default();
    let bundle_graph = BundleGraphSync::default();
    let bundle_manifest = BundleManifestSync::default();
    let outputs = OutputsSync::default();
    let programmatic_reporter = ProgrammaticReporter::new(
      config.clone(),
      asset_map.clone(),
      bundles.clone(),
      bundle_manifest.clone(),
    );
    let nodejs_adapter = NodejsAdapter::new(NodejsAdapterOptions {
      workers: config.node_workers.clone() as u8,
    })?;

    let mut reporter = AppReporter::new(
      config.clone(),
      dependency_map.clone(),
      asset_map.clone(),
      asset_graph.clone(),
      bundles.clone(),
      bundle_graph.clone(),
      outputs.clone(),
      nodejs_adapter.clone(),
    );

    reporter.print_config();

    /*
      load_plugins() will read source the .machrc and will
      fetch then initialize the referenced plugins
    */
    let plugins = load_plugins(&config, &config.machrc, nodejs_adapter.clone())?;

    reporter.print_init_stats();

    /*
      resolve_and_transform() will read source files, identify import statements
      before modifying their contents (like removing TypeScript types).

      This will loop until there are no more import statements to resolve
    */
    resolve_and_transform(
      config.clone(),
      plugins.clone(),
      asset_map.clone(),
      asset_graph.clone(),
      dependency_map.clone(),
    )?;

    reporter.print_transform_stats();

    /*
      bundle() will take the asset graph and organize related assets
      into groupings. Each grouping will be emitted as a "bundle"
    */
    bundle(
      config.clone(),
      asset_map.clone(),
      asset_graph.clone(),
      dependency_map.clone(),
      bundles.clone(),
      bundle_graph.clone(),
    )?;

    reporter.print_bundle_stats();

    /*
      package() will take the bundles, obtain their referenced Assets
      and modify them such that they can work in the context of an
      emitted file.

      It also injects the runtime and rewrites import
      statements to point to the new paths
    */
    package(
      config.clone(),
      asset_map.clone(),
      asset_graph.clone(),
      dependency_map.clone(),
      bundles.clone(),
      bundle_graph.clone(),
      bundle_manifest.clone(),
      outputs.clone(),
    )?;

    reporter.print_package_stats();

    /*
      emit() writes the contents of the bundles to disk
    */
    emit(config.clone(), outputs)?;

    reporter.print_emit_stats();
    reporter.print_finished_stats();
    programmatic_reporter.emit_build_report();
    
    return Ok(BuildResult{});
  }
}
