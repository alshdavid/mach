use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::mpsc::Receiver;
use std::sync::Arc;

use super::RpcMessage;
use super::AdapterOutgoingResponse;

pub type Engine = String;
pub type RpcHosts = HashMap<Engine, Arc<dyn RpcHost>>;

pub trait RpcHost: Debug + Send + Sync {
  fn is_running(&self) -> bool;
  fn init(&self) -> Result<(), String>;
  fn ping(&self) -> Result<(), String>;
}
