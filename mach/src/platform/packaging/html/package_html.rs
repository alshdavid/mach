use std::path::PathBuf;
use std::sync::Arc;

use html5ever::parse_document;
use html5ever::serialize::serialize;
use html5ever::serialize::SerializeOpts;
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::RcDom;
use markup5ever_rcdom::SerializableHandle;
use std::sync::Mutex;

use crate::kit::html;
use crate::public;
use crate::public::AssetGraph;
use crate::public::AssetMap;
use crate::public::Bundle;
use crate::public::BundleGraph;
use crate::public::BundleManifest;
use crate::public::Bundles;
use crate::public::DependencyMap;
use crate::public::Output;
use crate::public::Outputs;

pub fn package_html(
  _config: Arc<public::Config>,
  asset_map: Arc<Mutex<AssetMap>>,
  dependency_map: Arc<DependencyMap>,
  asset_graph: Arc<AssetGraph>,
  bundles: Arc<Bundles>,
  bundle_graph: Arc<BundleGraph>,
  outputs: Arc<Mutex<Outputs>>,
  bundle: Bundle,
  bundle_manifest: Arc<BundleManifest>,
) {
  let entry_asset = bundle.entry_asset.as_ref().unwrap();
  let Some(dependencies) = asset_graph.get_dependencies(&entry_asset) else {
    return;
  };
  if dependencies.len() == 0 {
    return;
  }
  let asset_file_path_rel = entry_asset.clone();
  
  let asset_content = {
    let mut asset_map = asset_map.lock().unwrap();
    let Some(asset) = asset_map.get_mut(&entry_asset) else {
      panic!("could not find asset")
    };
    std::mem::take(&mut asset.content)
  };

  let dom = parse_document(RcDom::default(), Default::default())
    .from_utf8()
    .read_from(&mut asset_content.as_slice())
    .unwrap();

  let mut script_nodes = html::query_selector_all(
    &dom.document.clone(),
    html::QuerySelectorOptions {
      tag_name: Some("script".to_string()),
      attribute: Some(("src".to_string(), None)),
    },
  );

  for script_node in &mut script_nodes {
    let Some(specifier) = html::get_attribute(&script_node, "src") else {
      continue;
    };

    let Some(dependency) =
      dependency_map.get_dependency_for_specifier(&asset_file_path_rel, &specifier)
    else {
      continue;
    };

    let bundle_id = bundle_graph.get(&dependency.id).unwrap();
    let file_path = bundle_manifest.get(bundle_id).unwrap();

    html::set_attribute(script_node, "src", file_path);
  }

  let css_home = 'block: {
    for tag_name in ["head", "body"] {
      let Some(css_home) = html::query_selector(
        &dom.document,
        html::QuerySelectorOptions {
          tag_name: Some(tag_name.to_string()),
          attribute: None,
        },
      ) else {
        continue;
      };
      break 'block css_home;
    }
    break 'block dom.document.clone();
  };

  for bundle in bundles.iter() {
    if bundle.kind == "css" {
      css_home
        .children
        .borrow_mut()
        .push(html::create_element(html::CreateElementOptions {
          tag_name: "link",
          attributes: &[("rel", "stylesheet"), ("href", &bundle.name)],
        }));
    }
  }

  let document: SerializableHandle = dom.document.clone().into();
  let mut output = Vec::<u8>::new();
  serialize(&mut output, &document, SerializeOpts::default()).unwrap();

  outputs.lock().unwrap().push(Output {
    content: output,
    filepath: PathBuf::from(&bundle.name),
  });
}
