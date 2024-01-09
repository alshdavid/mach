use std::path::PathBuf;

use swc_core::ecma::ast::Program;

use crate::platform::hash::{hash_path_buff_sha_256, hash_string_sha_256};

use super::JavaScriptAsset;

pub type AssetId = String;

#[derive(Debug, Clone)]
pub enum Asset {
  Unknown(UnknownAsset),
  JavaScript(JavaScriptAsset),
  Style,
  Markup,
}

impl Asset {
  pub fn id(&self) -> AssetId {
    return match self {
      Asset::JavaScript(a) => a.id.clone(),
      _ => panic!(),
    };
  }

  pub fn file_path(&self) -> PathBuf {
    return match self {
      Asset::JavaScript(a) => a.file_path.clone(),
      _ => panic!(),
    };
  }

  pub fn file_path_relative(&self) -> PathBuf {
    return match self {
      Asset::JavaScript(a) => a.file_path_relative.clone(),
      _ => panic!(),
    };
  }

  pub fn file_path_relative_hash(&self) -> String {
    return match self {
      Asset::JavaScript(a) => a.file_path_relative_hash.clone(),
      _ => panic!(),
    };
  }

  pub fn source_content_hash(&self) -> String {
    return match self {
      Asset::JavaScript(a) => a.source_content_hash.clone(),
      _ => panic!(),
    };
  }
}

#[derive(Debug, Clone)]
pub struct UnknownAsset {
  pub id: AssetId,
  pub file_path: PathBuf,
  pub file_path_relative: PathBuf,
  pub file_path_relative_hash: String,
  pub contents: String,
  pub contents_hash: String,
}

impl UnknownAsset {
  pub fn new(
    root_path: &PathBuf,
    asset_filepath_absolute: &PathBuf,
    asset_contents: &str,
  ) -> Self {
    let relative_path = pathdiff::diff_paths(asset_filepath_absolute, root_path).unwrap();
    let relative_path_hash = hash_path_buff_sha_256(&relative_path);
    let contents_hash = hash_string_sha_256(&asset_contents);
    let id = relative_path.to_str().unwrap().to_string();

    return UnknownAsset {
      id,
      file_path: asset_filepath_absolute.clone(),
      file_path_relative: relative_path,
      file_path_relative_hash: relative_path_hash,
      contents: asset_contents.to_string(),
      contents_hash,
    };
  }

  pub fn to_javascript(&self, program: Program) -> JavaScriptAsset {
    return JavaScriptAsset {
      id: self.id.clone(),
      file_path: self.file_path.clone(),
      file_path_relative: self.file_path_relative.clone(),
      file_path_relative_hash: self.file_path_relative_hash.clone(),
      source_content_hash: self.contents_hash.clone(),
      program,
    }
  }
}
