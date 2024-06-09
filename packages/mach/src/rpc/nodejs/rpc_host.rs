use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::thread;

use anyhow::anyhow;
use napi;
use napi::threadsafe_function::ThreadSafeCallContext;
use napi::threadsafe_function::ThreadsafeFunction;
use napi::threadsafe_function::ThreadsafeFunctionCallMode;
use napi::Env;
use napi::JsFunction;
use napi::JsUnknown;
use napi::Status;
use parking_lot::Mutex;
use serde::de::DeserializeOwned;

use crate::public::RpcHost;
use crate::public::RpcMessage;

#[derive(Debug)]
pub struct RpcHostNodejs {
  node_workers: Vec<Sender<RpcMessage>>,
  tx_rpc: Sender<RpcMessage>,
}

impl RpcHostNodejs {
  pub fn new(
    node_workers: Vec<Sender<RpcMessage>>,
    env: &Env,
    mut callback: JsFunction,
  ) -> napi::Result<Self> {
    // Create a threadsafe function that casts the incoming message data to something
    // accessible in JavaScript. The function accepts a return value from a JS callback
    let mut threadsafe_function: ThreadsafeFunction<RpcMessage> =
      env.create_threadsafe_function(&callback, 0, |ctx: ThreadSafeCallContext<RpcMessage>| {
        let id = Self::get_message_id(&ctx.value);
        match ctx.value {
          RpcMessage::Ping { response } => {
            let callback = Self::create_callback(&ctx.env, response)?;
            let id = ctx.env.create_uint32(id)?.into_unknown();
            let message = ctx.env.to_js_value(&())?;
            Ok(vec![id, message, callback])
          }
          RpcMessage::Init { response } => {
            let callback = Self::create_callback(&ctx.env, response)?;
            let id = ctx.env.create_uint32(id)?.into_unknown();
            let message = ctx.env.to_js_value(&())?;
            Ok(vec![id, message, callback])
          }
        }
      })?;

    threadsafe_function.unref(&env)?;

    // Forward RPC events to the threadsafe function from a new thread
    let (tx_rpc, rx_rpc) = channel();

    thread::spawn(move || {
      while let Ok(msg) = rx_rpc.recv() {
        if !matches!(
          threadsafe_function.call(Ok(msg), ThreadsafeFunctionCallMode::Blocking),
          Status::Ok
        ) {
          return;
        };
      }
    });

    Ok(Self {
      tx_rpc,
      node_workers,
    })
  }

  // Generic method to create a "resolve" javascript function to
  // return the value from the thread safe function
  pub fn create_callback<Returns: DeserializeOwned + 'static>(
    env: &Env,
    reply: Sender<Returns>,
  ) -> napi::Result<JsUnknown> {
    println!("0");
    let callback = env
      .create_function_from_closure("callback", move |ctx| {
        println!("2");
        let response = ctx
          .env
          .from_js_value::<Returns, JsUnknown>(ctx.get::<JsUnknown>(0)?)?;

        println!("3");
        if reply.send(response).is_err() {
          return Err(napi::Error::from_reason("Unable to send rpc response"));
        }

        ctx.env.get_undefined()
      })?
      .into_unknown();

    println!("1");

    Ok(callback)
  }

  // Map the RPC messages to numerical values to make matching
  // easier from within JavaScript
  pub fn get_message_id(message: &RpcMessage) -> u32 {
    match message {
      RpcMessage::Ping { response: _ } => 0,
      RpcMessage::Init { response: _ } => 1,
    }
  }
}

// Forward events to Nodejs
impl RpcHost for RpcHostNodejs {
  fn init(&self) -> anyhow::Result<()> {
    let tx_rpc = self.tx_rpc.clone();

    for _ in self.node_workers.iter() {
      let (tx, rx) = channel();
      tx_rpc.send(RpcMessage::Init { response: tx }).unwrap();
      rx.recv().unwrap();
    }

    Ok(())
  }

  fn is_running(&self) -> bool {
    true
  }

  fn ping(&self) -> anyhow::Result<()> {
    let (tx, rx) = channel();
    self.tx_rpc.send(RpcMessage::Ping { response: tx })?;

    for sender in self.node_workers.iter() {
      let (tx, rx) = channel();
      sender.send(RpcMessage::Ping { response: tx })?;
      rx.recv()?;
    }

    Ok(rx.recv()?.map_err(|e| anyhow!(e))?)
  }
}
