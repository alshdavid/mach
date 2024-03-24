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
