use crate::public::Config;
use crate::public::DependencyOptions;
use crate::public::MutableAsset;
use crate::public::Transformer;

use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::Handle;
use markup5ever_rcdom::NodeData;
use markup5ever_rcdom::RcDom;

#[derive(Debug)]
pub struct DefaultTransformerHtml {}

impl Transformer for DefaultTransformerHtml {
  fn transform(
    &self,
    asset: &mut MutableAsset,
    _config: &Config,
  ) -> Result<(), String> {
    let code = asset.get_code();
    let Ok(script_specifiers) = get_script_src_attrs(&code) else {
      return Err("Unable process HTML".to_string());
    };

    for script_specifier in script_specifiers {
      asset.add_dependency(DependencyOptions {
        specifier: script_specifier,
        specifier_type: crate::public::SpecifierType::ESM,
        priority: crate::public::DependencyPriority::Lazy,
        resolve_from: asset.file_path.clone(),
        imported_symbols: vec![crate::public::ImportSymbol::Namespace("".to_string())],
        bundle_behavior: crate::public::BundleBehavior::Default,
      });
    }

    return Ok(());
  }
}

fn get_script_src_attrs(html: &str) -> Result<Vec<String>, ()> {
  let mut script_src_attrs = Vec::<String>::new();

  let dom = parse_document(RcDom::default(), Default::default())
    .from_utf8()
    .read_from(&mut html.as_bytes())
    .unwrap();

  walk(&dom.document, &mut script_src_attrs);

  return Ok(script_src_attrs);
}

fn walk(
  handle: &Handle,
  attrs_list: &mut Vec<String>,
) {
  let node = handle;
  match node.data {
    NodeData::Element {
      ref name,
      ref attrs,
      ..
    } => {
      if name.local.to_string() == "script" {
        for attr in attrs.borrow().iter() {
          if attr.name.local.to_string() == "src" {
            attrs_list.push(attr.value.to_string());
            break;
          }
        }
      }
    }
    _ => {}
  }

  for child in node.children.borrow().iter() {
    walk(child, attrs_list);
  }
}
