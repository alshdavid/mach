use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpecifierType {
  #[default]
  ESM,
  Commonjs,
}
