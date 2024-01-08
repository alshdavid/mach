use std::fs;

use swc_core::common::SourceMap;

use crate::platform::Container;
use crate::public;
use crate::public::AssetMap;
use crate::public::Bundle;
use crate::public::BundleMap;
use crate::public::DependencyMap;
use crate::platform::hash::truncate;

use super::render::render;

pub fn emit(
  config: &public::Config,
  asset_map_ref: &mut Container<AssetMap>,
  dependency_map_ref: &mut Container<DependencyMap>,
  bundle_map_ref: &mut Container<BundleMap>,
  source_map_ref: &mut Container<SourceMap>,
) -> Result<(), String> {
  let asset_map = asset_map_ref.take();
  let dependency_map = dependency_map_ref.take();
  let bundle_map = bundle_map_ref.take();
  let source_map = source_map_ref.take_arc();

  fs::create_dir_all(&config.dist_dir).unwrap();
  
  for bundle in bundle_map.iter() {
    let Bundle::JavaScript(bundle) = bundle else { continue };
    let rendered = render(&bundle.output, source_map.clone());
    fs::write(config.dist_dir.join(format!("{}", truncate(&bundle.name, 15))), rendered).unwrap();
  }

  asset_map_ref.insert(asset_map);
  dependency_map_ref.insert(dependency_map);
  bundle_map_ref.insert(bundle_map);
  source_map_ref.insert_arc(source_map);
  Ok(())
}
