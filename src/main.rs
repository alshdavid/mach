mod bundle;
mod config;
mod linking;
mod optimize;
mod package;
mod public;
mod render;
mod transform;
mod platform;

use std::env;
use std::fs;
use std::time::Instant;

use swc_core::common::sync::Lrc;
use swc_core::common::SourceMap;

use crate::bundle::bundle;
use crate::config::AppConfig;
use crate::linking::link;
use crate::optimize::optimize;
use crate::package::package;
use crate::public::AssetMap;
use crate::public::DependencyMap;
use crate::render::render;
use crate::transform::transform_pipeline;

/*
    TODO:
        * import alias support (https://parceljs.org/features/dependency-resolution/#global-aliases)
        * threaded transformations
        * multiple entries
        * css import
        * html entries
*/

fn main() {
  let timing_start = Instant::now();
  let config = AppConfig::new();

  println!("Entry:    {}", config.entry_point.to_str().unwrap());
  println!("Root:     {}", config.project_root.to_str().unwrap());
  println!("Out Dir:  {}", config.dist_dir.to_str().unwrap());
  println!("Threads:  {}", config.threads);

  let asset_map = AssetMap::new();
  let dependency_map = DependencyMap::new();
  let source_map = Lrc::new(SourceMap::default());

  // This phase parses source files into AST, identifying import/export
  // statements and repeating the process on the targets - crawling the
  // sources until there are no files left to process.
  // During this phase we generate a graph of relationships between files.
  let (asset_map, dependency_map, dependency_index, source_map) =
    match link(&config, asset_map, dependency_map, source_map) {
      Ok(v) => v,
      Err(err) => {
        println!("Error Linking:\n{}", err);
        return;
      }
    };

  let timing_linking = timing_start.elapsed().as_secs_f64();
  println!("Assets:   {}", asset_map.len());
  println!("Timings:");
  println!("   Linking:      {:.4}s", timing_linking);

  // This phase modifies the assets applying transformations like
  // converting JSX/TSX or stripping TypeScript types.
  let (asset_map, dependency_map, dependency_index, source_map) = match transform_pipeline(
    &config,
    asset_map,
    dependency_map,
    dependency_index,
    source_map,
  ) {
    Ok(v) => v,
    Err(err) => {
      println!("Error Transforming:\n{}", err);
      return;
    }
  };

  let timing_transforming = timing_start.elapsed().as_secs_f64();
  println!(
    "   Transforming: {:.4}s",
    timing_transforming - timing_linking
  );

  // This phase analyzes the bundle graph and determines how to divide
  // assets into their respective bundles. Each bundle represents a file
  // that will be written to disk.
  let (source_map, bundle_map, bundle_dependency_index) = match bundle(
    &config,
    asset_map,
    dependency_map,
    dependency_index,
    source_map,
  ) {
    Ok(v) => v,
    Err(err) => {
      println!("Error Bundling:\n{}", err);
      return;
    }
  };

  let timing_bundling = timing_start.elapsed().as_secs_f64();
  println!(
    "   Bundling:     {:.4}s",
    timing_bundling - timing_transforming
  );

  // This stage generates a Module for each file in the bundle map, wrapping
  // the assets within each bundle in the runtime code and transforming the
  // import/export/commonjs statements so they are compatible
  let (mut source_map, mut packages) =
    match package(&config, bundle_map, bundle_dependency_index, source_map) {
      Ok(v) => v,
      Err(err) => {
        println!("Error Packaging:\n{}", err);
        return;
      }
    };

  let timing_packaging = timing_start.elapsed().as_secs_f64();
  println!(
    "   Packaging:    {:.4}s",
    timing_packaging - timing_bundling
  );

  // This stage minifies the code within the bundles
  if config.optimize == true {
    (source_map, packages) = match optimize(packages, source_map) {
      Ok(v) => v,
      Err(err) => {
        println!("Error Optimizing:\n{}", err);
        return;
      }
    };
  }

  let timing_optimize = timing_start.elapsed().as_secs_f64();
  println!(
    "   Optimizing:   {:.4}s",
    timing_optimize - timing_packaging
  );

  // This stage writes the bundle modules to disk
  fs::create_dir_all(&config.dist_dir).unwrap();
  for (out_file, bundle) in packages {
    let bundle_str = render(&bundle, source_map.clone());
    fs::write(config.dist_dir.join(out_file), bundle_str).unwrap();
    // println!("{}", bundle_str);
  }

  let timing_writing = timing_start.elapsed().as_secs_f64();
  println!("   Writing:      {:.4}s", timing_writing - timing_packaging);
  println!("   ------");
  println!(
    "   Total:        {:.4}s",
    timing_start.elapsed().as_secs_f64()
  );
}
