use std::sync::Arc;

use crate::platform::adapters::node_js::NodeAdapter;
use crate::platform::bundling::bundle;
use crate::platform::emit::emit;
use crate::platform::packaging::package;
use crate::platform::plugins::load_plugins;
use crate::platform::public::AssetGraph;
use crate::platform::public::AssetMap;
use crate::platform::public::BundleGraph;
use crate::platform::public::Bundles;
use crate::platform::public::Config;
use crate::platform::public::DependencyMap;
use crate::platform::public::Packages;
use crate::platform::transformation::transform;

use super::{parse_config, BuildCommand};

async fn main_async(config: Config) {
  // Bundle state
  let mut asset_map = AssetMap::new();
  let mut dependency_map = DependencyMap::new();
  let mut asset_graph = AssetGraph::new();
  let mut bundles = Bundles::new();
  let mut bundle_graph = BundleGraph::new();
  let mut packages = Packages::new();

  // Adapters
  let node_adapter = Arc::new(NodeAdapter::new(config.node_workers).await);

  // TODO move this into a "reporter" plugin
  println!("Entry:         {}", config.entry_point.to_str().unwrap());
  println!("Root:          {}", config.project_root.to_str().unwrap());
  if !&config.machrc.is_default {
    println!(
      "Mach Config:   {}",
      config.machrc.file_path.to_str().unwrap()
    );
  } else {
    println!("Mach Config:   Default");
  }
  println!("Out Dir:       {}", config.dist_dir.to_str().unwrap());
  println!("Optimize:      {}", config.optimize);
  println!("Threads:       {}", config.threads);
  println!("Node Workers:  {}", config.node_workers);

  // Initialize plugins
  let Ok(plugins) = load_plugins(&config.machrc, node_adapter.clone()).await else {
    panic!("Unable to initialize plugins");
  };

  // This phase reads source files and transforms them. New imports
  // are discovered as files are parsed, looping until no more imports exist
  if let Err(err) = transform(
    &config,
    &mut asset_map,
    &mut dependency_map,
    &mut asset_graph,
    &plugins,
  )
  .await
  {
    println!("Transformation Error");
    println!("{}", err);
    return;
  }

  println!("Assets:        {}", asset_map.len());

  // dbg!(&asset_map);
  // dbg!(&asset_graph);
  // dbg!(&dependency_map);

  if let Err(err) = bundle(
    &config,
    &mut asset_map,
    &mut dependency_map,
    &mut asset_graph,
    &mut bundles,
    &mut bundle_graph,
  ) {
    println!("Bundling Error");
    println!("{}", err);
    return;
  }

  // dbg!(&bundles);
  // dbg!(&bundle_graph);

  if let Err(err) = package(
    &config,
    &mut asset_map,
    &mut dependency_map,
    &mut asset_graph,
    &mut bundles,
    &mut bundle_graph,
    &mut packages,
  ) {
    println!("Packaging Error");
    println!("{}", err);
    return;
  }

  // dbg!(&packages);

  if let Err(err) = emit(&config, &mut bundles, &mut packages) {
    println!("Packaging Error");
    println!("{}", err);
    return;
  }

  println!(
    "Finished in:   {:.3}s",
    config.start_time.elapsed().unwrap().as_nanos() as f64 / 1_000_000 as f64 / 1000 as f64
  );
}


pub fn main(command: BuildCommand) {
  let config = parse_config(command).expect("Failed to init config");
  tokio::runtime::Builder::new_multi_thread()
    .worker_threads(config.threads)
    .enable_all()
    .build()
    .unwrap()
    .block_on(main_async(config));
}