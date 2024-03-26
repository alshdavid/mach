use std::collections::HashSet;
use std::path::PathBuf;

use crate::public;
use crate::public::AssetGraph;
use crate::public::AssetMap;
use crate::public::Bundle;
use crate::public::BundleGraph;
use crate::public::Bundles;

/// This will create a single JavaScript and CSS bundle.
/// It will create many HTML "bundles"
pub fn bundle_single(
  _config: &public::Config,
  asset_map: &AssetMap,
  asset_graph: &AssetGraph,
  bundles: &mut Bundles,
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
      js_bundle.assets.insert(asset.file_path_relative.clone());
    }

    if asset.kind == "css" {
      css_bundle.assets.insert(asset.file_path_relative.clone());
    }

    if asset.kind == "html" {
      html_bundles.push(Bundle {
        kind: "html".to_string(),
        assets: HashSet::<PathBuf>::from_iter(vec![asset.file_path_relative.clone()]),
        entry_asset: Some(asset.file_path_relative.clone()),
        ..Bundle::default()
      });
    }
  }

  if css_bundle.assets.len() > 0 {
    css_bundle.content_key = css_bundle.generate_id();
    css_bundle.name =
      css_bundle.generate_name(asset_map.get_many(&css_bundle.get_assets()).unwrap());

    for asset_id in &css_bundle.assets {
      let Some(dependencies) = asset_graph.get_dependencies(&asset_id) else {
        continue;
      };

      for dependency in dependencies {
        bundle_graph.insert(dependency.0.clone(), css_bundle.content_key.clone());
      }
    }
  }

  if js_bundle.assets.len() > 0 {
    js_bundle.content_key = js_bundle.generate_id();
    js_bundle.name = js_bundle.generate_name(asset_map.get_many(&js_bundle.get_assets()).unwrap());

    for asset_id in &js_bundle.assets {
      let Some(dependencies) = asset_graph.get_dependencies(&asset_id) else {
        continue;
      };

      for dependency in dependencies {
        bundle_graph.insert(dependency.0.clone(), js_bundle.content_key.clone());
      }
    }
  }

  for mut html_bundle in html_bundles {
    html_bundle.name = html_bundle
      .entry_asset
      .as_ref()
      .unwrap()
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
          bundle_graph.insert(dependency_id.clone(), js_bundle.content_key.clone());
        }
        if asset.kind == "css" {
          bundle_graph.insert(dependency_id.clone(), css_bundle.content_key.clone());
        }
      }
    }

    bundles.push(html_bundle);
  }

  if css_bundle.assets.len() > 0 {
    bundles.push(css_bundle);
  }

  if js_bundle.assets.len() > 0 {
    bundles.push(js_bundle);
  }

  return Ok(());
}
