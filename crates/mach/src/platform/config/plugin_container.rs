use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use libmach::Resolver;
use libmach::Transformer;

pub type PluginContainerSync = Arc<PluginContainer>;

#[derive(Default, Debug)]
pub struct PluginContainer {
  pub resolvers: Vec<Box<dyn Resolver>>,
  pub transformers: TransformerMap,
}

impl PluginContainer {
  pub fn to_sync(&mut self) -> PluginContainerSync {
    Arc::new(std::mem::take(self))
  }
}

#[derive(Default, Debug)]
pub struct TransformerMap {
  pub transformers: HashMap<String, Vec<Box<dyn Transformer>>>,
}

impl TransformerMap {
  pub fn get(
    &self,
    file_target: &TransformerTarget,
  ) -> Result<(String, &Vec<Box<dyn Transformer>>), String> {
    for (pattern, transformers) in &self.transformers {
      if glob_match::glob_match(&pattern, &file_target.file_name) {
        return Ok((pattern.clone(), transformers));
      }
    }
    return Err(format!("No transformer found {:?}", file_target.file_path,));
  }
}

pub struct TransformerTarget<'a> {
  pub file_name: String,
  pub file_stem: String,
  pub file_extension: String,
  pub file_path: &'a Path,
}

impl<'a> TransformerTarget<'a> {
  pub fn new(path: &'a Path) -> Self {
    let mut target = TransformerTarget {
      file_name: String::new(),
      file_stem: String::new(),
      file_extension: String::new(),
      file_path: path,
    };

    if let Some(file_extension) = path.extension() {
      target.file_extension = file_extension.to_str().unwrap().to_string();
    };

    let Some(file_name) = path.file_name() else {
      panic!();
    };

    if target.file_extension == "" {
      target.file_stem = file_name.to_str().unwrap().to_string();
      target.file_name = target.file_stem.clone();
    } else {
      target.file_stem = file_name
        .to_str()
        .unwrap()
        .to_string()
        .replace(&format!(".{}", target.file_extension), "");
      target.file_name = format!("{}.{}", target.file_stem, target.file_extension);
    }

    return target;
  }

  pub fn update(
    &mut self,
    file_extension: &str,
  ) {
    self.file_extension = file_extension.to_string();
    if file_extension != "" {
      self.file_name = format!("{}.{}", self.file_stem, self.file_extension);
    }
  }
}
