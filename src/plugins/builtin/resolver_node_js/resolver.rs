use std::sync::Arc;

use async_trait::async_trait;

use crate::public::Dependency;
use crate::public::ResolveResult;
use crate::public::Resolver;
use crate::adapters::node_js::NodeAdapter;
use crate::adapters::node_js::requests::LoadPluginRequest;
use crate::adapters::node_js::requests::RunResolverRequest;
use crate::adapters::node_js::requests::RunResolverResponse;

#[derive(Debug)]
pub struct ResolverNodeJs {
  plugin_key: String,
  node_adapter: Arc<NodeAdapter>,
}

impl ResolverNodeJs {
  pub async fn new(
    node_adapter: Arc<NodeAdapter>,
    specifier: &str,
  ) -> Self {
    let plugin_key = snowflake::ProcessUniqueId::new().to_string();

    let req = LoadPluginRequest {
      plugin_key: plugin_key.clone(),
      specifier: specifier.to_string(),
    };

    node_adapter.send_all("load_plugin", &req).await.unwrap();

    return ResolverNodeJs {
      node_adapter,
      plugin_key,
    };
  }
}

#[async_trait]
impl Resolver for ResolverNodeJs {
  async fn resolve(&self, dependency: &Dependency) -> Result<Option<ResolveResult>, String> {
    let req = RunResolverRequest {
      plugin_key: self.plugin_key.clone(),
      dependency: dependency.clone(),
    };
    let result: RunResolverResponse = self.node_adapter.send("run_resolver", &req).await.unwrap();
    if let Some(file_path) = result.file_path {
      return Ok(Some(ResolveResult{
        file_path,
      }));
    }
    return Ok(None);
  }
} 
