/*
  This contains the library bindings for the public API for Mach
*/
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;

use mach_bundler_core::BuildOptions;
use mach_bundler_core::Mach;
use mach_bundler_core::MachOptions;
use napi::bindgen_prelude::External;
use napi::bindgen_prelude::FromNapiValue;
use napi::Env;
use napi::JsNumber;
use napi::JsObject;
use napi::JsString;
use napi::JsUnknown;
use napi::ValueType;
use napi_derive::napi;
use serde::Deserialize;
use serde::Serialize;

#[napi(object)]
pub struct MachNapi {
  pub node_workers: u32,
  pub mach: External<Arc<Mach>>,
}

//
// NEW
//
#[napi]
pub fn mach_napi_new(
  env: Env,
  options: JsObject,
) -> napi::Result<MachNapi> {
  let mach_options = MachOptions {
    rpc_hosts: Default::default(),
    threads: 'block: {
      let Ok(value) = options.get_named_property::<JsNumber>("threads") else {
        break 'block Default::default();
      };
      value.get_uint32()? as usize
    },
    entries: 'block: {
      let Ok(value) = options.get_named_property::<JsUnknown>("entries") else {
        break 'block Default::default();
      };

      match value.get_type() {
        Ok(ValueType::String) => {
          let value = JsString::from_unknown(value)?;
          let value = PathBuf::from(value.into_utf8()?.as_str()?.to_string());
          vec![value]
        }
        Ok(ValueType::Object) => {
          let value = JsObject::from_unknown(value)?;
          let mut output = vec![];
          for i in 0..value.get_array_length()? {
            let element = value.get_element::<JsString>(i)?;
            let element = PathBuf::from(element.into_utf8()?.as_str()?.to_string());
            output.push(element)
          }
          output
        }
        _ => Default::default(),
      }
    },
    env: 'block: {
      let Ok(value) = options.get_named_property::<JsObject>("env") else {
        break 'block Default::default();
      };
      env.from_js_value(value)?
    },
    out_folder: 'block: {
      let Ok(value) = options.get_named_property::<JsString>("outFolder") else {
        break 'block std::env::current_dir().unwrap().join("dist");
      };
      env.from_js_value(value)?
    },
    project_root: PathBuf::from(
      options
        .get_named_property::<JsString>("projectRoot")?
        .into_utf8()?
        .as_str()?,
    ),
    config: Default::default(),
  };

  // dbg!(&mach_options);

  let mut node_workers = mach_options.threads.clone();
  if let Ok(value) = options.get_named_property::<JsNumber>("nodeWorkers") {
    node_workers = value.get_uint32()? as usize;
  };

  Ok(MachNapi {
    node_workers: node_workers as u32,
    mach: External::new(Arc::new(Mach::new(mach_options))),
  })
}

//
// BUILD
//
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildOptionsNapi {
  pub clean: Option<bool>,
  pub optimize: Option<bool>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildResultNapi {
  pub bundle_manifest: HashMap<String, String>,
  pub entries: HashMap<String, String>,
}

#[napi]
pub fn mach_napi_build(
  env: Env,
  this: MachNapi,
  options: JsObject,
) -> napi::Result<JsObject> {
  let options_napi = env.from_js_value::<BuildOptionsNapi, JsObject>(options)?;
  let mut options = BuildOptions::default();

  if let Some(clean) = options_napi.clean {
    options.clean = clean;
  }

  if let Some(optimize) = options_napi.optimize {
    options.optimize = optimize;
  }

  let (deferred, promise) = env.create_deferred()?;

  thread::spawn({
    let mach = this.mach.clone();
    move || match mach.build(options) {
      Ok(result) => deferred.resolve(move |env| {
        Ok(env.to_js_value(&BuildResultNapi {
          bundle_manifest: result.bundle_manifest,
          entries: result.entries,
        }))
      }),
      Err(err) => deferred.reject(napi::Error::from_reason(format!("{:?}", err))),
    }
  });

  Ok(promise)
}
