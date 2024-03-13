use std::path::PathBuf;
use std::time::Duration;

use deno_core::Extension;
use deno_core::Op;
use deno_embed::op2;
use deno_embed::deno_current_thread;
use deno_embed::deno_napi::v8;
use deno_embed::run_script;
use deno_embed::DenoInitOptions;
use deno_embed::ModuleCodeString;
use deno_core::*;

#[op2]
pub fn op_hello_world(
  #[global] f: v8::Global<v8::Function>,
) {
  println!("hi");
}

// deno_core::extension!(
//   deno_runtime,
//   ops = [op_main_module, op_ppid],
//   options = { main_module: ModuleSpecifier },
//   state = |state, options| {
//     state.put::<ModuleSpecifier>(options.main_module);
//   },
// );

deno_core::extension!(
  mach_foo,
  ops = [op_hello_world],
  js = ["src/test.js"],
);

fn main() { 
  // let my_ext = Extension {
  //   name: "mach:foo",
  //   ops: std::borrow::Cow::Borrowed(&[op_hello_world::DECL]),
  //   op_state_fn: Some(Box::new(|state| {})),
  //   enabled: true,
  //   ..Default::default()
  // };


  deno_embed::deno_current_thread(async move {
    let exit_code = deno_embed::run_script(
      deno_embed::DenoInitOptions{ // Combination of RunFlags and Flags
        script: "/home/dalsh/Development/alshdavid/mach/pkg/index.js".to_string(),
        extensions: vec![mach_foo::init_ops_and_esm()],
        ..Default::default()
      })
      .await;

    println!("{:?}", exit_code);
  });
}

// mod cmd;
// mod kit;
// mod platform;
// mod public;

// use clap::Parser;
// use clap::Subcommand;
// use cmd::build::BuildCommand;
// use cmd::dev::DevCommand;
// use cmd::version::VersionCommand;
// use cmd::watch::WatchCommand;

// /*
//   Main just acts as a router to run CLI commands
// */
// #[derive(Debug, Subcommand)]
// pub enum CommandType {
//   /// Build a project
//   Build(BuildCommand),
//   /// Start a web server and reload when changes are detected
//   Dev(DevCommand),
//   /// Build and rebuild when changes are detected
//   Watch(WatchCommand),
//   /// Print version information
//   Version(VersionCommand),
// }

// fn main() {
//   let command = Commands::parse();

//   match command.command {
//     CommandType::Build(command) => {
//       cmd::build::main(command);
//     }
//     CommandType::Dev(command) => {
//       cmd::dev::main(command);
//     }
//     CommandType::Watch(command) => {
//       cmd::watch::main(command);
//     }
//     CommandType::Version(_) => {
//       cmd::version::main();
//     }
//   }
// }

// #[derive(Parser, Debug)]
// struct Commands {
//   #[clap(subcommand)]
//   command: CommandType,
// }
