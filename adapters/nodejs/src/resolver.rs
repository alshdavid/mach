use std::fmt::Debug;
use std::sync::Arc;

use async_trait::async_trait;
use libmach::Resolver;
use libmach::Dependency;
use libmach::ResolveResult;

use crate::engine::requests::LoadPluginRequest;
use crate::engine::requests::RunResolverRequest;
use crate::engine::requests::RunResolverResponse;
use crate::engine::NodejsConnection;

pub struct NodejsResolver {
  specifier: String,
  plugin_key: String,
  node_adapter: Arc<NodejsConnection>,
}

impl NodejsResolver {
  pub async fn new(
    node_adapter: Arc<NodejsConnection>,
    specifier: &str,
  ) -> Result<Self, String> {
    let plugin_key = snowflake::ProcessUniqueId::new().to_string();

    let req = LoadPluginRequest {
      plugin_key: plugin_key.clone(),
      specifier: specifier.to_string(),
    };

    node_adapter.send_all("load_plugin", &req).await?;

    return Ok(NodejsResolver {
      specifier: specifier.to_string(),
      node_adapter,
      plugin_key,
    });
  }
}

#[async_trait]
impl Resolver for NodejsResolver {
  async fn resolve(
    &self,
    dependency: &Dependency,
  ) -> Result<Option<ResolveResult>, String> {
    let req = RunResolverRequest {
      plugin_key: self.plugin_key.clone(),
      dependency: dependency.clone(),
    };
    let result: RunResolverResponse = self.node_adapter.send("run_resolver", &req).await.unwrap();
    if let Some(file_path) = result.file_path {
      return Ok(Some(ResolveResult { file_path }));
    }
    return Ok(None);
  }
}

impl Debug for NodejsResolver {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    f.write_str(&format!("ResolverNodeJs({})", self.specifier))
  }
}

