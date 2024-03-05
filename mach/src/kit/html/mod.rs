use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

use html5ever::parse_document;
use html5ever::serialize::serialize;
use html5ever::serialize::SerializeOpts;
use html5ever::tendril::TendrilSink;
use html5ever::Attribute;
use html5ever::LocalName;
use html5ever::Namespace;
use html5ever::QualName;
use markup5ever_rcdom::Handle;
use markup5ever_rcdom::Node;
use markup5ever_rcdom::NodeData;
use markup5ever_rcdom::RcDom;
use markup5ever_rcdom::SerializableHandle;

/*
{
  "407c55513cdc22a": "e14bb8092cf1ed9.css",
  "cc33a425931bc8f": "index.e266a5c3966ebec.js",
  "db89c592f62b063": "index.html"
}
*/
pub struct CreateElementOptions<'a> {
  pub tag_name: &'a str,
  pub attributes: &'a [(&'a str, &'a str)],
}

pub fn create_element(options: CreateElementOptions) -> Rc<Node> {
  let mut attrs = Vec::<Attribute>::new();

  for (attribute_key, attribute_value) in options.attributes {
    attrs.push(Attribute {
      name: QualName {
        prefix: None,
        ns: Namespace::from(""),
        local: LocalName::from(*attribute_key),
      },
      value: From::from(*attribute_value),
    })
  }

  let element = NodeData::Element {
    name: QualName {
      prefix: None,
      ns: Namespace::from("http://www.w3.org/1999/xhtml"),
      local: LocalName::from(options.tag_name),
    },
    attrs: RefCell::new(attrs),
    template_contents: RefCell::new(None),
    mathml_annotation_xml_integration_point: false,
  };

  return Node::new(element);
}

pub fn set_attribute(
  source_node: &mut Handle,
  attribute_key: &str,
  attribute_value: &str,
) -> bool {
  match source_node.data {
    NodeData::Element { ref attrs, .. } => {
      for attr in attrs.borrow_mut().iter_mut() {
        if attr.name.local.to_string() != *attribute_key {
          continue;
        }

        attr.value = From::from(attribute_value);
        return true;
      }
    }
    _ => {}
  }
  return false;
}

pub fn get_attribute(
  source_node: &Handle,
  attribute_key: &str,
) -> Option<String> {
  match source_node.data {
    NodeData::Element { ref attrs, .. } => {
      for attr in attrs.borrow_mut().iter_mut() {
        if attr.name.local.to_string() != *attribute_key {
          continue;
        }

        return Some(attr.value.to_string());
      }
    }
    _ => {}
  }
  return None;
}

pub fn get_tag_name(source_node: &Handle) -> Result<String, String> {
  match source_node.data {
    NodeData::Element { ref name, .. } => {
      return Ok(name.local.to_string());
    }
    _ => {}
  };
  return Err("Could not find tag name".to_string());
}
pub struct QuerySelectorOptions {
  pub tag_name: Option<String>,
  pub attribute: Option<(String, Option<String>)>,
}

pub fn query_selector(
  source_node: &Handle,
  selector: QuerySelectorOptions,
) -> Option<Handle> {
  let mut nodes = Vec::<Handle>::from([source_node.clone()]);

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
        if let Some(tag_name) = &selector.tag_name {
          if name.local.to_string() != *tag_name {
            continue;
          }
        }
        if let Some((attribute_key, attribute_value)) = &selector.attribute {
          for attr in attrs.borrow_mut().iter_mut() {
            if attr.name.local.to_string() != *attribute_key {
              continue;
            }

            if let Some(attribute_value) = &attribute_value {
              if attr.value.to_string() != *attribute_value {
                continue;
              }
            }
          }
        }
        return Some(node.clone());
      }
      _ => {}
    }
  }

  return None;
}

pub fn query_selector_all(
  source_node: &Handle,
  selector: QuerySelectorOptions,
) -> Vec<Handle> {
  let mut nodes = Vec::<Handle>::from([source_node.clone()]);
  let mut found = Vec::<Handle>::new();

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
        if let Some(tag_name) = &selector.tag_name {
          if name.local.to_string() != *tag_name {
            continue;
          }
        }
        if let Some((attribute_key, attribute_value)) = &selector.attribute {
          for attr in attrs.borrow_mut().iter_mut() {
            if attr.name.local.to_string() != *attribute_key {
              continue;
            }

            if let Some(attribute_value) = &attribute_value {
              if attr.value.to_string() != *attribute_value {
                continue;
              }
            }
          }
        }
        found.push(node.clone());
      }
      _ => {}
    }
  }

  return found;
}
