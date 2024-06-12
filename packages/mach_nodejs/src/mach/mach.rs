use std::sync::Arc;

use mach_bundler_core::rpc::nodejs::RpcHostNodejs;
use mach_bundler_core::rpc::RpcHosts;
use mach_bundler_core::Mach;
use mach_bundler_core::MachOptions;
use napi::Env;
use napi::JsFunction;
use napi::JsNumber;
use napi::JsObject;
use napi::JsUndefined;
use napi_derive::napi;

use crate::cmd::build;

#[napi]
pub struct MachNapi {
  mach: Arc<Mach>,
  rpc_host_nodejs: Arc<RpcHostNodejs>,
}

#[napi]
pub fn mach_napi_new(
  env: Env,
  options: JsObject,
) -> napi::Result<MachNapi> {
  let threads;
  if options.has_named_property("threads")? {
    let js_threads = options.get_named_property::<JsNumber>("threads")?;
    threads = env.from_js_value::<usize, JsNumber>(js_threads)?;
  } else {
    threads = num_cpus::get_physical()
  }

  let mut rpc_hosts = RpcHosts::default();

  let node_workers: JsNumber = options.get_property(env.create_string("nodeWorkers")?)?;
  let node_workers = node_workers.get_uint32()?;
  let rpc_host_nodejs = Arc::new(RpcHostNodejs::new(
    &env,
    options.get_named_property("rpc")?,
    node_workers,
  )?);

  rpc_hosts.insert(
    "nodejs".to_string(),
    rpc_host_nodejs.clone(),
  );

  Ok(MachNapi {
    rpc_host_nodejs,
    mach: Arc::new(Mach::new(MachOptions {
      threads: Some(threads),
      rpc_hosts,
    })),
  })
}

#[napi]
pub fn mach_napi_build(
  mach_napi: &MachNapi,
  env: Env,
  options: JsObject,
  callback: JsFunction,
) -> napi::Result<JsUndefined> {
  build(
    mach_napi.rpc_host_nodejs.clone(),
    mach_napi.mach.clone(),
    env,
    options,
    callback,
  )
}
