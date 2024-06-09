use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;

use mach_bundler_core::BuildOptions;
use mach_bundler_core::Mach;
use napi::threadsafe_function::ThreadSafeCallContext;
use napi::threadsafe_function::ThreadsafeFunction;
use napi::threadsafe_function::ThreadsafeFunctionCallMode;
use napi::Env;
use napi::JsFunction;
use napi::JsObject;
use napi::JsUndefined;
use serde::Deserialize;
use serde::Serialize;

// use crate::shared::mach_build_command;

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
pub struct BuildResult {
  pub bundle_manifest: HashMap<String, String>,
  pub entries: HashMap<String, String>,
}

pub fn build(
  mach: Arc<Mach>,
  env: Env,
  options: JsObject,
  callback: JsFunction,
) -> napi::Result<JsUndefined> {
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

  let thread_safe_callback: ThreadsafeFunction<usize> =
    env.create_threadsafe_function(&callback, 0, |ctx: ThreadSafeCallContext<usize>| {
      let message = ctx.env.to_js_value(&())?;
      Ok(vec![message])
    })?;

  thread::spawn(move || {
    match mach.build(options) {
      Ok(_result) => {
        thread_safe_callback.call(Ok(42), ThreadsafeFunctionCallMode::Blocking);
      }
      Err(err) => {
        thread_safe_callback.call(
          Err(napi::Error::from_reason(format!("{}", err))),
          ThreadsafeFunctionCallMode::NonBlocking,
        );
      }
    };
  });

  env.get_undefined()
}
