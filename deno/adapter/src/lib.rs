mod deno_cli;
mod deno_embed;
mod deno_snapshots;
mod mach_extensions;
mod message_type;

use deno_embed::deno_current_thread;
use mach_extensions::ON_CONNECT;

use mach_libdeno::DenoConfig;
use mach_libdeno::DenoWorkerFarm;
use mach_libdeno::DenoWrapper;

#[no_mangle]
pub extern "C" fn init_deno(config: Box<DenoConfig>) -> Box<DenoWrapper> {
  let rx = ON_CONNECT.lock().unwrap().1.take().unwrap();
  let threads = config.threads.clone();

  std::thread::spawn(move || {
    deno_current_thread(async move {
      let options = deno_embed::DenoInitOptions {
        script: "/home/dalsh/Development/alshdavid/mach/deno/adapter/javascript/index.js"
          .to_string(),
        args: vec![format!("{}", threads)],
        ..Default::default()
      };
      let mut worker = deno_embed::run_script(options).await.unwrap();

      worker.run().await.unwrap();
    });
  });

  let worker_farm = DenoWorkerFarm::from_rx(rx, config.threads.clone());
  return Box::new(DenoWrapper::new(worker_farm));
}
