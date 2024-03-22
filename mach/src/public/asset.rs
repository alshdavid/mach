use std::hash::Hasher;
use std::hash::Hash;
use std::fmt::Debug;
use std::path::PathBuf;
use swc_core::ecma::ast::Program;

use crate::kit::hash::{hash_path_buff_sha_256, hash_sha_256, hash_string_sha_256};

use super::BundleBehavior;

#[derive(Clone, Default)]
pub struct Asset {
  pub name: String,
  pub file_path: PathBuf,
  pub file_path_rel: PathBuf,
  /// Describes the type of the Asset. Stars as the file extension
  pub kind: String,
  pub content: AssetContent,
  pub bundle_behavior: BundleBehavior,
}

impl Debug for Asset {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    f.debug_struct("Asset")
      .field("file_path", &self.file_path)
      .field("file_path_rel", &self.file_path_rel)
      .field("bundle_behavior", &self.bundle_behavior)
      .field("kind", &self.kind)
      .field("content", &self.content)
      .finish()
  }
}

impl Asset {
  pub fn get_content(&self) -> Result<&AssetContent, String> {
    return Ok(&self.content);
  }

  pub fn content_hash(&self) -> Result<String, String> {
    let content = self.get_content()?;

    let content_hash: String = match &content {
      AssetContent::Bytes(bytes) => {
        hash_sha_256(bytes)
      },
      AssetContent::JavaScript(program) => {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        program.hash(&mut hasher);
        let digest = hasher.finish();
        format!("{:x}", digest)
      },
    };

    let path_hash = hash_path_buff_sha_256(&self.file_path_rel);
    let hash = hash_string_sha_256(&format!("{} {}", path_hash, content_hash));
    return Ok(hash);
  }
}


#[derive(Clone)]
pub enum AssetContent {
  Bytes(Vec<u8>),
  JavaScript(Program),
}

impl Default for AssetContent {
    fn default() -> Self {
        return AssetContent::Bytes(vec![]);
    }
}

impl Debug for AssetContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::JavaScript(_) => f.debug_tuple("JavaScript").field(&"JavaScript".to_string()).finish(),
            Self::Bytes(_) => f.debug_tuple("Bytes").field(&"Bytes".to_string()).finish(),
        }
    }
}

