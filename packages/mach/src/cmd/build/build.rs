use std::collections::HashMap;

use anyhow;
use serde::Deserialize;
use serde::Serialize;

use super::super::MachOptions;
use crate::core::bundling::bundle;
use crate::core::emit::emit;
use crate::core::emit::emit_file;
use crate::core::packaging::package;
use crate::core::plugins::load_plugins;
use crate::core::resolve_and_transform::resolve_and_transform;
use crate::types::Compilation;
use crate::types::DebugAssetGraphOptions;
use crate::types::MachConfig;

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
  let mut c = Compilation {
    machrc: mach_options.config,
    rpc_hosts: mach_options.rpc_hosts,
    config: MachConfig {
      threads: mach_options.threads,
      entries: mach_options.entries,
      project_root: mach_options.project_root,
      env: mach_options.env,
      out_folder: mach_options.out_folder,
    },
    asset_graph: Default::default(),
    bundle_graph: Default::default(),
    plugins: Default::default(),
    outputs: Default::default(),
  };

  // This will read the Machrc and initialize the referenced plugins
  load_plugins(&mut c)?;

  // This will resolve imports, transform files and build the AssetGraph.
  resolve_and_transform(&mut c)?;
  emit_file(
    &c,
    "asset_graph.dot",
    c.debug_asset_graph_dot(DebugAssetGraphOptions {
      show_specifiers: false,
    })?,
  )?;

  // This will read the asset graph and organize related assets into groupings (a.k.a bundles)
  bundle(&mut c)?;
  emit_file(&c, "bundle_graph.dot", c.debug_bundle_graph_dot()?)?;

  // This will apply the runtime to and optimize the bundles
  package(&mut c)?;

  // This will write the contents of the packaged bundles to disk
  emit(&mut c)?;

  Ok(BuildResult::default())
}
