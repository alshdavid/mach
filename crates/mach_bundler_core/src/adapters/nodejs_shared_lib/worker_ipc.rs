use ipc_channel_adapter::child::sync::HostReceiver;
use ipc_channel_adapter::child::sync::HostSender;
use napi::Env;
use napi::JsFunction;
use napi::JsUndefined;

use super::worker_callback::worker_callback;
use crate::public::AdapterIncomingRequest;
use crate::public::AdapterIncomingResponse;
use crate::public::AdapterOutgoingRequest;
use crate::public::AdapterOutgoingResponse;

pub fn worker_ipc(
  env: Env,
  child_sender: String,
  child_receiver: String,
  callback: JsFunction,
) -> napi::Result<JsUndefined> {
  let (_, rx_ipc) =
    HostReceiver::<AdapterOutgoingRequest, AdapterOutgoingResponse>::new(&child_sender).unwrap();

  let _tx_ipc =
    HostSender::<AdapterIncomingRequest, AdapterIncomingResponse>::new(&child_receiver).unwrap();

  worker_callback(&env, rx_ipc, callback);

  env.get_undefined()
}
