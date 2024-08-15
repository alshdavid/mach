use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum BundleBehavior {
  #[default]
  Default,
  Inline,
  Isolated,
}
