use std::sync::mpsc::channel;
use std::sync::Arc;

use napi::threadsafe_function::ThreadSafeCallContext;
use napi::threadsafe_function::ThreadsafeFunction;
use napi::threadsafe_function::ThreadsafeFunctionCallMode;
use napi::Env;
use napi::JsFunction;
use napi::JsUnknown;
use napi::Status;

use super::super::RpcConnectionRef;
use super::super::RpcHost;
use super::napi::create_done_callback;
use super::rpc_host_message::RpcHostMessage;
use super::RpcConnectionNodejs;
use super::RpcConnectionNodejsMulti;

pub struct RpcHostNodejs {
  threadsafe_function: ThreadsafeFunction<RpcHostMessage>,
  node_workers: usize,
}

impl std::fmt::Debug for RpcHostNodejs {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    f.debug_struct("RpcHostNodejs")
      .field("node_workers", &self.node_workers)
      .finish()
  }
}

impl RpcHostNodejs {
  pub fn new(
    env: &Env,
    callback: JsFunction,
    node_workers: usize,
  ) -> napi::Result<Self> {
    let mut threadsafe_function: ThreadsafeFunction<RpcHostMessage> = env
      .create_threadsafe_function(
        &callback,
        0,
        |ctx: ThreadSafeCallContext<RpcHostMessage>| {
          let id = ctx.env.create_uint32(ctx.value.get_id())?.into_unknown();
          let (message, callback) = Self::map_rpc_message(&ctx.env, ctx.value)?;
          Ok(vec![id, message, callback])
        },
      )?;

    threadsafe_function.unref(&env)?;

    Ok(Self {
      node_workers,
      threadsafe_function,
    })
  }

  fn call_rpc(
    &self,
    msg: RpcHostMessage,
  ) {
    if !matches!(
      self
        .threadsafe_function
        .call(Ok(msg), ThreadsafeFunctionCallMode::NonBlocking),
      Status::Ok
    ) {
      return;
    };
  }

  fn map_rpc_message(
    env: &Env,
    msg: RpcHostMessage,
  ) -> napi::Result<(JsUnknown, JsUnknown)> {
    Ok(match msg {
      RpcHostMessage::Ping { response: reply } => {
        let message = env.to_js_value(&())?;
        let callback = create_done_callback(&env, reply)?;
        (message, callback)
      }
    })
  }
}

impl RpcHost for RpcHostNodejs {
  fn engine(&self) -> String {
    "nodejs".into()
  }

  fn ping(&self) -> anyhow::Result<()> {
    let (tx, rx) = channel();
    self.call_rpc(RpcHostMessage::Ping { response: tx });
    Ok(rx.recv()?.map_err(|e| anyhow::anyhow!(e))?)
  }

  fn start(&self) -> anyhow::Result<RpcConnectionRef> {
    let mut connections = vec![];

    for _ in 0..self.node_workers {
      connections.push(RpcConnectionNodejs::new())
    }

    Ok(Arc::new(RpcConnectionNodejsMulti::new(connections)))
  }
}
