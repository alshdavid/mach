use libmach::AssetGraph;
use libmach::AssetMap;
use libmach::BundleGraph;
use libmach::BundleMap;
use libmach::DependencyMap;
use libmach::MachConfig;

/// This will try to create lazy JavaScript, CSS bundles + multiple HTML "bundles".
pub fn bundle_with_splitting(
  _config: &MachConfig,
  _asset_map: &AssetMap,
  _dependency_map: &DependencyMap,
  _asset_graph: &AssetGraph,
  _bundles: &mut BundleMap,
  _bundle_graph: &mut BundleGraph,
) -> Result<(), String> {
  todo!();
  // let mut css_bundle = Bundle {
  //   kind: "css".to_string(),
  //   ..Bundle::default()
  // };

  // let mut entries: Vec<DepRef> = asset_graph
  //   .get_dependencies(&ENTRY_ASSET)
  //   .expect("no entry assets")
  //   .iter()
  //   .map(|(dep_id, asset_id)| DepRef {
  //     asset_id: (*asset_id).clone(),
  //     from_dependency: (*dep_id).clone(),
  //     is_entry: true,
  //   })
  //   .collect();

  // while let Some(dep_ref) = entries.pop() {
  //   // dbg!(&dep_ref);
  //   let mut sync_dependencies = HashSet::<String>::new();
  //   let mut bundle = Bundle {
  //     ..Bundle::default()
  //   };

  //   if dep_ref.is_entry {
  //     bundle.entry_asset = Some(dep_ref.asset_id.clone());
  //   }

  //   let mut q = Vec::<PathBuf>::from([dep_ref.asset_id.clone()]);

  //   while let Some(asset_id) = q.pop() {
  //     let current_asset = asset_map.get(&asset_id).unwrap();

  //     // Bundle JavaScript
  //     if current_asset.kind == "js" {
  //       bundle.assets.insert(asset_id.clone());
  //       bundle.kind = "js".to_string();

  //       let Some(dependencies) = asset_graph.get_dependencies(&asset_id) else {
  //         continue;
  //       };

  //       for (dependency_id, asset_id) in dependencies {
  //         let dependency = dependency_map.get(dependency_id).unwrap();
  //         // dbg!(&dependency);
  //         match dependency.priority {
  //           public::DependencyPriority::Sync => {
  //             if sync_dependencies.insert(dependency_id.clone()) {
  //               q.push(asset_id.clone());
  //             };
  //           }
  //           public::DependencyPriority::Lazy => {
  //             entries.push(DepRef {
  //               asset_id: asset_id.clone(),
  //               from_dependency: dependency_id.clone(),
  //               is_entry: false,
  //             });
  //           }
  //         }
  //       }

  //       continue;
  //     }

  //     // Bundle CSS
  //     if current_asset.kind == "css" {
  //       css_bundle.assets.insert(asset_id.clone());
  //       continue;
  //     }

  //     // Bundle HTML
  //     if current_asset.kind == "html" {
  //       bundle.assets.insert(asset_id.clone());
  //       bundle.kind = "html".to_string();

  //       let Some(dependencies) = asset_graph.get_dependencies(&asset_id) else {
  //         continue;
  //       };

  //       for (dependency_id, asset_id) in dependencies {
  //         entries.push(DepRef {
  //           asset_id: asset_id.clone(),
  //           from_dependency: dependency_id.clone(),
  //           is_entry: true,
  //         });
  //       }

  //       continue;
  //     }
  //   }

  //   bundle.content_key = bundle.generate_id();

  //   if bundle.kind == "js" {
  //     for dep_id in sync_dependencies {
  //       bundle_graph.insert(dep_id, bundle.content_key.clone());
  //     }
  //     bundle.name = bundle.generate_name(asset_map.get_many(&bundle.get_assets()).unwrap());
  //   }

  //   if bundle.kind == "html" {
  //     bundle.name = bundle
  //       .entry_asset
  //       .as_ref()
  //       .unwrap()
  //       .file_name()
  //       .unwrap()
  //       .to_str()
  //       .unwrap()
  //       .to_string();
  //   }

  //   bundle_graph.insert(dep_ref.from_dependency.clone(), bundle.content_key.clone());
  //   bundles.push(bundle);
  // }

  // if css_bundle.assets.len() > 0 {
  //   css_bundle.content_key = css_bundle.generate_id();
  //   css_bundle.name =
  //     css_bundle.generate_name(asset_map.get_many(&css_bundle.get_assets()).unwrap());
  //   bundles.push(css_bundle);
  // }

  // return Ok(());
}

// #[derive(Debug)]
// struct DepRef {
//   asset_id: PathBuf,
//   from_dependency: String,
//   is_entry: bool,
// }
