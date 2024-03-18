use async_trait::async_trait;
use libmach::Adapter;
use libmach::AdapterOption;
use libmach::AdapterOptions;
use libmach::Resolver;
use libmach::Transformer;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use crate::engine::native_resolve;
use crate::engine::NodejsConnection;
use crate::resolver::NodejsResolver;
use crate::transformer::NodejsTransformer;

pub struct NodejsAdapter {
  pub nodejs_connection: Arc<NodejsConnection>,
}

#[async_trait]
impl Adapter for NodejsAdapter {
  async fn get_resolver(
    &self,
    config: AdapterOptions,
  ) -> Result<Box<dyn Resolver>, String> {
    let Some(specifier) = &config.get("specifier") else {
      return Err("No specifier".to_string());
    };
    let AdapterOption::String(specifier) = specifier else {
      return Err("Invalid specifier".to_string());
    };
    let resolver = NodejsResolver::new(
      self.nodejs_connection.clone(),
      specifier,
    ).await?;

    return Ok(Box::new(resolver));
  }

  async fn get_transformer(
    &self,
    _: AdapterOptions,
  ) -> Result<Box<dyn Transformer>, String> {
    return Ok(Box::new(NodejsTransformer {}));
  }

  async fn resolve_specifier(&self, from_path: &Path, specifier: &str) -> Result<PathBuf, String> {
    return native_resolve(from_path, specifier).await;
  }
}
