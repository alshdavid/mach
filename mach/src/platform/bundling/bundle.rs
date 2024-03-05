use std::collections::HashSet;
use std::path::PathBuf;

use crate::public;
use crate::public::AssetGraph;
use crate::public::AssetMap;
use crate::public::Bundle;
use crate::public::BundleGraph;
use crate::public::Bundles;
use crate::public::DependencyMap;
use crate::public::ENTRY_ASSET;

pub fn bundle(
  _config: &public::Config,
  asset_map: &AssetMap,
  dependency_map: &DependencyMap,
  asset_graph: &AssetGraph,
  bundles: &mut Bundles,
  bundle_graph: &mut BundleGraph,
) -> Result<(), String> {
  let mut css_bundle = Bundle {
    kind: "css".to_string(),
    ..Bundle::default()
  };

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
    // dbg!(&dep_ref);
    let mut sync_dependencies = HashSet::<String>::new();
    let mut bundle = Bundle {
      ..Bundle::default()
    };

    if dep_ref.is_entry {
      bundle.entry_asset = Some(dep_ref.asset_id.clone());
    }

    let mut q = Vec::<PathBuf>::from([dep_ref.asset_id.clone()]);

    while let Some(asset_id) = q.pop() {
      let current_asset = asset_map.get(&asset_id).unwrap();

      // Bundle JavaScript
      if current_asset.kind == "js" {
        bundle.assets.insert(asset_id.clone());
        bundle.kind = "js".to_string();

        let Some(dependencies) = asset_graph.get_dependencies(&asset_id) else {
          continue;
        };

        for (dependency_id, asset_id) in dependencies {
          let dependency = dependency_map.get(dependency_id).unwrap();
          // dbg!(&dependency);
          match dependency.priority {
            public::DependencyPriority::Sync => {
              sync_dependencies.insert(dependency_id.clone());
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
        continue;
      }

      // Bundle HTML
      if current_asset.kind == "html" {
        bundle.assets.insert(asset_id.clone());
        bundle.kind = "html".to_string();

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

    bundle.id = bundle.generate_id();

    if bundle.kind == "js" {
      for dep_id in sync_dependencies {
        bundle_graph.insert(dep_id, bundle.id.clone());
      }
      bundle.name = bundle.generate_name(asset_map.get_many(&bundle.get_assets()).unwrap());
    }

    if bundle.kind == "html" {
      bundle.name = bundle
        .entry_asset
        .as_ref()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    }

    bundle_graph.insert(dep_ref.from_dependency.clone(), bundle.id.clone());
    bundles.push(bundle);
  }

  if css_bundle.assets.len() > 0 {
    css_bundle.id = css_bundle.generate_id();
    css_bundle.name =
      css_bundle.generate_name(asset_map.get_many(&css_bundle.get_assets()).unwrap());
    bundles.push(css_bundle);
  }

  return Ok(());
}

#[derive(Debug)]
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
