use std::sync::Arc;

use async_trait::async_trait;

use crate::public::Dependency;
use crate::public::ResolveResult;
use crate::public::Resolver;
use crate::adapters::node_js::NodeAdapter;

use super::requests::LoadResolverRequest;
use super::requests::RunResolverRequest;
use super::requests::RunResolverResponse;

#[derive(Debug)]
pub struct ResolverNodeJs {
  resolver_key: String,
  node_adapter: Arc<NodeAdapter>,
}

impl ResolverNodeJs {
  pub async fn new(
    node_adapter: Arc<NodeAdapter>,
    specifier: &str,
  ) -> Self {
    let req = LoadResolverRequest {
      specifier: specifier.to_string(),
    };

    node_adapter.send_all("load_resolver", &req).await.unwrap();

    return ResolverNodeJs {
      node_adapter,
      resolver_key: specifier.to_string(),
    };
  }
}

#[async_trait]
impl Resolver for ResolverNodeJs {
  async fn resolve(&self, dependency: &Dependency) -> Result<Option<ResolveResult>, String> {
    let req = RunResolverRequest {
      resolver_key: self.resolver_key.clone(),
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
