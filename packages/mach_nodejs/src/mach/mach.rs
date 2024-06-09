use std::sync::Arc;

use mach_bundler_core::public::RpcHosts;
use mach_bundler_core::rpc::nodejs::RpcHostNodejs;
use mach_bundler_core::Mach;
use mach_bundler_core::MachOptions;
use napi::Env;
use napi::JsFunction;
use napi::JsNumber;
use napi::JsObject;
use napi::JsUndefined;
use napi_derive::napi;

use crate::cmd::build;

// Public API for Parcel

#[napi]
pub struct MachNapi {
  mach: Arc<Mach>
}

#[napi]
impl MachNapi {
  #[napi(constructor)]
  pub fn new(
    env: Env,
    options: JsObject,
  ) -> napi::Result<Self> {
    let mut rpc_hosts = RpcHosts::default();

    if options.has_named_property("rpc")? {
      let callback = options.get_named_property::<JsFunction>("rpc")?;
      let nodejs_rpc_host = RpcHostNodejs::new(&env, callback)?;
      rpc_hosts.insert("nodejs".to_string(), Arc::new(nodejs_rpc_host));
    }

    let mut threads = None::<usize>;
    if options.has_named_property("threads")? {
      let js_threads = options.get_named_property::<JsNumber>("threads")?;
      threads = Some(env.from_js_value(js_threads)?);
    }

    Ok(Self {
      mach: Arc::new(Mach::new(MachOptions {
        threads,
        rpc_hosts: Some(rpc_hosts),
      }))
    })
  }

  #[napi]
  pub fn build(
    &self,
    env: Env,
    options: JsObject,
  ) -> napi::Result<JsUndefined> {
    build(self.mach.clone(), env, options)
  }
}
