use std::fmt::Debug;
use std::fmt::Formatter;
use std::sync::mpsc::Receiver;

use crate::public::Adapter;
use crate::public::AdapterOutgoingRequest;
use crate::public::AdapterOutgoingResponse;

#[derive(Clone)]
pub struct NodejsNapiAdapter {}

impl Adapter for NodejsNapiAdapter {
  fn is_running(&self) -> bool {
    todo!()
  }

  fn init(&self) -> Result<(), String> {
    todo!()
  }

  fn send_all(
    &self,
    _req: AdapterOutgoingRequest,
  ) -> Result<Vec<AdapterOutgoingResponse>, String> {
    todo!()
  }

  fn send(
    &self,
    _req: AdapterOutgoingRequest,
  ) -> Receiver<AdapterOutgoingResponse> {
    todo!()
  }

  fn send_and_wait(
    &self,
    _req: AdapterOutgoingRequest,
  ) -> Result<AdapterOutgoingResponse, String> {
    todo!()
  }
}

impl Debug for NodejsNapiAdapter {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> std::fmt::Result {
    f.debug_struct("NodejsNapiAdapter").finish()
  }
}
