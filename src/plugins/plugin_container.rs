use std::collections::HashMap;
use std::path::Path;

use crate::public::Resolver;
use crate::public::Transformer;

#[derive(Default, Debug)]
pub struct PluginContainer {
  pub resolvers: Vec<Box<dyn Resolver>>,
  pub transformers: TransformerMap,
}

#[derive(Default, Debug)]
pub struct TransformerMap {
  pub transformers: HashMap<String, Vec<Box<dyn Transformer>>>,
}

impl TransformerMap {
  pub fn get(
    &self,
    file_path: &Path,
  ) -> Option<&Vec<Box<dyn Transformer>>> {
    let file_name = file_path.file_name().unwrap();
    for (pattern, transformers) in &self.transformers {
      if glob_match::glob_match(&pattern, file_name.to_str().unwrap()) {
        return Some(transformers);
      }
    }
    return None;
  }
}
