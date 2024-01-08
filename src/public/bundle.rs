use swc_core::ecma::ast::Script;

use super::AssetId;

#[derive(Debug)]
pub enum Bundle {
  JavaScript(JavaScriptBundle),
  Style,
  Markup,
}

impl Bundle {
  pub fn name(&self) -> AssetId {
    return match self {
      Bundle::JavaScript(a) => a.name.clone(),
      _ => panic!(),
    };
  }
}

#[derive(Debug)]
pub struct JavaScriptBundle {
  pub name: String,
  pub entry: Option<AssetId>,
  pub assets: Vec<AssetId>,
  pub output: Script,
}
