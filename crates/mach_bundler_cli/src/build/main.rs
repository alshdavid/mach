use super::parse_config;
use super::reporter::AppReporter;
use super::BuildCommand;
use mach_bundler_core::platform::adapters::nodejs::NodejsAdapter;
use mach_bundler_core::platform::adapters::nodejs::NodejsAdapterOptions;
use mach_bundler_core::platform::bundling::bundle;
use mach_bundler_core::platform::config::load_plugins;
use mach_bundler_core::platform::emit::emit;
use mach_bundler_core::platform::packaging::package;
use mach_bundler_core::platform::transformation::resolve_and_transform;
use mach_bundler_core::public::programmatic::ProgrammaticReporter;
use mach_bundler_core::public::AssetGraphSync;
use mach_bundler_core::public::AssetMapSync;
use mach_bundler_core::public::BundleGraphSync;
use mach_bundler_core::public::BundleManifestSync;
use mach_bundler_core::public::BundleMapSync;
use mach_bundler_core::public::DependencyMapSync;
use mach_bundler_core::public::OutputsSync;

pub fn main(command: BuildCommand) -> Result<(), String> {
  let config = parse_config(command)?;

  /*
    This is the bundler state. It is passed into
    the bundling phases with read or write permissions
    depending on how that phase uses them
  */
  let asset_map = AssetMapSync::default();
  let dependency_map = DependencyMapSync::default();
  let asset_graph = AssetGraphSync::default();
  let bundles = BundleMapSync::default();
  let bundle_graph = BundleGraphSync::default();
  let bundle_manifest = BundleManifestSync::default();
  let outputs = OutputsSync::default();
  let programmatic_reporter = ProgrammaticReporter::new(
    config.clone(),
    asset_map.clone(),
    bundles.clone(),
    bundle_manifest.clone(),
  );
  let nodejs_adapter = NodejsAdapter::new(NodejsAdapterOptions {
    workers: config.node_workers.clone() as u8,
  })?;

  let mut reporter = AppReporter::new(
    config.clone(),
    dependency_map.clone(),
    asset_map.clone(),
    asset_graph.clone(),
    bundles.clone(),
    bundle_graph.clone(),
    outputs.clone(),
    nodejs_adapter.clone(),
  );

  reporter.print_config();

  /*
    load_plugins() will read source the .machrc and will
    fetch then initialize the referenced plugins
  */
  let plugins = load_plugins(&config, &config.machrc, nodejs_adapter.clone())?;

  reporter.print_init_stats();

  /*
    resolve_and_transform() will read source files, identify import statements
    before modifying their contents (like removing TypeScript types).

    This will loop until there are no more import statements to resolve
  */
  resolve_and_transform(
    config.clone(),
    plugins.clone(),
    asset_map.clone(),
    asset_graph.clone(),
    dependency_map.clone(),
  )?;

  reporter.print_transform_stats();

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
  )?;

  reporter.print_bundle_stats();

  /*
    package() will take the bundles, obtain their referenced Assets
    and modify them such that they can work in the context of an
    emitted file.

    It also injects the runtime and rewrites import
    statements to point to the new paths
  */
  package(
    config.clone(),
    asset_map.clone(),
    asset_graph.clone(),
    dependency_map.clone(),
    bundles.clone(),
    bundle_graph.clone(),
    bundle_manifest.clone(),
    outputs.clone(),
  )?;

  reporter.print_package_stats();

  /*
    emit() writes the contents of the bundles to disk
  */
  emit(config.clone(), outputs)?;

  reporter.print_emit_stats();
  reporter.print_finished_stats();
  programmatic_reporter.emit_build_report();
  Ok(())
}
