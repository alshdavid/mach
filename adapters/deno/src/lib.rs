mod adapter;
mod deno_cli;
mod deno_embed;
mod deno_snapshots;
mod mach_extensions;

use std::sync::mpsc::channel;

use adapter::DenoAdapter;
use deno_embed::deno_current_thread;
use libmach::Adapter;
use libmach::AdapterBootstrapOptions;
use libmach::AdapterBootstrapResult;

use mach_extensions::mach_hello_world;
use mach_extensions::mach_load_resolver;

#[no_mangle]
pub extern "C" fn bootstrap(_config: AdapterBootstrapOptions) -> AdapterBootstrapResult {
  let (tx, rx) = channel::<String>();

  deno_current_thread(async {
    let options = deno_embed::DenoInitOptions{
      script: "/home/dalsh/Development/alshdavid/mach/adapters/deno/javascript/index.ts".to_string(),
      extensions: vec![
        mach_hello_world::init_ops_and_esm(),
        mach_load_resolver::init_ops_and_esm(),
      ],
      ..Default::default()
    };
    deno_embed::run_script(options).await.unwrap();
  });

  let adapter: Box<dyn Adapter> = Box::new(DenoAdapter {});
  return Box::new(Box::new(Ok(adapter)));
}
