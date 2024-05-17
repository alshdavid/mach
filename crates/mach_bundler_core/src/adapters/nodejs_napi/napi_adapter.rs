use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use crate::public::Adapter;
use crate::public::AdapterIncomingRequest;
use crate::public::AdapterIncomingResponse;
use crate::public::AdapterOutgoingRequest;
use crate::public::AdapterOutgoingResponse;

#[derive(Clone)]
pub struct NodejsNapiAdapter {
}

impl Adapter for NodejsNapiAdapter {
  fn new(options: HashMap<String, String>) -> Result<Self, String>
  where
    Self: Sized,
  {
    todo!()
  }

  fn is_running(&self) -> bool {
    todo!()
  }

  fn init(&self) -> Result<(), String> {
    todo!()
  }

  fn send_all(
    &self,
    req: AdapterOutgoingRequest,
  ) -> Result<Vec<AdapterOutgoingResponse>, String> {
    todo!()
  }

  fn send(
    &self,
    req: AdapterOutgoingRequest,
  ) -> Receiver<AdapterOutgoingResponse> {
    todo!()
  }

  fn send_and_wait(
    &self,
    req: AdapterOutgoingRequest,
  ) -> Result<AdapterOutgoingResponse, String> {
    todo!()
  }
}

impl Debug for NodejsNapiAdapter {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> std::fmt::Result {
    f.debug_struct("NodejsNapiAdapter")
      .finish()
  }
}
