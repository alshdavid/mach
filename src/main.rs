mod adapters;
mod args;
mod config;
mod default_plugins;
mod platform;
mod public;
mod transformation;

use std::time::SystemTime;

use crate::config::parse_config;
use crate::public::AssetGraph;
use crate::public::AssetMap;
use crate::public::DependencyGraph;
use crate::transformation::transform;

fn main() {
  let start_time = SystemTime::now();
  let config = parse_config().expect("Could not parse CLI args");

  // Bundle state
  let mut asset_map = AssetMap::new();
  let mut asset_graph = AssetGraph::new();
  let mut dependency_graph = DependencyGraph::new();

  // TODO move this into a "reporter" plugin
  println!("Entry:         {}", config.entry_point.to_str().unwrap());
  println!("Root:          {}", config.project_root.to_str().unwrap());
  if let Some(machrc) = &config.mach_config {
    println!("Mach Config:   {}", machrc.file_path.to_str().unwrap());
  } else {
    println!("Mach Config:   None");
  }
  println!("Out Dir:       {}", config.dist_dir.to_str().unwrap());
  println!("Optimize:      {}", config.optimize);
  println!("Threads:       {}", config.threads);
  println!("Node Workers:  {}", config.node_workers);

  // This phase reads source files and transforms them. New imports
  // are discovered as files are parsed, looping until no more imports exist
  if let Err(err) = transform(
    &config,
    &mut asset_map,
    &mut asset_graph,
    &mut dependency_graph,
  ) {
    println!("Transformation Error");
    println!("{}", err);
    return;
  }

  println!("Assets:        {}", asset_map.len());
  // dbg!(&asset_map);
  // dbg!(&asset_graph);
  // dbg!(&dependency_graph);

  println!("Finished in:   {:.3}s", start_time.elapsed().unwrap().as_nanos() as f64 / 1_000_000 as f64 / 1000 as f64);
}
