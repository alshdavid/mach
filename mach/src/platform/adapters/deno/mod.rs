use mach_libdeno::DenoWrapper;
use mach_libdeno::DenoConfig;

#[derive(Debug, Default)]
pub struct DenoAdapter {
  deno: Box<DenoWrapper>
}

impl DenoAdapter {
  pub fn init(config: DenoConfig) -> Self {
    let exe_path = std::env::current_exe().unwrap();
    let bin_path = exe_path.parent().unwrap();
    let lib_path = bin_path.join("deno.so");

    let deno = unsafe {
      let Ok(lib) = libloading::Library::new(lib_path) else {
        panic!();
      };
      let init_deno: libloading::Symbol<extern fn(Box<DenoConfig>) -> Box<DenoWrapper>> =
        lib.get(b"init_deno").unwrap();
  
      init_deno(Box::new(config))
    };

    Self { deno }
  }
}
