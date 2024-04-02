use crate::platform::bundling::bundle;
use crate::platform::config::load_plugins;
use crate::platform::emit::emit;
use crate::platform::packaging::package;
use crate::platform::transformation::link_and_transform;
use libmach::AdapterMap;
use libmach::AssetGraphSync;
use libmach::AssetMapSync;
use libmach::BundleGraph;
use libmach::BundleMap;
use libmach::DependencyMapSync;
use libmach::Outputs;

use super::parse_config;
use super::reporter::AppReporter;
use super::BuildCommand;

pub fn main(command: BuildCommand) -> Result<(), String> {
  let config = parse_config(command)?;

  /*
    This is the bundler state. It is passed into
    the bundling phases with read or write permissions
    depending on how that phase uses them
  */
  let asset_map = AssetMapSync::default();
  let dependency_map = DependencyMapSync::default();
  let asset_graph = AssetGraphSync::default();
  let mut bundles = BundleMap::new();
  let mut bundle_graph = BundleGraph::new();
  let mut outputs = Outputs::new();
  let mut reporter = AppReporter::new(&config);
  let mut adapter_map = AdapterMap::new();

  reporter.print_config();

  /*
    load_plugins() will read source the .machrc and will
    fetch then initialize the referenced plugins
  */
  let plugins = load_plugins(&config, &config.machrc, &mut adapter_map)?;

  /*
    link_and_transform() will read source files, identify import statements
    before modifying their contents (like removing TypeScript types).

    This will loop until there are no more import statements to resolve
  */
  link_and_transform(
    config.clone(),
    plugins.clone(),
    asset_map.clone(),
    asset_graph.clone(),
    dependency_map.clone(),
  )?;

  reporter.print_transform_stats(&asset_map);

  if config.debug {
    dbg!(&asset_map);
    dbg!(&dependency_map);
    dbg!(&asset_graph);
  }
  /*
    bundle() will take the asset graph and organize related assets
    into groupings. Each grouping will be emitted as a "bundle"
  */
  bundle(
    config.clone(),
    asset_map.clone(),
    asset_graph.clone(),
    dependency_map.clone(),
    &mut bundles,
    &mut bundle_graph,
  )?;

  reporter.print_bundle_stats(&bundles);

  if config.debug {
    dbg!(&bundles);
    dbg!(&bundle_graph);
  }

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
    &mut bundles,
    &mut bundle_graph,
    &mut outputs,
  )?;

  reporter.print_package_stats();

  if config.debug {
    dbg!(&outputs);
  }

  /*
    emit() writes the contents of the bundles to disk
  */
  emit(&config, &mut bundles, &mut outputs)?;

  reporter.print_emit_stats();
  reporter.print_finished_stats();
  Ok(())
}
