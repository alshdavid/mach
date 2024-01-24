mod app_config;
mod bundling;
mod default_plugins;
mod emitting;
mod packaging;
mod platform;
mod public;
mod transformation;

use std::sync::Arc;

use swc_core::common::SourceMap;

use crate::app_config::app_config;
use crate::bundling::bundle;
use crate::emitting::emit;
use crate::packaging::package;
use crate::public::AssetMap;
use crate::public::BundleMap;
use crate::public::DependencyMap;
use crate::transformation::transform;

fn main() {
  let config = app_config().expect("Could not parse CLI args");

  // Bundle state
  let mut asset_map = AssetMap::new();
  let mut dependency_map = DependencyMap::new();
  let mut bundle_map = BundleMap::new();
  let source_map = Arc::new(SourceMap::default());

  println!("Entry:       {}", config.entry_point.to_str().unwrap());
  println!("Root:        {}", config.project_root.to_str().unwrap());
  println!("Workspace:   {:?}", config.workspace_root);
  println!("Out Dir:     {}", config.dist_dir.to_str().unwrap());
  println!("Optimize:    {}", config.optimize);
  println!("Threads:     {}", config.threads);

  // This phase reads source files and transforms them. New imports
  // are discovered as files are parsed, looping until no more imports exist
  if let Err(err) = transform(
    &config,
    &mut asset_map,
    &mut dependency_map,
    source_map.clone(),
  ) {
    println!("Transformation Error");
    println!("{}", err);
    return;
  }

  // This phase reads the dependency graph and produces multiple bundles,
  // each bundle representing and output file
  if let Err(err) = bundle(
    &config,
    &asset_map,
    &dependency_map,
    &mut bundle_map,
  ) {
    println!("Bundling Error");
    println!("{}", err);
    return;
  }

  // // This phase reads the bundle graph and applies the "runtime" code,
  // // to the assets. This is things like rewriting import statements
  if let Err(err) = package(
    &config,
    &mut asset_map,
    &mut dependency_map,
    &mut bundle_map,
    source_map.clone(),
  ) {
    println!("Packaging Error:");
    println!("{}", err);
    return;
  }

  // // This phase writes the bundles to disk
  if let Err(err) = emit(
    &config,
    &bundle_map,
    source_map.clone(),
  ) {
    println!("Emitting Error");
    println!("{}", err);
    return;
  }
}
