use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::mpsc::Receiver;
use std::sync::Arc;

use super::AdapterOutgoingRequest;
use super::AdapterOutgoingResponse;

pub type Engine = String;
pub type AdapterMap = HashMap<Engine, Arc<dyn Adapter>>;

pub trait Adapter: Debug + Send + Sync {
  fn new(options: HashMap<String, String>) -> Result<Self, String>
  where
    Self: Sized;

  fn is_running(&self) -> bool;

  fn init(&self) -> Result<(), String>;

  fn send_all(
    &self,
    req: AdapterOutgoingRequest,
  ) -> Result<Vec<AdapterOutgoingResponse>, String>;

  fn send(
    &self,
    req: AdapterOutgoingRequest,
  ) -> Receiver<AdapterOutgoingResponse>;

  fn send_and_wait(
    &self,
    req: AdapterOutgoingRequest,
  ) -> Result<AdapterOutgoingResponse, String>;
}
