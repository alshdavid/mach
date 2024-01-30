use std::fs;
use std::sync::Arc;

use swc_core::common::SourceMap;

use crate::public;
use crate::public::Bundle;
use crate::public::BundleMap;

use super::render::render;

pub fn emit(
  config: &public::Config,
  bundle_map: &BundleMap,
  source_map: Arc<SourceMap>,
) -> Result<(), String> {
  fs::create_dir_all(&config.dist_dir).unwrap();

  for bundle in bundle_map.iter() {
    let Bundle::JavaScript(bundle) = bundle else {
      continue;
    };
    let rendered = render(&bundle.output, source_map.clone(), config.optimize.clone());
    fs::write(
      config.dist_dir.join(format!("{}.js", bundle.name)),
      rendered,
    )
    .unwrap();
  }
  Ok(())
}
