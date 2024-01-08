use std::path::PathBuf;

use super::JavaScriptAsset;

pub type AssetId = String;

#[derive(Debug, Clone)]
pub enum Asset {
  JavaScript(JavaScriptAsset),
  Style,
  Markup,
}

impl Asset {
  pub fn id(&self) -> AssetId {
    return match self {
      Asset::JavaScript(a) => a.id.clone(),
      _ => panic!(),
    }
  }

  pub fn file_path(&self) -> PathBuf {
    return match self {
      Asset::JavaScript(a) => a.file_path.clone(),
      _ => panic!(),
    }
  }

  pub fn file_path_relative(&self) -> PathBuf {
    return match self {
      Asset::JavaScript(a) => a.file_path_relative.clone(),
      _ => panic!(),
    }
  }

  pub fn file_path_relative_hash(&self) -> String {
    return match self {
      Asset::JavaScript(a) => a.file_path_relative_hash.clone(),
      _ => panic!(),
    }
  }

  pub fn source_content_hash(&self) -> String {
    return match self {
      Asset::JavaScript(a) => a.source_content_hash.clone(),
      _ => panic!(),
    }
  }
}

