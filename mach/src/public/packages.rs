use std::path::PathBuf;

pub type Outputs = Vec<Output>;

pub struct Output {
  pub content: Vec<u8>,
  pub filepath: PathBuf,
}

impl std::fmt::Debug for Output {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    f.debug_struct("Output")
      .field("content (size)", &self.content.len())
      .field("filepath", &self.filepath)
      .finish()
  }
}
