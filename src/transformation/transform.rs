use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use swc_core::common::SourceMap;

use crate::default_plugins::resolver::resolve;
use crate::default_plugins::transformers::javascript::JavaScriptTransformer;
use crate::platform::Container;
use crate::public;
use crate::public::Asset;
use crate::public::AssetId;
use crate::public::AssetMap;
use crate::public::Dependency;
use crate::public::DependencyKind;
use crate::public::DependencyMap;
use crate::public::Transformer;
use crate::public::TransformerContext;
use crate::public::UnknownAsset;

type ImportSpecifier = String;

pub fn transform(
  config: &public::Config,
  asset_map_ref: &mut Container<AssetMap>,
  dependency_map_ref: &mut Container<DependencyMap>,
  source_map: Arc<SourceMap>,
) -> Result<(), String> {
  let mut asset_map = asset_map_ref.take();
  let mut dependency_map = dependency_map_ref.take();

  let asset_filepath_reference = HashMap::<PathBuf, AssetId>::new();

  let mut queue = Vec::<(AssetId, (ImportSpecifier, DependencyKind))>::from(vec![(
    AssetId::default(),
    (
      config.entry_point.to_str().unwrap().to_string(),
      DependencyKind::Static,
    ),
  )]);

  let mut transformers = Vec::<Box<dyn Transformer>>::new();
  transformers.push(Box::new(JavaScriptTransformer{}));

  'main_loop: while let Some((parent_asset_id, (import_specifier, dependency_kind))) = queue.pop() {
    // Get filepath to parent asset
    let Some(parent_asset_path) = get_parent_file_path(&asset_map, &config, &parent_asset_id)
    else {
      return Err(format!(
        "Could not find parent with ID: {:?}",
        parent_asset_id
      ));
    };

    // Get filepath to current asset
    let Ok(new_asset_absolute_path) = resolve(&parent_asset_path, &import_specifier) else {
      return Err(format!(
        "Could not resolve specifier {} from {:?}",
        import_specifier, parent_asset_path
      ));
    };

    // If the asset already exists then link the dependency and continue on
    if let Some(existing_asset_id) = asset_filepath_reference.get(&new_asset_absolute_path) {
      dependency_map.insert(
        existing_asset_id,
        Dependency {
          parent_asset_id,
          target_asset_id: existing_asset_id.clone(),
          import_specifier,
          kind: dependency_kind,
        },
      );
      continue;
    }

    // Read the contents of the asset
    let Ok(asset_contents) = fs::read_to_string(&new_asset_absolute_path) else {
      return Err(format!("File Read Error: {:?}", new_asset_absolute_path));
    };

    // Transformer pipeline
    let mut new_asset = Asset::Unknown(UnknownAsset::new(
      &config.project_root, 
      &new_asset_absolute_path, 
      &asset_contents,
    ));

    let mut dependencies = Vec::<(String, DependencyKind)>::new();
    let ctx = TransformerContext::new(
        config,
        source_map.clone(),
        &mut dependencies,
    );

    for transformer in transformers.iter() {
      match transformer.transform(&ctx, &mut new_asset) {
        public::TransformResult::Convert(converted_asset) => {
          new_asset = converted_asset;
        },
        public::TransformResult::End => {
          break 'main_loop;
        },
        public::TransformResult::Next => {
          continue;
        },
        public::TransformResult::Err(err) => {
          return Err(err);
        },
      };
    }

    let Asset::JavaScript(new_asset) = new_asset else { continue; };

    dependency_map.insert(
      &parent_asset_id.clone(),
      Dependency {
        parent_asset_id,
        target_asset_id: new_asset.id.clone(),
        import_specifier,
        kind: dependency_kind,
      },
    );

    while let Some(dependencies) = dependencies.pop() {
      queue.push((
        new_asset.id.clone(),
        dependencies,
      ));
    }

    asset_map.insert(Asset::JavaScript(new_asset));
  }

  println!("Asset {}", asset_map.len());

  asset_map_ref.insert(asset_map);
  dependency_map_ref.insert(dependency_map);
  Ok(())
}

fn get_parent_file_path(
  asset_map: &AssetMap,
  config: &public::Config,
  asset_id: &AssetId,
) -> Option<PathBuf> {
  if asset_map.len() == 0 {
    return Some(config.project_root.clone());
  }
  let Some(parent_asset) = asset_map.get(asset_id) else {
    return None;
  };
  return Some(parent_asset.file_path());
}
