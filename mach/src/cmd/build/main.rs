
use crate::platform::bundling::bundle;
use crate::platform::emit::emit;
use crate::platform::packaging::package;
use crate::platform::config::load_plugins;
use crate::platform::transformation::link_and_transform;
use libmach::AdapterMap;
use libmach::AssetGraph;
use libmach::AssetMap;
use libmach::BundleGraph;
use libmach::Bundles;
use libmach::Config;
use libmach::DependencyMap;
use libmach::Outputs;

use super::parse_config;
use super::BuildCommand;

async fn main_async(config: Config) -> Result<(), String> {
  config.log_details();

  /*
    This is the bundler state. It is passed into
    the bundling phases with read or write permissions
    depending on how that phase uses them
  */
  let mut asset_map = AssetMap::new();
  let mut dependency_map = DependencyMap::new();
  let mut asset_graph = AssetGraph::new();
  let mut bundles = Bundles::new();
  let mut bundle_graph = BundleGraph::new();
  let mut outputs = Outputs::new();
  let mut adapter_map = AdapterMap::new();

  /*
  load_plugins() will read source the .machrc and will
  fetch then initialize the referenced plugins
  */
  let mut plugins = load_plugins(
    &config.machrc,
    &mut adapter_map,
  ).await?;

  /*
    link_and_transform() will read source files, identify import statements
    before modifying their contents (like removing TypeScript types).

    This will loop until there are no more import statements to resolve
  */
  link_and_transform(
    &config,
    &mut plugins,
    &mut asset_map,
    &mut dependency_map,
    &mut asset_graph,
  )
  .await?;

  let time_transform = config.time_elapsed();
  println!(
    "  Transform:     {:.3}s  (Assets {})",
    time_transform,
    asset_map.len()
  );

  /*
    bundle() will take the asset graph and organize related assets
    into groupings. Each grouping will be emitted as a "bundle"
  */
  bundle(
    &config,
    &asset_map,
    &dependency_map,
    &asset_graph,
    &mut bundles,
    &mut bundle_graph,
  )?;

  let time_bundle = config.time_elapsed();
  println!(
    "  Bundle:        {:.3}s  (Bundles {})",
    time_bundle - time_transform,
    bundles.len()
  );

  /*
    package() will take the bundles, obtain their referenced Assets
    and modify them such that they can work in the context of an
    emitted file.

    It also injects the runtime and rewrites import
    statements to point to the new paths
  */
  package(
    &config,
    &mut dependency_map,
    &mut asset_graph,
    &mut bundles,
    &mut bundle_graph,
    &mut asset_map,
    &mut outputs,
  )
  .await?;

  let time_package = config.time_elapsed();
  println!("  Package:       {:.3}s", time_package - time_bundle);

  /*
    emit() writes the contents of the bundles to disk
  */
  emit(&config, &mut bundles, &mut outputs)?;

  let time_emit = config.time_elapsed();
  println!("  Emit:          {:.3}s", time_emit - time_package);

  println!("Finished in:   {:.3}s", config.time_elapsed(),);
  Ok(())
}

/*
  main() initializes the config and starts the async runtime
  then main_async() takes over.
*/
pub fn main(command: BuildCommand) {
  let config = match parse_config(command) {
    Ok(config) => config,
    Err(msg) => {
      println!("Init Error:");
      println!("  {}", msg);
      std::process::exit(1);
    }
  };
  if let Err(msg) = tokio::runtime::Builder::new_multi_thread()
    .worker_threads(config.threads)
    .enable_all()
    .build()
    .unwrap()
    .block_on(main_async(config))
  {
    println!("Build Error:");
    println!("{}", msg);
  };
}
