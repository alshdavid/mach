use libmach::AssetGraph;
use libmach::AssetMap;
use libmach::Bundle;
use libmach::BundleGraph;
use libmach::BundleMap;
use libmach::DependencyMap;
use libmach::MachConfig;

/// This will create a single JavaScript and CSS bundle.
/// It will create many HTML "bundles"
pub fn bundle_single(
  _config: &MachConfig,
  asset_map: &AssetMap,
  asset_graph: &AssetGraph,
  bundles: &mut BundleMap,
  bundle_graph: &mut BundleGraph,
  dependency_map: &DependencyMap,
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
      js_bundle.add_asset(&asset);
    }

    if asset.kind == "css" {
      css_bundle.add_asset(&asset);
    }

    if asset.kind == "html" {
      let mut bundle = Bundle {
        kind: "html".to_string(),
        ..Bundle::default()
      };
      bundle.set_entry_asset(&asset);
      bundle.add_asset(&asset);
      html_bundles.push(bundle);
    }
  }

  if css_bundle.assets.len() > 0 {
    css_bundle.name = format!("{}.css", css_bundle.content_hash());

    for (_, (asset_id, _)) in &css_bundle.assets {
      let Some(dependencies) = asset_graph.get_dependencies(&asset_id) else {
        continue;
      };

      for dependency in dependencies {
        bundle_graph.insert(dependency.0.clone(), css_bundle.id.clone());
      }
    }
  }

  if js_bundle.assets.len() > 0 {
    js_bundle.name = format!("{}.js", js_bundle.content_hash());

    for (_, (asset_id, _)) in &js_bundle.assets {
      let Some(dependencies) = asset_graph.get_dependencies(&asset_id) else {
        continue;
      };

      for dependency in dependencies {
        bundle_graph.insert(dependency.0.clone(), js_bundle.id.clone());
      }
    }
  }

  if js_bundle.assets.len() > 0 && html_bundles.len() == 0 {
    for dependency in dependency_map.dependencies.values() {
      if dependency.is_entry {
        let asset_id = asset_graph.get_asset_id_for_dependency(dependency).unwrap();
        js_bundle.entry_asset.replace(asset_id);
        break;
      }
    }
  }

  for mut html_bundle in html_bundles {
    let entry_asset_id = html_bundle.entry_asset.as_ref().unwrap();
    let entry_asset = asset_map.get(entry_asset_id).unwrap();

    html_bundle.name = format!("{}.html", entry_asset.name);

    for (_, (asset_id, _)) in &html_bundle.assets {
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
