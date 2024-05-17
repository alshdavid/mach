use std::path::PathBuf;

use mach_bundler_core::BuildOptions as BuildOptionsCore;
use mach_bundler_core::Mach as MachCore;
use napi::JsObject;
use napi::Env;
use napi_derive::napi;
use serde::Serialize;
use serde::Deserialize;

use super::mach::Mach;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MachBuildOptions {
  pub entries: Option<Vec<PathBuf>>,
  pub out_folder: Option<PathBuf>,
  pub clean: Option<bool>,
  pub optimize: Option<bool>,
  pub bundle_splitting: Option<bool>,
  pub threads: Option<usize>,
  pub node_workers: Option<usize>,
}

#[napi]
impl Mach {
  #[napi]
  pub fn build(&self, env: Env, options: JsObject) -> napi::Result<()> {
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

    let _result = MachCore::new().build(options).unwrap();
    println!("Build complete");
    return Ok(());
  }
}
