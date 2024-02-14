use std::{collections::HashMap, path::PathBuf};

use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Machrc {
  pub file_path: PathBuf,
  pub resolvers: Option<Vec<String>>,
  pub transformers: Option<HashMap<String, Vec<String>>>,
}
