use std::fs;

use super::run_resolvers::RunResolversResult;
use crate::platform::config::PluginContainerSync;
use crate::platform::config::TransformerTarget;
use crate::public::Asset;
use crate::public::DependencyOptions;
use crate::public::ExportSymbol;
use crate::public::MachConfig;
use crate::public::ModuleSymbol;
use crate::public::MutableAsset;

pub fn run_transformers(
  config: &MachConfig,
  plugins: &PluginContainerSync,
  asset: &mut Asset,
  resolve_result: &RunResolversResult,
) -> Result<Vec<DependencyOptions>, String> {
  // Replicating Parcel's filename parse logic. Might just remove this
  let mut file_target = TransformerTarget::new(&resolve_result.file_path);

  let file_path = resolve_result.file_path.clone();
  let mut asset_kind = file_target.file_extension.clone();
  let Ok(mut content) = fs::read(&resolve_result.file_path) else {
    return Err(format!(
      "Unable to read file: {:?}",
      resolve_result.file_path
    ));
  };
  let mut asset_dependencies = Vec::<DependencyOptions>::new();
  let mut linking_symbols = Vec::<ModuleSymbol>::new();

  let mut mutable_asset = MutableAsset::new(
    &file_path,
    &mut asset_kind,
    &mut content,
    &mut asset_dependencies,
    &mut linking_symbols,
  );

  let (mut pattern, mut transformers) = plugins.transformers.get(&file_target)?;

  let mut i = 0;
  while i != transformers.len() {
    let Some(transformer) = transformers.get(i) else {
      break;
    };

    transformer.transform(&mut mutable_asset, &config)?;

    // If the file type and pattern changes restart transformers
    if *mutable_asset.kind != file_target.file_extension {
      file_target.update(mutable_asset.kind);

      let (new_pattern, new_transformers) = plugins.transformers.get(&file_target)?;
      // Use new transformers if they are different to current ones
      if new_pattern != pattern {
        transformers = new_transformers;
        pattern = new_pattern;
        i = 0;
        continue;
      }
    }

    i += 1;
  }

  // Update existing Asset with new data
  asset.name = file_target.file_stem.clone();
  asset.content = content;
  asset.kind = asset_kind;
  asset.linking_symbols = linking_symbols;

  Ok(asset_dependencies)
}
