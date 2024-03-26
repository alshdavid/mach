use std::collections::HashSet;

use crate::public;
use crate::public::AssetGraph;
use crate::public::AssetId;
use crate::public::AssetMap;
use crate::public::Bundle;
use crate::public::BundleGraph;
use crate::public::BundleMap;

/// This will create a single JavaScript and CSS bundle.
/// It will create many HTML "bundles"
pub fn bundle_single(
  _config: &public::Config,
  asset_map: &AssetMap,
  asset_graph: &AssetGraph,
  bundles: &mut BundleMap,
  bundle_graph: &mut BundleGraph,
) -> Result<(), String> {
  let mut css_bundle = Bundle {
    kind: "css".to_string(),
    ..Bundle::default()
  };

  let mut js_bundle = Bundle {
    kind: "js".to_string(),
    ..Bundle::default()
  };

  let mut html_bundles = Vec::<Bundle>::new();

  for asset in asset_map.iter() {
    if asset.kind == "js" {
      js_bundle.assets.insert(asset.id.clone());
    }

    if asset.kind == "css" {
      css_bundle.assets.insert(asset.id.clone());
    }

    if asset.kind == "html" {
      html_bundles.push(Bundle {
        kind: "html".to_string(),
        assets: HashSet::<AssetId>::from_iter(vec![asset.id.clone()]),
        entry_asset: Some(asset.id.clone()),
        ..Bundle::default()
      });
    }
  }

  if css_bundle.assets.len() > 0 {
    css_bundle.name =
      css_bundle.generate_name(asset_map.get_many(&css_bundle.get_assets()).unwrap());

    for asset_id in &css_bundle.assets {
      let Some(dependencies) = asset_graph.get_dependencies(&asset_id) else {
        continue;
      };

      for dependency in dependencies {
        bundle_graph.insert(dependency.0.clone(), css_bundle.id.clone());
      }
    }
  }

  if js_bundle.assets.len() > 0 {
    js_bundle.name = js_bundle.generate_name(asset_map.get_many(&js_bundle.get_assets()).unwrap());

    for asset_id in &js_bundle.assets {
      let Some(dependencies) = asset_graph.get_dependencies(&asset_id) else {
        continue;
      };

      for dependency in dependencies {
        bundle_graph.insert(dependency.0.clone(), js_bundle.id.clone());
      }
    }
  }

  for mut html_bundle in html_bundles {
    let entry_asset_id = html_bundle
      .entry_asset
      .as_ref()
      .unwrap();

    let entry_asset = asset_map.get(entry_asset_id)
      .unwrap();

    html_bundle.name = entry_asset.file_path_absolute
      .file_name()
      .unwrap()
      .to_str()
      .unwrap()
      .to_string();

    for asset_id in &html_bundle.assets {
      let Some(dependencies) = asset_graph.get_dependencies(&asset_id) else {
        continue;
      };

      for (dependency_id, asset_id) in dependencies {
        let asset = asset_map.get(&asset_id).unwrap();
        if asset.kind == "js" {
          bundle_graph.insert(dependency_id.clone(), js_bundle.id.clone());
        }
        if asset.kind == "css" {
          bundle_graph.insert(dependency_id.clone(), css_bundle.id.clone());
        }
      }
    }

    bundles.insert(html_bundle);
  }

  if css_bundle.assets.len() > 0 {
    bundles.insert(css_bundle);
  }

  if js_bundle.assets.len() > 0 {
    bundles.insert(js_bundle);
  }

  return Ok(());
}
