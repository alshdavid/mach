use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;

use super::build_parse_config::parse_config;
// use super::create_result::create_build_result;
// use crate::platform::bundling::bundle;
use crate::platform::config::load_plugins;
// use crate::platform::emit::emit;
// use crate::platform::packaging::package;
use crate::platform::transformation::resolve_and_transform;
use crate::public::Adapter;
use crate::public::AssetGraphSync;
use crate::public::AssetMap;
// use crate::public::AssetMapSync;
use crate::public::BundleGraphSync;
use crate::public::BundleManifestSync;
use crate::public::BundleMapSync;
use crate::public::Compilation;
use crate::public::DependencyMapSync;
use crate::public::Engine;
use crate::public::OutputsSync;

#[derive(Debug)]
pub struct BuildOptions {
  /// Input file to build
  pub entries: Option<Vec<String>>,
  /// Output folder
  pub out_folder: PathBuf,
  /// Root directory of project
  pub project_root: Option<PathBuf>,
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
  /// Map of adapters used to communicate with plugin contexts
  pub adapter_map: Option<HashMap<Engine, Arc<dyn Adapter>>>,
}

impl Default for BuildOptions {
  fn default() -> Self {
    Self {
      entries: None,
      out_folder: PathBuf::from("dist"),
      clean: false,
      optimize: true,
      bundle_splitting: false,
      project_root: None,
      threads: Some(num_cpus::get_physical()),
      node_workers: Some(num_cpus::get_physical()),
      adapter_map: None,
    }
  }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BuildResult {
  pub bundle_manifest: HashMap<String, String>,
  pub entries: HashMap<String, String>,
}

pub fn build(options: BuildOptions) -> Result<BuildResult, String> {
  let config = parse_config(&options)?;

  /*
    This is the bundler state. It is passed into
    the bundling phases with read or write permissions
    depending on how that phase uses them
  */
  let compilation = Compilation::new();
  let adapter_map = options.adapter_map.unwrap_or_default();

  /*
    load_plugins() will read source the .machrc and will
    fetch then initialize the referenced plugins
  */
  let plugins = load_plugins(&config, &config.machrc, &adapter_map)?;

  /*
    resolve_and_transform() build the AssetGraph.

    It does this by crawling the source files, identify import statements, modifying their contents
    (like removing TypeScript types) and looping until there are no more import statements to resolve.
  */
  resolve_and_transform(config.clone(), plugins.clone(), compilation)?;

  Ok(BuildResult::default())

  /*

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

  return Ok(create_build_result(asset_map, bundles, bundle_manifest));
  */
}
