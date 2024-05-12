use std::fs;

use crate::platform::config::PluginContainerSync;
use crate::platform::config::TransformerTarget;
use crate::public::AssetId;
use crate::public::AssetMapSync;
use crate::public::DependencyOptions;
use crate::public::MachConfig;
use crate::public::MutableAsset;
use crate::public::ResolveResult;

pub fn run_transformers(
  config: &MachConfig,
  plugins: &PluginContainerSync,
  asset_map: &AssetMapSync,
  resolve_result: ResolveResult,
  asset_id: AssetId,
) -> Result<Vec<DependencyOptions>, String> {
  let mut file_target = TransformerTarget::new(&resolve_result.file_path);

  let mut content =
    fs::read(&resolve_result.file_path).map_err(|_| "Unable to read file".to_string())?;

  let mut asset_dependencies = Vec::<DependencyOptions>::new();
  let mut asset_kind = file_target.file_extension.clone();

  let mut mutable_asset = MutableAsset::new(
    resolve_result.file_path.clone(),
    &mut asset_kind,
    &mut content,
    &mut asset_dependencies,
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
  let mut asset_map = asset_map.write().unwrap();
  let asset = asset_map.get_mut(&asset_id).unwrap();
  asset.name = file_target.file_stem.clone();
  asset.content = content;
  asset.kind = asset_kind;
  asset.file_path_relative =
    pathdiff::diff_paths(&resolve_result.file_path, &config.project_root).unwrap();

  Ok(asset_dependencies)
}
