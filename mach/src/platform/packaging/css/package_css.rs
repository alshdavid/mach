use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use crate::public;
use crate::public::AssetContent;
use crate::public::AssetGraph;
use crate::public::AssetMap;
use crate::public::Bundle;
use crate::public::BundleGraph;
use crate::public::BundleManifest;
use crate::public::Bundles;
use crate::public::DependencyMap;
use crate::public::Output;
use crate::public::Outputs;

pub fn package_css(
  _config: Arc<public::Config>,
  asset_map: Arc<Mutex<AssetMap>>,
  _dependency_map: Arc<DependencyMap>,
  _asset_graph: Arc<AssetGraph>,
  _bundles: Arc<Bundles>,
  _bundle_graph: Arc<BundleGraph>,
  outputs: Arc<Mutex<Outputs>>,
  bundle: Bundle,
  _bundle_manifest: Arc<BundleManifest>,
) -> Result<(), String> {
  let mut bundle_content = String::new();

  for asset_id in &bundle.assets {
    let asset_content = {
      let mut asset_map = asset_map.lock().unwrap();
      let asset = asset_map.get_mut(&asset_id).unwrap();
      let contents = asset.get_content()?;
      // Clone in case asset is used in multiple places
      contents.clone() 
    };

    let bytes = match asset_content {
      AssetContent::Bytes(bytes) => bytes,
      _ => return Err("Invalid CSS type".to_string()),
    };
  
    bundle_content.push_str(&String::from_utf8(bytes).unwrap())
  }

  outputs.lock().unwrap().push(Output {
    content: bundle_content.as_bytes().to_vec(),
    filepath: PathBuf::from(&bundle.name),
  });

  return Ok(());
}
