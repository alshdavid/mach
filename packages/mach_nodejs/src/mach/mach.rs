use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::Arc;

use mach_bundler_core::public::RpcHosts;
use mach_bundler_core::public::RpcMessage;
use mach_bundler_core::rpc::nodejs::RpcHostNodejs;
use mach_bundler_core::Mach;
use mach_bundler_core::MachOptions;
use napi::Env;
use napi::JsFunction;
use napi::JsNumber;
use napi::JsObject;
use napi::JsUndefined;
use napi_derive::napi;
use once_cell::sync::Lazy;
use parking_lot::Mutex;

use crate::cmd::build;

pub static WORKER_CHANNELS: Lazy<Mutex<Vec<Receiver<RpcMessage>>>> =
  Lazy::new(|| Default::default());

// Public API for Parcel
#[napi]
pub struct MachNapi {
  mach: Arc<Mach>,
  nodejs_rpc_host: Arc<RpcHostNodejs>
}

#[napi]
impl MachNapi {
  #[napi(constructor)]
  pub fn new(
    env: Env,
    options: JsObject,
  ) -> napi::Result<Self> {
    let threads;
    if options.has_named_property("threads")? {
      let js_threads = options.get_named_property::<JsNumber>("threads")?;
      threads = env.from_js_value::<usize, JsNumber>(js_threads)?;
    } else {
      threads = num_cpus::get_physical()
    }

    let mut rpc_hosts = RpcHosts::default();

    let node_workers;
    if options.has_named_property("nodeWorkers")? {
      let js_threads = options.get_named_property::<JsNumber>("nodeWorkers")?;
      node_workers = env.from_js_value(js_threads)?;
    } else {
      node_workers = threads.clone();
    }

    if !options.has_named_property("rpc")? {
      return Err(napi::Error::from_reason("Missing RPC callback"))
    }

    let callback = options.get_named_property::<JsFunction>("rpc")?;
    let mut tx_node_workers = vec![];
    for _ in 0..node_workers {
      let (tx, rx) = channel();
      WORKER_CHANNELS.lock().push(rx);
      tx_node_workers.push(tx);
    }
    let nodejs_rpc_host = Arc::new(RpcHostNodejs::new(tx_node_workers, &env, callback)?);
    rpc_hosts.insert("nodejs".to_string(), nodejs_rpc_host.clone());

    Ok(Self {
      nodejs_rpc_host,
      mach: Arc::new(Mach::new(MachOptions {
        threads: Some(threads),
        rpc_hosts,
      })),
    })
  }

  #[napi]
  pub fn build(
    &self,
    env: Env,
    options: JsObject,
    callback: JsFunction,
  ) -> napi::Result<JsUndefined> {
    build(self.nodejs_rpc_host.clone(), self.mach.clone(), env, options, callback)
  }
}

impl Drop for MachNapi {
    fn drop(&mut self) {
        println!("MachNapi.drop()")
    }
}