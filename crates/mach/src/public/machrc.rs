use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Machrc {
  pub file_path: PathBuf,
  pub resolvers: Option<Vec<String>>,
}
