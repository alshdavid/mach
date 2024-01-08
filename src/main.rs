mod app_config;
mod bundling;
mod default_plugins;
mod emitting;
mod packaging;
mod platform;
mod public;
mod transformation;

use swc_core::common::SourceMap;

use crate::app_config::app_config;
use crate::bundling::bundle;
use crate::emitting::emit;
use crate::packaging::package;
use crate::platform::Container;
use crate::public::AssetMap;
use crate::public::BundleMap;
use crate::public::DependencyMap;
use crate::transformation::transform;

fn main() {
  let config = app_config().expect("Could not parse CLI args");

  // Data structures are stored in containers that allow the
  // internal values to be extracted. Helps with multi-threading.
  let mut asset_map_ref = Container::new(AssetMap::new());
  let mut dependency_map_ref = Container::new(DependencyMap::new());
  let mut bundle_map_ref = Container::new(BundleMap::new());

  // Global SWC source map
  let mut source_map_ref = Container::new(SourceMap::default());

  println!("Entry:       {}", config.entry_point.to_str().unwrap());
  println!("Root:        {}", config.project_root.to_str().unwrap());
  println!("Workspace:   {:?}", config.workspace_root);
  println!("Out Dir:     {}", config.dist_dir.to_str().unwrap());
  println!("Threads:     {}", config.threads);

  // This phase reads source files and transforms them. New imports
  // are discovered as files are parsed, looping until no more imports exist
  if let Err(err) = transform(
    &config,
    &mut asset_map_ref,
    &mut dependency_map_ref,
    &mut source_map_ref,
  ) {
    println!("Transformation Error");
    println!("{}", err);
    return;
  }

  let assets = asset_map_ref.take();
  println!("Asset {}", assets.len());
  asset_map_ref.insert(assets);

  // This phase reads the dependency graph and produces multiple bundles,
  // each bundle representing and output file
  if let Err(err) = bundle(
    &mut asset_map_ref,
    &mut dependency_map_ref,
    &mut bundle_map_ref,
    &mut source_map_ref,
  ) {
    println!("Bundling Error");
    println!("{}", err);
    return;
  }

  // This phase reads the bundle graph and applies the "runtime" code,
  // to the assets. This is things like rewriting import statements
  if let Err(err) = package(
    &mut asset_map_ref,
    &mut dependency_map_ref,
    &mut bundle_map_ref,
    &mut source_map_ref,
  ) {
    println!("Packaging Error:");
    println!("{}", err);
    return;
  }

  // This phase writes the bundles to disk
  if let Err(err) = emit(
    &config,
    &mut asset_map_ref,
    &mut dependency_map_ref,
    &mut bundle_map_ref,
    &mut source_map_ref,
  ) {
    println!("Emitting Error");
    println!("{}", err);
    return;
  }
}
