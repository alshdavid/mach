use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use crate::public::AssetContentMap;
use crate::public::Bundle;
use crate::public::Output;
use crate::public::Outputs;

pub fn package_css(
  asset_content_map: Arc<Mutex<AssetContentMap>>,
  outputs: Arc<Mutex<Outputs>>,
  bundle: Bundle,
) {
  let mut bundle_content = String::new();

  for asset_id in &bundle.assets {
    let mut asset_map = asset_content_map.lock().unwrap();
    let contents = asset_map.bytes.get_mut(asset_id).unwrap();
    bundle_content.push_str(&String::from_utf8(*contents.clone()).unwrap())
  }

  outputs.lock().unwrap().push(Output {
    content: bundle_content.as_bytes().to_vec(),
    filepath: PathBuf::from(&bundle.name),
  });
}
