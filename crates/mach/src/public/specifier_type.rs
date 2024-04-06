use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum SpecifierType {
  #[default]
  ESM,
  Commonjs,
}
