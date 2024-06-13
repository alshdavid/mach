use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

use anyhow;

pub type Engine = String;
pub type RpcHosts = HashMap<Engine, RpcHostRef>;
pub type RpcHostRef = Arc<dyn RpcHost>;
pub type RpcConnectionRef = Arc<dyn RpcConnection>;

pub trait RpcHost: Debug + Send + Sync {
  fn engine(&self) -> String;
  fn ping(&self) -> anyhow::Result<()>;
  fn start(&self) -> anyhow::Result<RpcConnectionRef>;
}

pub trait RpcConnection: Debug + Send + Sync {
  fn ping(&self) -> anyhow::Result<()>;
}
