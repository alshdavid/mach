use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum BundleBehavior {
  #[default]
  Default,
  Inline,
  Isolated,
}
