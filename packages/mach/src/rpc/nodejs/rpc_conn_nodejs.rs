use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::thread;

use napi::threadsafe_function::ThreadSafeCallContext;
use napi::threadsafe_function::ThreadsafeFunction;
use napi::threadsafe_function::ThreadsafeFunctionCallMode;
use napi::Env;
use napi::JsFunction;
use napi::JsUnknown;
use napi::Status;

use super::super::RpcConnection;
use super::napi::create_done_callback;
use super::rpc_conn_message::RpcConnectionMessage;
use super::worker_init::get_worker_rx;
use super::worker_init::get_worker_tx;

#[derive(Debug)]
pub struct RpcConnectionNodejs {
  tx_rpc: Sender<RpcConnectionMessage>,
}

impl RpcConnectionNodejs {
  pub fn new() -> Self {
    Self {
      tx_rpc: get_worker_tx(),
    }
  }

  pub fn create_worker_callback(
    env: &Env,
    callback: JsFunction,
  ) -> napi::Result<()> {
    let threadsafe_function: ThreadsafeFunction<RpcConnectionMessage> = env
      .create_threadsafe_function(
        &callback,
        0,
        |ctx: ThreadSafeCallContext<RpcConnectionMessage>| {
          let id = ctx.env.create_uint32(ctx.value.get_id())?.into_unknown();
          let (message, callback) = Self::map_rpc_message(&ctx.env, ctx.value)?;
          Ok(vec![id, message, callback])
        },
      )?;

    thread::spawn(move || {
      let rx = get_worker_rx();
      while let Ok(msg) = rx.recv() {
        if !matches!(
          threadsafe_function.call(Ok(msg), ThreadsafeFunctionCallMode::NonBlocking),
          Status::Ok
        ) {
          return;
        };
      }
    });
    Ok(())
  }

  // Map the RPC message to a JavaScript type
  fn map_rpc_message(
    env: &Env,
    msg: RpcConnectionMessage,
  ) -> napi::Result<(JsUnknown, JsUnknown)> {
    Ok(match msg {
      RpcConnectionMessage::Ping { response: reply } => {
        let message = env.to_js_value(&())?;
        let callback = create_done_callback(&env, reply)?;
        (message, callback)
      }
    })
  }
}

impl RpcConnection for RpcConnectionNodejs {
  fn ping(&self) -> anyhow::Result<()> {
    let (tx, rx) = channel();
    self
      .tx_rpc
      .send(RpcConnectionMessage::Ping { response: tx })?;
    Ok(rx.recv()?.map_err(|e| anyhow::anyhow!(e))?)
  }
}
