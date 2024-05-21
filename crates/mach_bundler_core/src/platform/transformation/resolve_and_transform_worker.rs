// use std::sync::Arc;
// use std::sync::RwLock;

// use super::run_graph_update::run_update_graph;
// use super::run_resolvers::run_resolvers;
// use super::run_transformers::run_transformers;
// use crate::platform::config::PluginContainerSync;
// use crate::public::AssetGraphSync;
// use crate::public::AssetMapSync;
// use crate::public::Dependency;
// use crate::public::DependencyMapSync;
// use crate::public::MachConfig;

// pub fn resolve_and_transform_worker(
//   config: &MachConfig,
//   plugins: &PluginContainerSync,
//   asset_map: &AssetMapSync,
//   asset_graph: &AssetGraphSync,
//   dependency_map: &DependencyMapSync,
//   queue: &Arc<RwLock<Vec<Dependency>>>,
//   dependency: Dependency,
// ) -> Result<(), String> {
//   let resolve_result = run_resolvers(plugins, &dependency)?;

//   let (asset_id, inserted) = run_update_graph(
//     asset_map,
//     asset_graph,
//     dependency_map,
//     dependency,
//     &resolve_result,
//   )?;

//   if !inserted {
//     return Ok(());
//   }

//   let mut asset_dependencies = run_transformers(
//     config,
//     plugins,
//     asset_map,
//     resolve_result.clone(),
//     asset_id.clone(),
//   )?;

//   let mut new_dependencies = Vec::<Dependency>::new();

//   // Add new items to the queue
//   while let Some(dependency_options) = asset_dependencies.pop() {
//     let new_dependency = Dependency {
//       specifier: dependency_options.specifier.clone(),
//       specifier_type: dependency_options.specifier_type,
//       is_entry: false,
//       source_path: resolve_result.file_path.clone(),
//       source_asset: asset_id.clone(),
//       resolve_from: resolve_result.file_path.clone(),
//       priority: dependency_options.priority,
//       imported_symbols: dependency_options.imported_symbols,
//       bundle_behavior: dependency_options.bundle_behavior,
//       ..Default::default()
//     };

//     new_dependencies.push(new_dependency);
//   }

//   queue.write().unwrap().extend(new_dependencies);

//   Ok(())
// }
