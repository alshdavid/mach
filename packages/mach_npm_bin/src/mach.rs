/* 
  This contains the library bindings for the public API for Mach
*/
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;

use mach_bundler_core::rpc::nodejs::RpcHostNodejs;
use mach_bundler_core::rpc::RpcHost;
use mach_bundler_core::BuildOptions;
use mach_bundler_core::BuildResult;
use mach_bundler_core::Mach;
use mach_bundler_core::MachOptions;
use napi::bindgen_prelude::External;
use napi::threadsafe_function::ThreadSafeCallContext;
use napi::threadsafe_function::ThreadsafeFunction;
use napi::threadsafe_function::ThreadsafeFunctionCallMode;
use napi::Env;
use napi::JsFunction;
use napi::JsObject;
use napi_derive::napi;
use serde::Deserialize;
use serde::Serialize;

#[napi(object)]
pub struct MachNapi {
  pub node_worker_count: u32,
  pub mach: External<Arc<Mach>>,
}

//
// NEW
//
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewNapiOptions {
  pub threads: Option<usize>,
  pub node_workers: Option<usize>,
}

#[napi]
pub fn mach_napi_new(
  env: Env,
  options: JsObject,
  callback: JsFunction,
) -> napi::Result<MachNapi> {
  let js_options = env.from_js_value::<NewNapiOptions, JsObject>(options)?;
  let mut mach_options = MachOptions::default();

  if let Some(threads) = js_options.threads {
    mach_options.threads = threads
  }

  let node_workers: usize;
  if let Some(nw) = js_options.node_workers {
    node_workers = nw
  } else {
    node_workers = mach_options.threads;
  }

  let rpc_host_nodejs = Arc::new(RpcHostNodejs::new(&env, callback, node_workers)?);

  mach_options
    .rpc_hosts
    .insert(rpc_host_nodejs.engine(), rpc_host_nodejs.clone());

  Ok(MachNapi {
    node_worker_count: node_workers as u32,
    mach: External::new(Arc::new(Mach::new(mach_options))),
  })
}

//
// BUILD
//
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildOptionsNapi {
  pub entries: Option<Vec<String>>,
  pub project_root: Option<PathBuf>,
  pub out_folder: Option<PathBuf>,
  pub clean: Option<bool>,
  pub optimize: Option<bool>,
  pub bundle_splitting: Option<bool>,
  pub threads: Option<usize>,
  pub node_workers: Option<usize>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildResultNapi {
  pub bundle_manifest: HashMap<String, String>,
  pub entries: HashMap<String, String>,
}

#[napi]
pub fn mach_napi_build(
  this: MachNapi,
  env: Env,
  options: JsObject,
  callback: JsFunction,
) -> napi::Result<JsObject> {
  let options_napi = env.from_js_value::<BuildOptionsNapi, JsObject>(options)?;
  let mut options = BuildOptions::default();

  if let Some(entries) = options_napi.entries {
    options.entries = entries;
  }

  if let Some(out_folder) = options_napi.out_folder {
    options.out_folder = out_folder;
  }

  if let Some(clean) = options_napi.clean {
    options.clean = clean;
  }

  if let Some(optimize) = options_napi.optimize {
    options.optimize = optimize;
  }

  if let Some(bundle_splitting) = options_napi.bundle_splitting {
    options.bundle_splitting = bundle_splitting;
  }

  if let Some(project_root) = options_napi.project_root {
    options.project_root = Some(project_root);
  }

  let (tx, rx) = tokio::sync::oneshot::channel::<anyhow::Result<BuildResult>>();

  thread::spawn({
    let mach = this.mach.clone();
    move || tx.send(mach.build(options))
  });

  let thread_safe_callback: ThreadsafeFunction<BuildResult> =
    env.create_threadsafe_function(&callback, 0, |ctx: ThreadSafeCallContext<BuildResult>| {
      let result = BuildResultNapi {
        bundle_manifest: ctx.value.bundle_manifest,
        entries: ctx.value.entries,
      };
      let message = ctx.env.to_js_value(&result)?;
      Ok(vec![message])
    })?;

  env.spawn_future(async move {
    match rx.await.unwrap() {
      Ok(result) => {
        thread_safe_callback.call(Ok(result), ThreadsafeFunctionCallMode::NonBlocking);
      }
      Err(err) => {
        thread_safe_callback.call(
          Err(napi::Error::from_reason(format!("{}", err))),
          ThreadsafeFunctionCallMode::NonBlocking,
        );
      }
    };
    Ok(())
  })
}
