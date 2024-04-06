use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;

use html5ever::parse_document;
use html5ever::serialize::serialize;
use html5ever::serialize::SerializeOpts;
use html5ever::tendril::TendrilSink;
use crate::public::AssetGraphSync;
use crate::public::AssetMapSync;
use crate::public::BundleGraphSync;
use crate::public::BundleMapSync;
use crate::public::DependencyMapSync;
use markup5ever_rcdom::RcDom;
use markup5ever_rcdom::SerializableHandle;
use swc_core::common::SourceMap;

use crate::public::Bundle;
use crate::public::BundleManifest;
use crate::public::Output;
use crate::public::Outputs;

use crate::kit::html;
use crate::kit::swc;

use super::super::javascript::runtime_factory::RuntimeFactory;

pub fn package_html(
  asset_map: AssetMapSync,
  asset_graph: AssetGraphSync,
  dependency_map: DependencyMapSync,
  bundle_map: BundleMapSync,
  bundle_graph: BundleGraphSync,
  outputs: Arc<RwLock<Outputs>>,
  bundle: Bundle,
  bundle_manifest: &BundleManifest,
  js_runtime_factory: Arc<RuntimeFactory>,
) {
  let asset_graph = asset_graph.read().unwrap();
  let dependency_map = dependency_map.read().unwrap();
  let bundle_map = bundle_map.read().unwrap();
  let bundle_graph = bundle_graph.read().unwrap();

  let entry_asset = bundle.entry_asset.as_ref().unwrap();
  let Some(dependencies) = asset_graph.get_dependencies(&entry_asset) else {
    return;
  };
  if dependencies.len() == 0 {
    return;
  }
  let asset_id = entry_asset.clone();

  let asset_content = {
    let mut asset_map = asset_map.write().unwrap();
    let Some(asset) = asset_map.get_mut(&entry_asset) else {
      panic!("could not find asset")
    };
    std::mem::take(&mut asset.content)
  };

  let dom = parse_document(RcDom::default(), Default::default())
    .from_utf8()
    .read_from(&mut asset_content.as_slice())
    .unwrap();

  let head = html::query_selector(
    &dom.document,
    html::QuerySelectorOptions {
      tag_name: Some("head".to_string()),
      attribute: None,
    },
  );

  let body = html::query_selector(
    &dom.document,
    html::QuerySelectorOptions {
      tag_name: Some("body".to_string()),
      attribute: None,
    },
  );

  let mut script_nodes = html::query_selector_all(
    &dom.document.clone(),
    html::QuerySelectorOptions {
      tag_name: Some("script".to_string()),
      attribute: Some(("src".to_string(), None)),
    },
  );

  if script_nodes.len() > 0 {
    let mut stmts = js_runtime_factory.prelude("PROJECT_HASH");
    stmts.push(js_runtime_factory.manifest(&bundle_manifest).unwrap());
    stmts.push(js_runtime_factory.import_script());
    stmts.extend(js_runtime_factory.prelude_mach_require().into_iter());
    let stmts = js_runtime_factory.wrapper(stmts);
    let js = swc::render_stmts(&vec![stmts], Arc::new(SourceMap::default()));

    let import = html::create_element(html::CreateElementOptions {
      tag_name: "script",
      body: Some(&js),
      ..Default::default()
    });

    if let Some(head) = &head {
      head.children.borrow_mut().push(import);
    } else if let Some(body) = &body {
      body.children.borrow_mut().push(import);
    } else {
      dom.document.children.borrow_mut().push(import);
    }
  }

  for script_node in &mut script_nodes {
    let Some(specifier) = html::get_attribute(&script_node, "src") else {
      continue;
    };

    let Some(dependency) = dependency_map.get_dependency_for_specifier(&asset_id, &specifier)
    else {
      continue;
    };

    let x = asset_graph.get_asset_id_for_dependency(dependency).unwrap();
    let asset = asset_map
      .read()
      .unwrap()
      .get(&x)
      .unwrap()
      .file_path_relative
      .to_str()
      .unwrap()
      .to_string();

    let bundle_id = bundle_graph.get(&dependency.id).unwrap();
    let bundle_hash = bundle_map
      .iter()
      .find(|b| &b.id == bundle_id)
      .unwrap()
      .content_hash();
    let file_path = bundle_manifest.get(&bundle_hash).unwrap();

    html::set_attribute(
      script_node,
      "onload",
      &format!(
        "globalThis['PROJECT_HASH'].mach_require('{}', ['{}'])",
        asset, bundle_hash
      ),
    );
    html::set_attribute(script_node, "src", file_path);
  }

  for bundle in bundle_map.iter() {
    if bundle.kind == "css" {
      let elm = html::create_element(html::CreateElementOptions {
        tag_name: "link",
        attributes: Some(&[("rel", "stylesheet"), ("href", &bundle.name)]),
        ..Default::default()
      });
      if let Some(head) = &head {
        head.children.borrow_mut().push(elm);
      } else if let Some(body) = &body {
        body.children.borrow_mut().push(elm);
      } else {
        dom.document.children.borrow_mut().push(elm);
      }
    }
  }

  let document: SerializableHandle = dom.document.clone().into();
  let mut output = Vec::<u8>::new();
  serialize(&mut output, &document, SerializeOpts::default()).unwrap();

  outputs.write().unwrap().push(Output {
    content: output,
    filepath: PathBuf::from(&bundle.name),
  });
}
