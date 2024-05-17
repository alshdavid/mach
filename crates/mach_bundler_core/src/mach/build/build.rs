use std::path::PathBuf;

use super::build_parse_config::parse_config;
use crate::platform::adapters::nodejs::NodejsAdapter;
use crate::platform::adapters::nodejs::NodejsAdapterOptions;
use crate::platform::bundling::bundle;
use crate::platform::config::load_plugins;
use crate::platform::emit::emit;
use crate::platform::packaging::package;
use crate::platform::transformation::resolve_and_transform;
use crate::public::AssetGraphSync;
use crate::public::AssetMapSync;
use crate::public::BundleGraphSync;
use crate::public::BundleManifestSync;
use crate::public::BundleMapSync;
use crate::public::DependencyMapSync;
use crate::public::OutputsSync;

#[derive(Debug)]
pub struct BuildOptions {
  /// Input file to build
  pub entries: Option<Vec<PathBuf>>,
  /// Output folder
  pub out_folder: PathBuf,
  /// Delete output folder before emitting files
  pub clean: bool,
  /// Disable optimizations
  pub optimize: bool,
  /// Enable bundle splitting (experimental)
  pub bundle_splitting: bool,
  /// How many threads to use for compilation
  pub threads: Option<usize>,
  /// How many Node.js workers to spawn for plugins
  pub node_workers: Option<usize>,
}

pub struct BuildResult {}

pub fn build(
  options: BuildOptions,
) -> Result<BuildResult, String> {
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
  let nodejs_adapter = NodejsAdapter::new(NodejsAdapterOptions {
    workers: config.node_workers.clone() as u8,
  })?;

  /*
    load_plugins() will read source the .machrc and will
    fetch then initialize the referenced plugins
  */
  let plugins = load_plugins(&config, &config.machrc, nodejs_adapter.clone())?;

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

  /*
    emit() writes the contents of the bundles to disk
  */
  emit(config.clone(), outputs)?;

  return Ok(BuildResult {});
}
