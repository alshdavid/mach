use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow;
use serde::Deserialize;
use serde::Serialize;

use super::build_parse_config::parse_config;
// use super::create_result::create_build_result;
// use crate::platform::bundling::bundle;
use crate::platform::config::load_plugins;
use crate::platform::transformation::build_asset_graph;
// use crate::platform::emit::emit;
// use crate::platform::packaging::package;
use crate::public::RpcHost;
// use crate::public::AssetGraphSync;
// use crate::public::AssetMap;
// use crate::public::AssetMapSync;
// use crate::public::BundleGraphSync;
// use crate::public::BundleManifestSync;
// use crate::public::BundleMapSync;
use crate::public::Compilation;
// use crate::public::DependencyMapSync;
use crate::public::Engine;
use crate::mach::MachOptions;
// use crate::public::OutputsSync;

#[derive(Debug)]
pub struct BuildOptions {
  /// Input file to build
  pub entries: Vec<String>,
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
}

impl Default for BuildOptions {
  fn default() -> Self {
    Self {
      entries: vec![],
      out_folder: PathBuf::from("dist"),
      clean: false,
      optimize: true,
      bundle_splitting: false,
      project_root: None,
    }
  }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BuildResult {
  pub bundle_manifest: HashMap<String, String>,
  pub entries: HashMap<String, String>,
}

pub fn build(mach_options: MachOptions, options: BuildOptions) -> anyhow::Result<BuildResult> {
  let nodejs = mach_options.rpc_hosts.get("nodejs").unwrap().clone();

  nodejs.init()?;

  return Ok(BuildResult::default());
//   let config = parse_config(&options).map_err(|e| anyhow::anyhow!(e))?;

//   /*
//     This is the bundler state. It is passed into
//     the bundling phases with read or write permissions
//     depending on how that phase uses them
//   */
//   let mut compilation = Compilation::new();
//   let adapter_map = options.rpc_hosts;

//   /*
//     load_plugins() will read source the .machrc and will
//     fetch then initialize the referenced plugins
//   */
//   let plugins = load_plugins(&config, &config.machrc, &adapter_map)?;

//   /*
//     resolve_and_transform() build the AssetGraph.

//     It does this by crawling the source files, identify import statements, modifying their contents
//     (like removing TypeScript types) and looping until there are no more import statements to resolve.
//   */
//   build_asset_graph(config.clone(), plugins.clone(), &mut compilation).map_err(|e| anyhow::anyhow!(e));

//   Ok(BuildResult::default())

//   /*

//   /*
//     bundle() will take the asset graph and organize related assets
//     into groupings. Each grouping will be emitted as a "bundle"
//   */
//   bundle(
//     config.clone(),
//     asset_map.clone(),
//     asset_graph.clone(),
//     dependency_map.clone(),
//     bundles.clone(),
//     bundle_graph.clone(),
//   )?;

//   /*
//     package() will take the bundles, obtain their referenced Assets
//     and modify them such that they can work in the context of an
//     emitted file.

//     It also injects the runtime and rewrites import
//     statements to point to the new paths
//   */
//   package(
//     config.clone(),
//     asset_map.clone(),
//     asset_graph.clone(),
//     dependency_map.clone(),
//     bundles.clone(),
//     bundle_graph.clone(),
//     bundle_manifest.clone(),
//     outputs.clone(),
//   )?;

//   /*
//     emit() writes the contents of the bundles to disk
//   */
//   emit(config.clone(), outputs)?;

//   return Ok(create_build_result(asset_map, bundles, bundle_manifest));
//   */
}
