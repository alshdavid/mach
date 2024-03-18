use std::collections::HashMap;

use libmach::Adapter;
use libmach::AdapterBootstrapFn;

pub async fn load_dynamic_adapter(engine_name: &str) -> Result<Box<dyn Adapter>, String> {
  let exe_path = std::env::current_exe().unwrap();
  let exe_dir = exe_path.parent().unwrap();
  let mach_dir = exe_dir.parent().unwrap();
  let mach_lib_dir = mach_dir.join("adapters");
  let lib_dir = mach_lib_dir.join(engine_name);
  let lib_name = libloading::library_filename("lib");
  let lib_path = lib_dir.join(lib_name);
  unsafe {
    let Ok(lib) = libloading::Library::new(&lib_path) else {
      return Err(format!("Unable to load library {:?}", lib_path));
    };
    let bootstrap: libloading::Symbol<AdapterBootstrapFn> = lib.get(b"bootstrap").unwrap();
    bootstrap(Box::new(HashMap::new())).await
  }
}
