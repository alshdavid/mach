use std::path::PathBuf;

use crate::public;
use crate::public::AssetGraph;
use crate::public::AssetMap;
use crate::public::Bundle;
use crate::public::BundleGraph;
use crate::public::Bundles;
use crate::public::DependencyMap;
use crate::public::ENTRY_ASSET;
use crate::public::NO_ASSET;

pub fn bundle(
  _config: &public::Config,
  asset_map: &mut AssetMap,
  dependency_map: &mut DependencyMap,
  asset_graph: &mut AssetGraph,
  bundles: &mut Bundles,
  bundle_graph: &mut BundleGraph,
) -> Result<(), String> {
  let mut css_bundle = Bundle::new(NO_ASSET.as_path(), "css");

  let mut entries: Vec<DepRef> = asset_graph
    .get_dependencies(&ENTRY_ASSET)
    .expect("no entry assets")
    .iter()
    .map(|(dep_id, asset_id)| DepRef {
      asset_id: (*asset_id).clone(),
      from_dependency: (*dep_id).clone(),
      is_entry: true,
    })
    .collect();

  while let Some(dep_ref) = entries.pop() {
    let mut bundle = Bundle::new(&dep_ref.asset_id, "");

    bundle_graph.insert(dep_ref.from_dependency.clone(), bundle.id.clone());
    bundle.is_entry = dep_ref.is_entry;

    let mut q = Vec::<PathBuf>::from([dep_ref.asset_id.clone()]);

    while let Some(asset_id) = q.pop() {
      let current_asset = asset_map.get(&asset_id).unwrap();

      if bundle.name == "" {
        bundle.name = current_asset.name.clone();
      }

      // Bundle JavaScript
      if current_asset.kind == "js" {
        bundle.assets.insert(asset_id.clone());
        bundle.kind = "js".to_string();

        if bundle.output == "" {
          bundle.output = format!("{}.js", bundle.name);
        }

        let Some(dependencies) = asset_graph.get_dependencies(&asset_id) else {
          continue;
        };

        for (dependency_id, asset_id) in dependencies {
          let dependency = dependency_map.get(dependency_id).unwrap();
          match dependency.priority {
            public::DependencyPriority::Sync => {
              bundle_graph.insert(dependency_id.clone(), bundle.id.clone());
              q.push(asset_id.clone());
            }
            public::DependencyPriority::Lazy => {
              entries.push(DepRef {
                asset_id: asset_id.clone(),
                from_dependency: dependency_id.clone(),
                is_entry: false,
              });
            }
          }
        }

        continue;
      }

      // Bundle CSS
      if current_asset.kind == "css" {
        css_bundle.assets.insert(asset_id.clone());
        if css_bundle.entry_asset == *NO_ASSET {
          css_bundle.update_entry(&asset_id);
        }

        if bundle.output == "" {
          bundle.output = format!("{}.css", bundle.name);
        }

        continue;
      }

      // Bundle HTML
      if current_asset.kind == "html" {
        bundle.assets.insert(asset_id.clone());
        bundle.kind = "html".to_string();
        bundle.output = format!("{}.html", bundle.name);

        let Some(dependencies) = asset_graph.get_dependencies(&asset_id) else {
          continue;
        };

        for (dependency_id, asset_id) in dependencies {
          entries.push(DepRef {
            asset_id: asset_id.clone(),
            from_dependency: dependency_id.clone(),
            is_entry: true,
          });
        }

        continue;
      }
    }

    bundles.push(bundle);
  }

  if css_bundle.assets.len() > 0 {
    bundles.push(css_bundle);
  }

  return Ok(());
}

struct DepRef {
  asset_id: PathBuf,
  from_dependency: String,
  is_entry: bool,
}

/*
// Infer JS exports from imports
// I guess this is the start of tree shaking
// I have enough information to drop unused named exports
// but I don't know if I have enough information here for
// figuring out reexports or namespace exports
// I'm trying to see how far I can get without making
// the transformer tell me what exports are available

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ExportSymbol {
  Named(String),
  Default,
}

fn _infer_exports(
  asset_map: &mut AssetMap,
  dependency_map: &mut DependencyMap,
  asset_graph: &mut AssetGraph,
) -> HashMap<PathBuf, HashSet<ExportSymbol>> {
  let mut export_map = HashMap::<PathBuf, HashSet<ExportSymbol>>::new();

  for (asset_id, dependencies) in asset_graph.iter() {
    let asset = asset_map.get(asset_id).unwrap();
    if asset.kind != "js" {
      continue;
    }
    for (dependency_id, target_asset_id) in dependencies.iter() {
      if !export_map.contains_key(target_asset_id) {
        export_map.insert(target_asset_id.clone(), HashSet::new());
      }
      let exports = export_map.get_mut(target_asset_id).unwrap();
      let dependency = dependency_map.get(&dependency_id).unwrap();
      for sym in &dependency.imported_symbols {
        match sym {
          public::ImportSymbolType::Unnamed => {}
          public::ImportSymbolType::Named(key) => {
            exports.insert(ExportSymbol::Named(key.clone()));
          }
          public::ImportSymbolType::Default => {
            exports.insert(ExportSymbol::Default);
          }
          public::ImportSymbolType::Namespace(_) => {}
          public::ImportSymbolType::Reexport => {}
          public::ImportSymbolType::Dynamic => {}
          public::ImportSymbolType::Commonjs => {}
        }
      }
    }
  }

  return export_map;
}
*/
