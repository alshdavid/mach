// pub mod internal;
// mod adapter;

// pub use self::adapter::*;


/*
mod cmd;
mod kit;
mod platform;
mod public;

use platform::adapters::deno::internal::*;

fn main() {
  deno_current_thread(async {
    run_deno(RunDenoOptions { 
      cwd: std::env::current_exe().unwrap().parent().unwrap().to_path_buf(),
      specifier: "main.js".into(),
      eval: "console.log(fetch);".into()
    }).await;
  });
}

*/