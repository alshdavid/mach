use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildReport {
  pub bundle_manifest: HashMap<String, String>,
  pub entries: HashMap<String, String>,
}
