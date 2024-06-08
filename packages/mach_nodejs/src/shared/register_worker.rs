use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;

use mach_bundler_core::rpc::nodejs_napi::NapiOutgoingData;
use mach_bundler_core::rpc::nodejs_napi::worker_callback::worker_callback;
use napi::Env;
use napi::JsFunction;
use napi::JsUndefined;
use napi_derive::napi;
use once_cell::sync::Lazy;

pub type RegisterWorkerData = Sender<NapiOutgoingData>;
pub type RegisterWorkerSender = Sender<RegisterWorkerData>;
pub type RegisterWorkerReceiver = Arc<Mutex<Option<Receiver<RegisterWorkerData>>>>;

pub static REGISTER_WORKER: Lazy<(RegisterWorkerSender, RegisterWorkerReceiver)> =
  Lazy::new(|| {
    let (tx, rx) = channel::<RegisterWorkerData>();
    (tx, Arc::new(Mutex::new(Some(rx))))
  });

#[napi]
pub fn register_worker(
  env: Env,
  callback: JsFunction,
) -> napi::Result<JsUndefined> {
  let (tx_outgoing, rx_outgoing) = channel::<NapiOutgoingData>();
  REGISTER_WORKER.0.send(tx_outgoing).unwrap();

  worker_callback(&env, rx_outgoing, callback);

  env.get_undefined()
}
