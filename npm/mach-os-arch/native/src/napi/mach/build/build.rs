use std::collections::HashMap;
use std::path::PathBuf;
use std::thread;

use mach_bundler_core::BuildOptions as BuildOptionsCore;
use mach_bundler_core::BuildResult as BuildResultCore;
use napi::threadsafe_function::ThreadsafeFunctionCallMode;
use napi::Env;
use napi::JsFunction;
use napi::JsObject;
use napi::JsUndefined;
use serde::Deserialize;
use serde::Serialize;

use crate::shared::mach_build_command;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MachBuildOptions {
  pub entries: Option<Vec<PathBuf>>,
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
  env: Env,
  options: JsObject,
  callback: JsFunction,
) -> napi::Result<JsUndefined> {
  let options_napi = env.from_js_value::<MachBuildOptions, JsObject>(options)?;
  let mut options = BuildOptionsCore::default();

  if let Some(entries) = options_napi.entries {
    options.entries = Some(entries);
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

  if let Some(threads) = options_napi.threads {
    options.threads = Some(threads);
  }

  if let Some(project_root) = options_napi.project_root {
    options.project_root = Some(project_root);
  }

  let tsfn = env.create_threadsafe_function(
    &callback,
    0,
    |ctx: napi::threadsafe_function::ThreadSafeCallContext<BuildResultCore>| {
      let value = ctx.env.to_js_value(&ctx.value);
      Ok(vec![value])
    },
  )?;

  thread::spawn(move || {
    match mach_build_command(options) {
      Ok(result) => tsfn.call(Ok(result), ThreadsafeFunctionCallMode::NonBlocking),
      Err(error) => tsfn.call(
        Err(napi::Error::from_reason(error)),
        ThreadsafeFunctionCallMode::NonBlocking,
      ),
    };
  });

  env.get_undefined()
}
