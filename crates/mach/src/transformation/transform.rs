use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use swc_core::common::SourceMap;

use crate::default_plugins::resolver::resolve;
use crate::default_plugins::transformers::javascript::transformer;
use crate::public;
use crate::public::Asset;
use crate::public::AssetId;
use crate::public::AssetMap;
use crate::public::Dependency;
use crate::public::DependencyKind;
use crate::public::DependencyMap;

type ImportSpecifier = String;

pub fn transform(
  config: &public::Config,
  asset_map: &mut AssetMap,
  dependency_map: &mut DependencyMap,
  source_map: Arc<SourceMap>,
) -> Result<(), String> {
  let asset_filepath_reference = HashMap::<PathBuf, AssetId>::new();

  let entry_specifier = ImportSpecifier::from(config.entry_point.to_str().unwrap());
  let first_job = (
    AssetId::default(),
    (entry_specifier, DependencyKind::Static),
  );
  let mut queue = Vec::<(AssetId, (ImportSpecifier, DependencyKind))>::from(vec![first_job]);

  while let Some((parent_asset_id, (import_specifier, dependency_kind))) = queue.pop() {
    // Get filepath to parent asset
    let parent_asset_path = 'block: {
      // If it's the first asset then the parent is the root path
      if asset_map.len() == 0 {
        break 'block config.project_root.clone();
      }
      // Use the asset if we find the parent's ID
      if let Some(parent_asset) = asset_map.get(&parent_asset_id) {
        break 'block parent_asset.file_path.clone();
      }
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

    // Parse JavaScript
    let Ok((program, mut dependencies)) = transformer(
      &new_asset_absolute_path,
      &asset_contents,
      source_map.clone(),
      config,
    ) else {
      return Err(format!("File Parse Error: {:?}", new_asset_absolute_path));
    };

    // Create and commit new Asset
    let new_asset = Asset::new(
      &config.project_root,
      &new_asset_absolute_path,
      &asset_contents,
      program,
    );

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
        (dependencies.specifier, dependencies.kind),
      ));
    }

    asset_map.insert(new_asset);
  }

  Ok(())
}
