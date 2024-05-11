use crate::public::ContentMapSync;

use super::parse_config;
use super::reporter::AppReporter;
use super::BuildCommand;
use crate::platform::adapters::nodejs::NodejsAdapter;
use crate::platform::adapters::nodejs::NodejsAdapterOptions;
use crate::platform::bundling::bundle;
use crate::platform::config::load_plugins;
use crate::platform::emit::emit;
// use crate::platform::packaging::package;
use crate::platform::transformation::link_and_transform;
use crate::public::AssetGraphSync;
use crate::public::AssetMapSync;
use crate::public::BundleGraphSync;
use crate::public::BundleMapSync;
use crate::public::DependencyMapSync;
use crate::public::MachConfigSync;
use crate::public::OutputsSync;

async fn main_async(config: MachConfigSync) -> Result<(), String> {
  /*
    This is the bundler state. It is passed into
    the bundling phases with read or write permissions
    depending on how that phase uses them
  */
  let asset_map = AssetMapSync::default();
  let content_map = ContentMapSync::default();
  let dependency_map = DependencyMapSync::default();
  let asset_graph = AssetGraphSync::default();
  let bundles = BundleMapSync::default();
  let bundle_graph = BundleGraphSync::default();
  let outputs = OutputsSync::default();
  let mut reporter = AppReporter::new(&config);
  let nodejs_adapter = NodejsAdapter::new(NodejsAdapterOptions {
    workers: config.node_workers.clone() as u8,
  })
  .await;

  reporter.print_config();

  /*
    load_plugins() will read source the .machrc and will
    fetch then initialize the referenced plugins
  */
  let plugins = load_plugins(&config, &config.machrc, &nodejs_adapter).await?;

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
  ).await?;

  reporter.print_transform_stats(&asset_map).await;

  // if config.debug {
  //   dbg!(&asset_map.read().unwrap());
  //   dbg!(&dependency_map.read().unwrap());
  //   dbg!(&asset_graph.read().unwrap());
  // }
  /*
    bundle() will take the asset graph and organize related assets
    into groupings. Each grouping will be emitted as a "bundle"
  */
  bundle(
    config.clone(),
    asset_map.clone(),
    asset_graph.clone(),
    dependency_map.clone(),
    bundles.clone(),
    bundle_graph.clone(),
  ).await?;

  // reporter.print_bundle_stats(&bundles);

  // if config.debug {
  //   dbg!(&bundles.read().unwrap());
  //   dbg!(&bundle_graph.read().unwrap());
  // }

  /*
    package() will take the bundles, obtain their referenced Assets
    and modify them such that they can work in the context of an
    emitted file.

    It also injects the runtime and rewrites import
    statements to point to the new paths
  */
  // package(
  //   config.clone(),
  //   asset_map.clone(),
  //   asset_graph.clone(),
  //   dependency_map.clone(),
  //   bundles.clone(),
  //   bundle_graph.clone(),
  //   outputs.clone(),
  // ).await?;

  // reporter.print_package_stats();

  // if config.debug {
  //   dbg!(&outputs.read().unwrap());
  // }

  /*
    emit() writes the contents of the bundles to disk
  */
  emit(config.clone(), outputs).await?;

  // reporter.print_emit_stats();
  // reporter.print_finished_stats();
  Ok(())
}

/*
  main() initializes the config and starts the async runtime
  then main_async() takes over.
*/
pub fn main(command: BuildCommand) -> Result<(), String> {
  let config = parse_config(command)?;

  tokio::runtime::Builder::new_multi_thread()
    .worker_threads(config.threads)
    .enable_all()
    .build()
    .unwrap()
    .block_on(main_async(config))
}
