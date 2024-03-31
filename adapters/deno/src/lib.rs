mod adapter;
mod deno_cli;
mod deno_embed;
mod deno_snapshots;
mod mach_extensions;

use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

use adapter::DependencyGetters;
use deno_core::error::AnyError;
use deno_core::v8::Function;
use deno_core::v8::Value;
use deno_core::JsRuntime;

use adapter::DenoAction;
use adapter::DenoAdapter;
use deno_core::serde_v8;
use deno_core::v8::Global;
use deno_core::PollEventLoopOptions;
use deno_embed::deno_current_thread;
use libmach::Adapter;
use libmach::AdapterBootstrapOptions;
use libmach::AdapterBootstrapResult;

use mach_extensions::mach_hello_world;
use mach_extensions::mach_load_resolver;
use mach_extensions::mach_run_resolver_resolve;
use mach_extensions::RunResolverResolveState;
use mach_extensions::LoadResolverState;
use serde::Serialize;
use tokio::sync::oneshot;

#[no_mangle]
pub extern "C" fn bootstrap(_config: AdapterBootstrapOptions) -> AdapterBootstrapResult {
  let (tx, rx) = channel::<DenoAction>();
  let dependency_getters_local = DependencyGetters::default();
  let dependency_getters = dependency_getters_local.clone();
  
  std::thread::spawn(move || {
    deno_current_thread(async move {
      let options = deno_embed::DenoInitOptions {
        script: "/home/dalsh/Development/alshdavid/mach/adapters/deno/javascript/index.js"
          .to_string(),
        extensions: vec![
          mach_hello_world::init_ops_and_esm(),
          mach_load_resolver::init_ops_and_esm(),
          mach_run_resolver_resolve::init_ops_and_esm(dependency_getters.clone()),
        ],
        ..Default::default()
      };
      let mut worker = deno_embed::run_script(options).await.unwrap();

      worker.run().await.unwrap();

      let fn_load_resolver = {
        let state: LoadResolverState = worker.worker.js_runtime.op_state().borrow_mut().take();
        state.load_resolver_callback.unwrap()
      };

      let fn_run_resolver_resolve = {
        let state: RunResolverResolveState = worker.worker.js_runtime.op_state().borrow_mut().take();
        state.run_resolver_resolve_callback.unwrap()
      };

      while let Ok(action) = rx.recv() {
        match action {
          DenoAction::LoadResolver(file_path, next) => {
            run_js_callback(
              &mut worker.worker.js_runtime,
              &fn_load_resolver,
              file_path.to_str().unwrap(),
            ).await.unwrap();

            next.send(()).unwrap();
          }
          DenoAction::RunResolverResolve(specifier, dependency_id, next) => {
            run_js_callback(
              &mut worker.worker.js_runtime,
              &fn_run_resolver_resolve,
              &(specifier, dependency_id),
            ).await.unwrap();

            next.send(()).unwrap();
          },
        }
      }
    });
  });

  let adapter: Box<dyn Adapter> = Box::new(DenoAdapter { 
    tx,
    dependency_getters: dependency_getters_local,
  });
  return Box::new(Box::new(Ok(adapter)));
}


async fn run_js_callback<T: Serialize>(
  js_runtime: &mut JsRuntime,
  function: &Global<Function>,
  input: T,
) -> Result<Global<Value>, AnyError> {
  let request = {
    let mut scope = &mut js_runtime.handle_scope();
    let request = serde_v8::to_v8(scope, input).unwrap();
    Global::new(&mut scope, request)
  };

  let call = js_runtime
    .call_with_args(function, &[request]);

  js_runtime
    .with_event_loop_promise(call, PollEventLoopOptions::default())
    .await
}

