use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Machrc {
  pub is_default: bool,
  pub file_path: PathBuf,
  pub engines: Vec<String>,
  pub resolvers: Option<Vec<String>>,
  pub transformers: Option<HashMap<String, Vec<String>>>,
}

impl Default for Machrc {
  fn default() -> Self {
    Self {
      is_default: true,
      file_path: env::current_exe().unwrap(),
      engines: vec!["mach".to_string()],
      resolvers: Some(vec!["mach:resolver".to_string()]),
      transformers: Some(HashMap::from_iter([
        (
          "*.{js,mjs,jsm,jsx,es6,cjs,ts,tsx}".to_string(),
          vec!["mach:transformer/javascript".to_string()],
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
