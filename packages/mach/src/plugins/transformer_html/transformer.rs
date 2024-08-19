use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::Handle;
use markup5ever_rcdom::NodeData;
use markup5ever_rcdom::RcDom;

use crate::types::BundleBehavior;
use crate::types::DependencyOptions;
use crate::types::DependencyPriority;
use crate::types::MachConfig;
use crate::types::MutableAsset;
use crate::types::SpecifierType;
use crate::types::Transformer;

#[derive(Debug)]
enum DocumentLink {
  Script {
    src: Option<String>,
  },
  Link {
    href: Option<String>,
  },
}

#[derive(Debug)]
pub struct TransformerHtml {}

impl Transformer for TransformerHtml {
  fn transform(
    &self,
    asset: &mut MutableAsset,
    _config: &MachConfig,
  ) -> anyhow::Result<()> {
    let code = asset.get_code();
    let Ok(links) = get_script_src_attrs(&code) else {
      anyhow::bail!("Unable process HTML".to_string())
    };

    for link in links {
      let mut options = DependencyOptions {
        specifier: Default::default(),
        specifier_type: SpecifierType::ESM,
        priority: DependencyPriority::Lazy,
        resolve_from: asset.file_path.to_path_buf(),
        linking_symbol: Default::default(),
        bundle_behavior: BundleBehavior::Inline,
      };

      match link {
        DocumentLink::Script {
          src,
        } => {
          let Some(src) = src else {
            continue;
          };
          options.specifier = src;
        }

        DocumentLink::Link { href } => {
          let Some(href) = href else {
            continue;
          };
          options.specifier = href;
        }
      };

      asset.add_dependency(options);
    }

    return Ok(());
  }
}

fn get_script_src_attrs(html: &str) -> Result<Vec<DocumentLink>, ()> {
  let mut links = Vec::<DocumentLink>::new();

  let dom = parse_document(RcDom::default(), Default::default())
    .from_utf8()
    .read_from(&mut html.as_bytes())
    .unwrap();

  walk(&dom.document, &mut links);

  return Ok(links);
}

fn walk(
  handle: &Handle,
  links: &mut Vec<DocumentLink>,
) {
  let node = handle;
  match node.data {
    NodeData::Element {
      ref name,
      ref attrs,
      ..
    } => {
      if name.local.to_string() == "script" {
        let mut src = None;

        for attr in attrs.borrow().iter() {
          match attr.name.local.to_string().as_str() {
            "src" => src.replace(attr.value.to_string()),
            _ => None,
          };
        }

        links.push(DocumentLink::Script {
          src,
        });
      }

      if name.local.to_string() == "link" {
        let mut href = None;

        for attr in attrs.borrow().iter() {
          match attr.name.local.to_string().as_str() {
            "href" => href.replace(attr.value.to_string()),
            _ => None,
          };
        }

        links.push(DocumentLink::Link {
          href,
        });
      }
    }
    _ => {}
  }

  for child in node.children.borrow().iter() {
    walk(child, links);
  }
}
