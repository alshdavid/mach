use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use anyhow;

use super::RpcMessage;
use super::AdapterOutgoingResponse;

pub type Engine = String;
pub type RpcHosts = HashMap<Engine, Arc<dyn RpcHost>>;

pub trait RpcHost: Debug + Send + Sync {
  fn is_running(&self) -> bool;
  fn init(&self) -> anyhow::Result<()>;
  fn ping(&self) -> anyhow::Result<()>;
}
