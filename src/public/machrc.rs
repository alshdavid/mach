use std::{collections::HashMap, path::PathBuf};

#[derive(Clone, Debug)]
pub struct Machrc {
  pub file_path: PathBuf,
  pub resolvers: Option<Vec<String>>,
  pub transformers: Option<HashMap<String, Vec<String>>>,
}
