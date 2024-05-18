use std::cell::RefCell;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use napi::Env;
use napi::JsFunction;
use napi::JsUndefined;

use super::worker_callback::worker_callback;
use crate::public::AdapterOutgoingRequest;
use crate::public::AdapterOutgoingResponse;

pub type HostMessage = (AdapterOutgoingRequest, Sender<AdapterOutgoingResponse>);

thread_local! {
  static HOST_CHANNEL: (Sender<HostMessage>, RefCell<Option<Receiver<HostMessage>>>) = {
    let (tx, rx) = channel::<HostMessage>();
    (tx, RefCell::new(Some(rx)))
  };
}

pub fn worker_napi(
  env: Env,
  callback: JsFunction,
) -> napi::Result<JsUndefined> {
  // let _tx_ipc =
  //   HostSender::<AdapterIncomingRequest, AdapterIncomingResponse>::new(&child_receiver).unwrap();

  let rx_napi = HOST_CHANNEL.with(|(_, rx)| rx.borrow_mut().take().unwrap());

  worker_callback(&env, rx_napi, callback);

  env.get_undefined()
}
