use std::fmt::Debug;
use std::sync::Arc;

use async_trait::async_trait;

use crate::adapters::node_js::requests::LoadPluginRequest;
use crate::adapters::node_js::requests::RunTransformerRequest;
use crate::adapters::node_js::requests::RunTransformerResponse;
use crate::adapters::node_js::NodeAdapter;
use crate::public::Config;
use crate::public::MutableAsset;
use crate::public::Transformer;

pub struct TransformerNodeJs {
  specifier: String,
  plugin_key: String,
  node_adapter: Arc<NodeAdapter>,
}

impl TransformerNodeJs {
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

    return TransformerNodeJs {
      specifier: specifier.to_string(),
      node_adapter,
      plugin_key,
    };
  }
}

#[async_trait]
impl Transformer for TransformerNodeJs {
  async fn transform(
    &self,
    asset: &mut MutableAsset,
    config: &Config,
  ) -> Result<(), String> {
    let req = RunTransformerRequest {
      plugin_key: self.plugin_key.clone(),
      file_path: asset.file_path.clone(),
      code: asset.get_code().clone(),
      kind: asset.kind.clone(),
      config: config.clone(),
    };

    let response: RunTransformerResponse = self
      .node_adapter
      .send("run_transformer", &req)
      .await
      .unwrap();

    if !response.updated {
      return Ok(());
    }

    for dependency in response.dependencies {
      asset.add_dependency(dependency);
    }

    asset.set_code(&response.code);

    return Ok(());
  }
}

impl Debug for TransformerNodeJs {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    f.write_str(&format!("TransformerNodeJs({})", self.specifier))
  }
}
