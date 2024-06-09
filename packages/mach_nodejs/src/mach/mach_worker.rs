use std::thread;

use mach_bundler_core::public::RpcMessage;
use mach_bundler_core::rpc::nodejs::RpcHostNodejs;
use napi::threadsafe_function::ThreadSafeCallContext;
use napi::threadsafe_function::ThreadsafeFunction;
use napi::threadsafe_function::ThreadsafeFunctionCallMode;
use napi::Env;
use napi::JsFunction;
use napi::JsObject;
use napi::Status;
use napi_derive::napi;

use crate::mach::mach::WORKER_CHANNELS;

#[napi]
pub struct MachWorkerNapi {}

#[napi]
impl MachWorkerNapi {
  #[napi(constructor)]
  pub fn new(
    env: Env,
    options: JsObject,
  ) -> napi::Result<Self> {
    let Some(rx) = WORKER_CHANNELS.lock().pop() else {
      return Err(napi::Error::from_reason("Unable to initialize worker"));
    };

    if !options.has_named_property("rpc")? {
      return Err(napi::Error::from_reason("Unable to initialize worker"));
    }
    let callback = options.get_named_property::<JsFunction>("rpc")?;
    let tx_rpc = RpcHostNodejs::init_callback(&env, callback)?;

    thread::spawn(move || {
      while let Ok(msg) = rx.recv() {
        tx_rpc.send(msg).unwrap();
      }
    });

    Ok(Self {})
  }
}
