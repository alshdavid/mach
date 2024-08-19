use std::collections::HashMap;

use anyhow;
use serde::Deserialize;
use serde::Serialize;

use super::super::MachOptions;
use crate::core::plugins::load_plugins;
use crate::core::resolve_and_transform::resolve_and_transform;
use crate::types::Compilation;
use crate::types::MachConfig;
use crate::core::bundling::bundle;

#[derive(Debug)]
pub struct BuildOptions {
  /// Delete output folder before emitting files
  pub clean: bool,
  /// Disable optimizations
  pub optimize: bool,
}

impl Default for BuildOptions {
  fn default() -> Self {
    Self {
      clean: false,
      optimize: true,
    }
  }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BuildResult {
  pub bundle_manifest: HashMap<String, String>,
  pub entries: HashMap<String, String>,
}

pub fn build(
  mach_options: MachOptions,
  _build_options: BuildOptions,
) -> anyhow::Result<BuildResult> {
  
  // This is the bundler state. It is passed into the bundling phases with read or write access
  // depending on how that phase uses them
  let mut compilation = Compilation {
    machrc: mach_options.config,
    rpc_hosts: mach_options.rpc_hosts,
    config: MachConfig {
      threads: mach_options.threads,
      entries: mach_options.entries,
      project_root: mach_options.project_root,
      env: mach_options.env,
      out_folder: mach_options.out_folder,
    },
    asset_contents: Default::default(),
    asset_graph: Default::default(),
    bundle_graph: Default::default(),
    plugins: Default::default(),
  };

  // This will read the Machrc and initialize the referenced plugins
  load_plugins(&mut compilation)?;

  // This will resolve imports, transform files and build the AssetGraph.
  resolve_and_transform(&mut compilation)?;

  compilation.asset_graph.debug_render();


  // This will read the asset graph and organize related assets into groupings (a.k.a bundles)
  bundle(&mut compilation)?;

  Ok(BuildResult::default())

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
