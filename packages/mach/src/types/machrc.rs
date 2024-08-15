use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Machrc {
  pub resolvers: Option<Vec<String>>,
  pub transformers: Option<HashMap<String, Vec<String>>>,
}

impl Default for Machrc {
  fn default() -> Self {
    Self {
      resolvers: Some(vec!["mach:resolver".to_string()]),
      transformers: Some(HashMap::from_iter([
        (
          "*.{js,mjs,jsm,jsx,es6,cjs,ts,tsx}".to_string(),
          vec!["mach:transformer/javascript".to_string()],
        ),
        (
          "*.json".to_string(),
          vec!["mach:transformer/json".to_string()],
        ),
        (
          "*.css".to_string(),
          vec!["mach:transformer/css".to_string()],
        ),
        (
          "*.html".to_string(),
          vec!["mach:transformer/html".to_string()],
        ),
        (
          "*.{svg,png,json,gif,woff2,woff,txt}".to_string(),
          vec!["mach:transformer/drop".to_string()],
        ),
      ])),
    }
  }
}
