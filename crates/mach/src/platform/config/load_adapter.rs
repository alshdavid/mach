use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use libloading::Library;
use libmach::Adapter;
use libmach::AdapterBootstrapFn;
use libmach::AdapterOptions;
use libmach::MachConfig;
use once_cell::sync::Lazy;

static LIBS: Lazy<Arc<Mutex<HashMap<String, Arc<Library>>>>> =
  Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

pub fn load_dynamic_adapter(
  config: &MachConfig,
  engine_name: &str,
) -> Result<Box<dyn Adapter>, String> {
  let exe_path = std::env::current_exe().unwrap();
  let exe_dir = exe_path.parent().unwrap();
  let mach_dir = exe_dir.parent().unwrap();
  let mach_lib_dir = mach_dir.join("adapters");
  let lib_dir = mach_lib_dir.join(engine_name);
  let lib_name = libloading::library_filename("");
  let lib_path = lib_dir.join(lib_name);
  let lib_str = lib_path.to_str().unwrap().to_string();

  let lib = unsafe {
    let Ok(lib) = libloading::Library::new(&lib_path) else {
      return Err(format!(
        "Unable to load adapter: \"{}\"\nLooking for:\n\t{:?}",
        engine_name, lib_path
      ));
    };
    Arc::new(lib)
  };

  LIBS.lock().unwrap().insert(lib_str.clone(), lib.clone());

  println!("1");

  let bootstrap: libloading::Symbol<AdapterBootstrapFn> = unsafe { 
    let Ok(bootstrap) = lib.get(b"bootstrap") else {
      return Err(format!(
        "Unable to load bootstrap form adapter: \"{}\"\nFrom for:\n\t{:?}",
        engine_name, lib_path
      ));
    };
    bootstrap
  };
  println!("2");

  let bootstrap_fn = **bootstrap(Box::new(AdapterOptions {
    config: config.clone(),
  }));

  println!("3");

  let adapter = bootstrap_fn?;

  return Ok(adapter);
}
