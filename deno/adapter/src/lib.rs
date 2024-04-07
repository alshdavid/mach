use deno_embed::deno_current_thread;

mod deno_embed;
mod deno_cli;
mod deno_snapshots;

#[no_mangle]
pub extern "C" fn init_deno() {
  std::thread::spawn(move || {
    deno_current_thread(async move {
      let options = deno_embed::DenoInitOptions {
        script: "/home/dalsh/Development/alshdavid/mach/deno/adapter/javascript/index.js"
          .to_string(),
        ..Default::default()
      };
      let mut worker = deno_embed::run_script(options).await.unwrap();

      worker.run().await.unwrap();
    });
  }).join().unwrap();
}