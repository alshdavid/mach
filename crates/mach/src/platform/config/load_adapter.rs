use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use libloading::Library;
use libmach::Adapter;
use libmach::AdapterBootstrapFn;
use once_cell::sync::Lazy;

static LIBS: Lazy<Arc<Mutex<HashMap<String, Arc<Library>>>>> = Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

pub fn load_dynamic_adapter(engine_name: &str) -> Result<Box<dyn Adapter>, String> {
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
      return Err(format!("Unable to load adapter: \"{}\"\nLooking for:\n\t{:?}", engine_name, lib_path));
    };
    Arc::new(lib)
  };

  LIBS.lock().unwrap().insert(lib_str.clone(), lib.clone());

  let bootstrap: libloading::Symbol<AdapterBootstrapFn> = unsafe {
    lib.get(b"bootstrap").unwrap()
  };

  let bootstrap_fn = **bootstrap(Box::new(HashMap::new()));

  let adapter = bootstrap_fn?;

  return Ok(adapter);
}
