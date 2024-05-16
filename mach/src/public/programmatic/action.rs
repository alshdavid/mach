use serde::Deserialize;
use serde::Serialize;

use super::BuildReport;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ProgrammaticAction {
  BuildReport { data: BuildReport },
}
