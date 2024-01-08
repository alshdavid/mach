use swc_core::common::Span;
use swc_core::ecma::ast::Script;

use crate::platform::hash::hash_string_sha_256;
use crate::platform::Container;
use crate::platform::hash::truncate;
use crate::public;
use crate::public::Asset;
use crate::public::AssetMap;
use crate::public::Bundle;
use crate::public::BundleMap;
use crate::public::DependencyMap;
use crate::public::JavaScriptBundle;

pub fn bundle(
  config: &public::Config,
  asset_map_ref: &mut Container<AssetMap>,
  dependency_map_ref: &mut Container<DependencyMap>,
  bundle_map_ref: &mut Container<BundleMap>,
) -> Result<(), String> {
  let asset_map = asset_map_ref.take();
  let dependency_map = dependency_map_ref.take();
  let mut bundle_map = bundle_map_ref.take();

  // For now, generate a single javascript bundle
  // Someone smarter than I am can probably figure out a better bundling strategy
  let mut bundle = JavaScriptBundle {
    name: String::new(),
    entry: None,
    assets: vec![],
    output: Script {
      span: Span::default(),
      body: vec![],
      shebang: None,
    },
  };

  let mut hash_calc = String::new();

  // all assets go into this bundle
  for asset in asset_map.iter() {
    let Asset::JavaScript(asset) = asset else {
      continue;
    };
    bundle.assets.push(asset.id.clone());
    hash_calc.push_str(&asset.id);
    hash_calc.push_str(&asset.source_content_hash);
  }

  bundle.name = truncate(&hash_string_sha_256(&hash_calc), 15);

  // find entry asset
  'lookup: for (_, dependencies) in dependency_map.iter() {
    for dependency in dependencies.iter() {
      if dependency.parent_asset_id == "" {
        let asset = asset_map.get(&dependency.target_asset_id).unwrap();
        bundle.entry = Some(asset.id());

        let file_path = asset.file_path();
        let file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();
        let file_extension = file_path.extension().unwrap().to_str().unwrap().to_string();
        bundle.name = format!(
          "{}-{}",
          &file_name[0..file_name.len() - file_extension.len() - 1],
          bundle.name
        );
        break 'lookup;
      }
    }
  }

  if config.optimize {
    bundle.name = format!(
      "{}-min",
      bundle.name
    );
  }

  bundle.assets.sort_by(|a, b| a.cmp(&b));

  // Temporary
  bundle.name = "index".into();

  bundle_map.insert(Bundle::JavaScript(bundle));

  asset_map_ref.insert(asset_map);
  dependency_map_ref.insert(dependency_map);
  bundle_map_ref.insert(bundle_map);
  Ok(())
}
