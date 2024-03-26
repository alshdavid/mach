use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum SpecifierType {
  #[default]
  ESM,
  Commonjs,
}
