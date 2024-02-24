use std::path::PathBuf;

use crate::public;
use crate::public::AssetGraph;
use crate::public::AssetMap;
use crate::public::Bundle;
use crate::public::Bundles;
use crate::public::DependencyMap;
use crate::public::ExportSymbol;
use crate::public::ENTRY_ASSET;

pub fn bundle(
  _config: &public::Config,
  asset_map: &mut AssetMap,
  dependency_map: &mut DependencyMap,
  asset_graph: &mut AssetGraph,
  bundles: &mut Bundles,
) -> Result<(), String> {
  let mut css_bundle = Bundle::new("css");
  let mut js_bundle = Bundle::new("js");

  // Create one bundle for now
  let (_, entry_asset_id) = *asset_graph
    .get_dependencies(&ENTRY_ASSET)
    .unwrap()
    .get(0)
    .unwrap();

  js_bundle.entry_asset = entry_asset_id.clone();

  let mut q = Vec::<PathBuf>::from([entry_asset_id.clone()]);

  while let Some(asset_id) = q.pop() {
    let current_asset = asset_map.get(&asset_id).unwrap();
    if current_asset.kind == "js" {
      js_bundle.assets.insert(asset_id.clone());
    } else if current_asset.kind == "css" {
      css_bundle.assets.insert(asset_id.clone());
    } else {
      continue;
    }

    let Some(dependencies) = asset_graph.get_dependencies(&asset_id) else {
      continue;
    };

    // tree shaking
    for (dependency_id, asset_id) in dependencies {
      q.push(asset_id.clone());

      let asset = asset_map.get(asset_id).unwrap();
      let dependency = dependency_map.get(dependency_id).unwrap();

      for import in &dependency.imported_symbols {
        match import {
          public::ImportSymbolType::Named(name) => 'exports: {
            for export in &asset.exports {
              if let ExportSymbol::Named(export_name) = &export {
                if export_name == name {
                  js_bundle.insert_export_symbol(asset_id, export.clone());
                  break 'exports;
                }
              }
            }
            return Err(format!("{:?} does not export {:?}", asset_id, name));
          }
          public::ImportSymbolType::Unnamed => {
            for export in &asset.exports {
              js_bundle.insert_export_symbol(asset_id, export.clone());
            }
          }
          public::ImportSymbolType::Default => 'exports: {
            for export in &asset.exports {
              if let ExportSymbol::Default = &export {
                js_bundle.insert_export_symbol(asset_id, export.clone());
                break 'exports;
              }
            }
            return Err(format!("{:?} does not have a default export", asset_id));
          }
          public::ImportSymbolType::Namespace(_) => {
            for export in &asset.exports {
              js_bundle.insert_export_symbol(asset_id, export.clone());
            }
          }
          public::ImportSymbolType::Reexport => {
            for export in &asset.exports {
              js_bundle.insert_export_symbol(asset_id, export.clone());
            }
          }
          public::ImportSymbolType::Dynamic => {
            for export in &asset.exports {
              js_bundle.insert_export_symbol(asset_id, export.clone());
            }
          }
          public::ImportSymbolType::Commonjs => {
            for export in &asset.exports {
              js_bundle.insert_export_symbol(asset_id, export.clone());
            }
          }
        };
      }
    }
  }

  bundles.push(js_bundle);
  if css_bundle.assets.len() != 0 {
    bundles.push(css_bundle);
  }
  return Ok(());
}
