use std::fs;

use super::run_resolvers::RunResolversResult;
use crate::types::BundleBehavior;
use crate::types::Compilation;
use crate::types::DependencyOptions;
use crate::types::LinkingSymbol;
use crate::types::MutableAsset;

pub struct TransformerPipelineResult {
  pub name: String,
  pub kind: String,
  pub content: Vec<u8>,
  pub linking_symbols: Vec<LinkingSymbol>,
  pub bundle_behavior: BundleBehavior,
  pub dependencies: Vec<DependencyOptions>,
}

pub fn run_transformers(
  c: &Compilation,
  resolve_result: &RunResolversResult,
) -> anyhow::Result<TransformerPipelineResult> {
  let mut file_path = resolve_result.file_path.clone();

  let Ok(mut content) = fs::read(&resolve_result.file_path) else {
    anyhow::bail!("Unable to read file: {:?}", resolve_result.file_path)
  };
  let mut asset_dependencies = Vec::<DependencyOptions>::new();
  let mut linking_symbols = Vec::<LinkingSymbol>::new();
  let mut bundle_behavior = BundleBehavior::Inline;

  let (mut pattern, mut transformers) = c.plugins.transformers.get(&file_path)?;

  let mut i = 0;
  while i != transformers.len() {
    let Some(transformer) = transformers.get(i) else {
      break;
    };

    let mut asset_kind = file_path
      .extension()
      .unwrap_or_default()
      .to_str()
      .unwrap_or_default()
      .to_string();
    let original_asset_kind = asset_kind.clone();

    let mut mutable_asset = MutableAsset::new(
      &file_path,
      &mut asset_kind,
      &mut content,
      &mut asset_dependencies,
      &mut linking_symbols,
      &mut bundle_behavior,
    );

    transformer.transform(&mut mutable_asset, &c.config)?;
    drop(mutable_asset);

    // If the file type and pattern changes restart transformers
    if asset_kind != original_asset_kind {
      file_path.set_extension(asset_kind);

      let (new_pattern, new_transformers) = c.plugins.transformers.get(&file_path)?;
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

  Ok(TransformerPipelineResult {
    name: file_path
      .file_stem()
      .unwrap_or_default()
      .to_str()
      .unwrap_or_default()
      .to_string(),
    kind: file_path
      .extension()
      .unwrap_or_default()
      .to_str()
      .unwrap_or_default()
      .to_string(),
    content,
    linking_symbols,
    bundle_behavior,
    dependencies: asset_dependencies,
  })
}
