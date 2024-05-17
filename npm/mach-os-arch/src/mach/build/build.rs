use std::collections::HashMap;
use std::path::PathBuf;

use mach_bundler_core::BuildOptions as BuildOptionsCore;
use mach_bundler_core::Mach as MachCore;
use napi::Env;
use napi::JsObject;
use napi::JsUnknown;
use serde::Deserialize;
use serde::Serialize;

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
) -> napi::Result<JsUnknown> {
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

  if let Some(node_workers) = options_napi.node_workers {
    options.node_workers = Some(node_workers);
  }

  if let Some(project_root) = options_napi.project_root {
    options.project_root = Some(project_root);
  }

  match MachCore::new().build(options) {
    Ok(report) => {
      let js_result = env.to_js_value(&BuildResult {
        bundle_manifest: report.bundle_manifest,
        entries: report.entries,
      })?;

      return Ok(js_result);
    }
    Err(error) => {
      return Err(napi::Error::from_reason(error));
    }
  };
}
