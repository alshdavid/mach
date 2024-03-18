use std::collections::HashMap;
use std::path::PathBuf;
use std::pin::pin;
use std::pin::Pin;

use libloading::Library;
use libloading::library_filename;
use libmach::AdapterBootstrapFn;
use libmach::AdapterOption;
use libmach::Dependency;
use libmach::DependencyPriority;
use libmach::SpecifierType;
use libmach::BundleBehavior;

use super::BuildCommand;

pub fn main(command: BuildCommand) {
  tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap()
    .block_on(main_async());
}

pub async fn main_async() {
  let exe_path = std::env::current_exe().unwrap();
  let exe_dir = exe_path.parent().unwrap();
  let mach_dir = exe_dir.parent().unwrap();
  let mach_lib_dir = mach_dir.join("adapters");
  println!("{:?}", mach_lib_dir.join("adapter_noop").join("adapter.so"));
  unsafe {
    let lib = libloading::Library::new(mach_lib_dir.join("noop").join("lib.so")).unwrap();
    let bootstrap: libloading::Symbol<AdapterBootstrapFn> = lib.get(b"bootstrap").unwrap();
    let adapter = bootstrap(Box::new(HashMap::new()));
    let adapter = adapter.await.unwrap();
    let resolver = adapter.get_resolver(HashMap::from([("foo".to_string(), AdapterOption::String("bar".to_string()))])).await.unwrap();

    resolver.resolve(&Dependency{
      id: "hello world".into(),
      specifier: "".into(),
      specifier_type: SpecifierType::ESM,
      is_entry: false,
      priority: DependencyPriority::Lazy,
      source_path: PathBuf::new(),
      resolve_from: PathBuf::new(),
      resolve_from_rel: PathBuf::new(),
      imported_symbols: vec![],
      bundle_behavior: BundleBehavior::Default,
    }).await.unwrap();
  }
}

// use std::sync::Arc;

// use crate::platform::adapters::node_js::NodeAdapter;
// use crate::platform::bundling::bundle;
// use crate::platform::emit::emit;
// use crate::platform::packaging::package;
// use crate::platform::config::load_plugins;
// use crate::platform::transformation::link_and_transform;
// use libmach::AssetGraph;
// use libmach::AssetMap;
// use libmach::BundleGraph;
// use libmach::Bundles;
// use libmach::Config;
// use libmach::DependencyMap;
// use libmach::Outputs;

// use super::parse_config;
// use super::BuildCommand;

// async fn main_async(config: Config) -> Result<(), String> {
//   config.log_details();

//   /*
//     This is the bundler state. It is passed into
//     the bundling phases with read or write permissions
//     depending on how that phase uses them
//   */
//   let mut asset_map = AssetMap::new();
//   let mut dependency_map = DependencyMap::new();
//   let mut asset_graph = AssetGraph::new();
//   let mut bundles = Bundles::new();
//   let mut bundle_graph = BundleGraph::new();
//   let mut outputs = Outputs::new();

//   /*
//     Adapters are responsible for interoperability with
//     external plugin execution contexts (like Node.js)
//   */
//   let node_adapter = Arc::new(NodeAdapter::new(config.node_workers).await);

//   /*
//   load_plugins() will read source the .machrc and will
//   fetch then initialize the referenced plugins
//   */
//   let mut plugins = load_plugins(&config.machrc, node_adapter.clone()).await?;

//   /*
//     link_and_transform() will read source files, identify import statements
//     before modifying their contents (like removing TypeScript types).

//     This will loop until there are no more import statements to resolve
//   */
//   link_and_transform(
//     &config,
//     &mut plugins,
//     &mut asset_map,
//     &mut dependency_map,
//     &mut asset_graph,
//   )
//   .await?;

//   let time_transform = config.time_elapsed();
//   println!(
//     "  Transform:     {:.3}s  (Assets {})",
//     time_transform,
//     asset_map.len()
//   );

//   /*
//     bundle() will take the asset graph and organize related assets
//     into groupings. Each grouping will be emitted as a "bundle"
//   */
//   bundle(
//     &config,
//     &asset_map,
//     &dependency_map,
//     &asset_graph,
//     &mut bundles,
//     &mut bundle_graph,
//   )?;

//   let time_bundle = config.time_elapsed();
//   println!(
//     "  Bundle:        {:.3}s  (Bundles {})",
//     time_bundle - time_transform,
//     bundles.len()
//   );

//   /*
//     package() will take the bundles, obtain their referenced Assets
//     and modify them such that they can work in the context of an
//     emitted file.

//     It also injects the runtime and rewrites import
//     statements to point to the new paths
//   */
//   package(
//     &config,
//     &mut dependency_map,
//     &mut asset_graph,
//     &mut bundles,
//     &mut bundle_graph,
//     &mut asset_map,
//     &mut outputs,
//   )
//   .await?;

//   let time_package = config.time_elapsed();
//   println!("  Package:       {:.3}s", time_package - time_bundle);

//   /*
//     emit() writes the contents of the bundles to disk
//   */
//   emit(&config, &mut bundles, &mut outputs)?;

//   let time_emit = config.time_elapsed();
//   println!("  Emit:          {:.3}s", time_emit - time_package);

//   println!("Finished in:   {:.3}s", config.time_elapsed(),);
//   Ok(())
// }

// /*
//   main() initializes the config and starts the async runtime
//   then main_async() takes over.
// */
// pub fn main(command: BuildCommand) {
//   let config = match parse_config(command) {
//     Ok(config) => config,
//     Err(msg) => {
//       println!("Init Error:");
//       println!("  {}", msg);
//       std::process::exit(1);
//     }
//   };
//   if let Err(msg) = tokio::runtime::Builder::new_multi_thread()
//     .worker_threads(config.threads)
//     .enable_all()
//     .build()
//     .unwrap()
//     .block_on(main_async(config))
//   {
//     println!("Build Error:");
//     println!("{}", msg);
//   };
// }
