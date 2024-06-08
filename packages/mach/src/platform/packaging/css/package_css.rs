use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;

use crate::public::AssetMapSync;
use crate::public::Bundle;
use crate::public::Output;
use crate::public::Outputs;

pub fn package_css(
  asset_map: AssetMapSync,
  outputs: Arc<RwLock<Outputs>>,
  bundle: Bundle,
) {
  let mut bundle_content = String::new();

  for (_, (asset_id, _)) in &bundle.assets {
    let mut asset_map = asset_map.write().unwrap();
    let asset = asset_map.get_mut(&asset_id).unwrap();
    let contents = std::mem::take(&mut asset.content);
    bundle_content.push_str(&String::from_utf8(contents).unwrap())
  }

  outputs.write().unwrap().push(Output {
    content: bundle_content.as_bytes().to_vec(),
    filepath: PathBuf::from(&bundle.name),
  });
}
