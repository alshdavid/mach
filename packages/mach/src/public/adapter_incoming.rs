use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AdapterIncomingRequest {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AdapterIncomingResponse {}
