use std::path::PathBuf;

use crate::public;
use crate::public::AssetGraph;
use crate::public::AssetMap;
use crate::public::Bundle;
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
) -> Result<(), String> {
  let mut css_bundle = Bundle::default();
  css_bundle.kind = "css".to_string();

  let mut entries: Vec<PathBuf> = asset_graph
    .get_dependencies(&ENTRY_ASSET)
    .expect("no entry assets")
    .iter()
    .map(|x| x.1.clone())
    .collect();

  while let Some(entry_asset_id) = entries.pop() {
    let mut bundle = Bundle::default();
    bundle.entry_asset = entry_asset_id.clone();

    let mut q = Vec::<PathBuf>::from([entry_asset_id.clone()]);

    while let Some(asset_id) = q.pop() {
      let current_asset = asset_map.get(&asset_id).unwrap();

      if bundle.name == "" {
        bundle.name = current_asset.name.clone();
      }

      if current_asset.kind == "js" {
        bundle.assets.insert(asset_id.clone());
        bundle.kind = "js".to_string();
      } else if current_asset.kind == "css" {
        css_bundle.assets.insert(asset_id.clone());
        if css_bundle.entry_asset == *NO_ASSET {
          css_bundle.entry_asset = asset_id.clone();
          css_bundle.name = current_asset.name.clone();
        }
      } else if current_asset.kind == "html" {
        bundle.assets.insert(asset_id.clone());
        bundle.kind = "html".to_string();
      }else {
        continue;
      }

      let Some(dependencies) = asset_graph.get_dependencies(&asset_id) else {
        continue;
      };

      for (dependency_id, asset_id) in dependencies {
        let dependency = dependency_map.get(dependency_id).unwrap();
        match dependency.priority {
          public::DependencyPriority::Sync => {
            q.push(asset_id.clone());
          }
          public::DependencyPriority::Lazy => {
            entries.push(asset_id.clone());
          }
        }
      }
    }

    bundles.push(bundle);
  }

  bundles.push(css_bundle);
  return Ok(());
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