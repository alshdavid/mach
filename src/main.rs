mod adapters;
mod args;
mod config;
mod default_plugins;
mod platform;
mod public;
mod plugins;
mod transformation;

use std::sync::Arc;

use crate::adapters::node_js::NodeAdapter;
use crate::plugins::load_plugins;
use crate::public::Config;
use crate::config::parse_config;
use crate::public::AssetGraph;
use crate::public::AssetMap;
use crate::public::DependencyGraph;
use crate::transformation::transform;

async fn main_async(config: Config) {
  // Bundle state
  let mut asset_map = AssetMap::new();
  let mut asset_graph = AssetGraph::new();
  let mut dependency_graph = DependencyGraph::new();
  
  // Adapters
  let node_adapter = Arc::new(NodeAdapter::new(1).await);

  // TODO move this into a "reporter" plugin
  println!("Entry:         {}", config.entry_point.to_str().unwrap());
  println!("Root:          {}", config.project_root.to_str().unwrap());
  if let Some(machrc) = &config.machrc {
    println!("Mach Config:   {}", machrc.file_path.to_str().unwrap());
  } else {
    println!("Mach Config:   None");
  }
  println!("Out Dir:       {}", config.dist_dir.to_str().unwrap());
  println!("Optimize:      {}", config.optimize);
  println!("Threads:       {}", config.threads);
  println!("Node Workers:  {}", config.node_workers);
  
  // Initialize plugins
  let Ok(plugins) = load_plugins(
    &config.machrc,
    node_adapter.clone()
  ).await else {
    panic!("Unable to initalize plugins");
  };

  // This phase reads source files and transforms them. New imports
  // are discovered as files are parsed, looping until no more imports exist
  if let Err(err) = transform(
    &config,
    &mut asset_map,
    &mut asset_graph,
    &mut dependency_graph,
    &plugins,
  ).await {
    println!("Transformation Error");
    println!("{}", err);
    return;
  }

  println!("Assets:        {}", asset_map.len());
  // dbg!(&asset_map);
  // dbg!(&asset_graph);
  // dbg!(&dependency_graph);

  println!("Finished in:   {:.3}s", config.start_time.elapsed().unwrap().as_nanos() as f64 / 1_000_000 as f64 / 1000 as f64);
}

fn main() {
  let config = parse_config().expect("Could not parse CLI args");
  tokio::runtime::Builder::new_multi_thread()
    .worker_threads(config.threads)
    .enable_all()
    .build()
    .unwrap()
    .block_on(main_async(config));
}