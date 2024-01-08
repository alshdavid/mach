mod bundle;
mod app_config;
mod linking;
mod optimize;
mod package;
mod public;
mod render;
mod transform;
mod platform;

use std::time::Instant;

use public::AssetMapRef;
use swc_core::common::SourceMap;

use crate::app_config::AppConfig;
use crate::linking::DependencyIndex;
use crate::linking::link;
use crate::public::AssetMap;
use crate::public::DependencyIndexRef;
use crate::public::DependencyMap;
use crate::public::DependencyMapRef;
use crate::public::SourceMapRef;

fn main() {
  let timing_start = Instant::now();

  let config = match AppConfig::from_env() {
    Ok(v) => v,
    Err(err) => {
      println!("{}", err);
      return;
    }
  };

  println!("Entry:       {}", config.entry_point.to_str().unwrap());
  println!("Root:        {}", config.project_root.to_str().unwrap());
  println!("Workspace:   {:?}", config.workspace_root);
  println!("Out Dir:     {}", config.dist_dir.to_str().unwrap());
  println!("Threads:     {}", config.threads);

  let mut asset_map_ref: AssetMapRef = Some(AssetMap::new());
  let mut dependency_map_ref: DependencyMapRef = Some(DependencyMap::new());
  let mut dependency_index_ref: DependencyIndexRef = Some(DependencyIndex::new());
  let mut source_map_ref: SourceMapRef = Some(SourceMap::default());

  // This phase parses source files into AST, identifying import/export
  // statements and repeating the process on the targets - crawling the
  // sources until there are no files left to process.
  // During this phase we generate a graph of relationships between files.
  if let Err(err) = link(
    &config, 
    &mut asset_map_ref,
    &mut dependency_map_ref, 
    &mut dependency_index_ref,
    &mut source_map_ref,
  ) {
    println!("Error Linking:\n{}", err);
    return;
  }

  let asset_map = asset_map_ref.take().unwrap();
  println!("Assets: {}", asset_map.len());
 
  // let timing_writing = timing_start.elapsed().as_secs_f64();
  // println!("   Writing:      {:.4}s", timing_writing - timing_packaging);
  println!("   ------");
  println!(
    "   Total:        {:.4}s",
    timing_start.elapsed().as_secs_f64()
  );
}
