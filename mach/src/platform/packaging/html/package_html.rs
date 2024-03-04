use std::path::PathBuf;

use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::Handle;
use markup5ever_rcdom::NodeData;
use markup5ever_rcdom::RcDom;
use html5ever::serialize::serialize;
use html5ever::serialize::SerializeOpts;
use markup5ever_rcdom::SerializableHandle;

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
  _config: &public::Config,
  asset_map: &mut AssetMap,
  dependency_map: &DependencyMap,
  asset_graph: &AssetGraph,
  _bundles: &Bundles,
  bundle_graph: &BundleGraph,
  outputs: &mut Outputs,
  bundle: &Bundle,
  bundle_manifest: &BundleManifest,
) {
  let entry_asset = bundle.entry_asset.as_ref().unwrap();
  let Some(dependencies) = asset_graph.get_dependencies(&entry_asset) else {
    return;
  };
  if dependencies.len() == 0 {
    return;
  }
  let Some(asset) = asset_map.get_mut(&entry_asset) else {
    panic!("could not find asset")
  };

  let dom = parse_document(RcDom::default(), Default::default())
    .from_utf8()
    .read_from(&mut asset.content.as_slice())
    .unwrap();

  let mut nodes = Vec::<Handle>::from([dom.document.clone()]);

  while let Some(node) = nodes.pop() {
    for child in node.children.borrow().iter() {
      nodes.push(child.clone());
    }

    match node.data {
      NodeData::Element {
        ref name,
        ref attrs,
        ..
      } => {
        if name.local.to_string() != "script" {
          continue;
        }
        for attr in attrs.borrow_mut().iter_mut() {
          if attr.name.local.to_string() != "src" {
            continue;
          }
          let specifier = attr.value.to_string();

          let (dependency_id, _) = 'block: {
            for (dependency_id, dependency) in &dependency_map.dependencies {
              if dependency.specifier == *specifier {
                break 'block (dependency_id, dependency);
              }
            }
            panic!(
              "Could not find dependency for specifier\n  {}",
              specifier
            );
          };

          let bundle_id = bundle_graph.get(dependency_id).unwrap();
          let file_path = bundle_manifest.get(bundle_id).unwrap();

          attr.value = From::from(file_path.clone());
          break;
        }
      }
      _ => {}
    }
  }

  let document: SerializableHandle = dom.document.clone().into();
  let mut output = Vec::<u8>::new();
  serialize(&mut output, &document, SerializeOpts::default()).unwrap();
  outputs.push(Output{
    content: output,
    filepath: PathBuf::from(&bundle.name),
})
  
}
