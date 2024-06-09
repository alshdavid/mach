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
    println!("hi");
    let Some(rx) = WORKER_CHANNELS.lock().pop() else {
      println!("e1");
      return Err(napi::Error::from_reason("Unable to initialize worker"));
    };

    if !options.has_named_property("rpc")? {
      println!("e2");
      return Err(napi::Error::from_reason("Unable to initialize worker"));
    }
    let callback = options.get_named_property::<JsFunction>("rpc")?;

    let mut threadsafe_function: ThreadsafeFunction<RpcMessage> =
      env.create_threadsafe_function(&callback, 0, |ctx: ThreadSafeCallContext<RpcMessage>| {
        println!("um {:?}", ctx.value);
        let id = RpcHostNodejs::get_message_id(&ctx.value);
        match ctx.value {
          RpcMessage::Ping { response } => {
            let callback = RpcHostNodejs::create_callback(&ctx.env, response)?;
            let id = ctx.env.create_uint32(id)?.into_unknown();
            let message = ctx.env.to_js_value(&())?;
            Ok(vec![id, message, callback])
          }
          RpcMessage::Init { response } => {
            let callback = RpcHostNodejs::create_callback(&ctx.env, response)?;
            let id = ctx.env.create_uint32(id)?.into_unknown();
            let message = ctx.env.to_js_value(&())?;
            Ok(vec![id, message, callback])
          }
        }
      })?;

    threadsafe_function.unref(&env)?;

    thread::spawn(move || {
      while let Ok(msg) = rx.recv() {
        if !matches!(
          threadsafe_function.call(Ok(msg), ThreadsafeFunctionCallMode::Blocking),
          Status::Ok
        ) {
          return;
        };
      }
    });

    Ok(Self {})
  }
}
