use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use crate::public::Resolver;
use crate::public::Transformer;

pub type PluginContainerSync = Arc<PluginContainer>;

#[derive(Clone, Default, Debug)]
pub struct PluginContainer {
  pub resolvers: Vec<Arc<dyn Resolver>>,
  pub transformers: TransformerMap,
}

#[derive(Clone, Default, Debug)]
pub struct TransformerMap {
  pub transformers: HashMap<String, Vec<Arc<dyn Transformer>>>,
}

impl TransformerMap {
  pub fn get(
    &self,
    file_target: &Path,
  ) -> anyhow::Result<(String, &Vec<Arc<dyn Transformer>>)> {
    let Some(basename) = file_target.file_name() else {
      anyhow::bail!("Unable to get basename");
    };
    let Some(basename) = basename.to_str() else {
      anyhow::bail!("Unable to get basename");
    };
    for (pattern, transformers) in &self.transformers {
      if glob_match::glob_match(&pattern, basename) {
        return Ok((pattern.clone(), transformers));
      }
    }
    anyhow::bail!("No transformer found {:?}", file_target)
  }
}
