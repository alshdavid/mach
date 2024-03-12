// use std::path::PathBuf;
// use std::time::Duration;

// use deno_embed::deno_current_thread;
// use deno_embed::init_deno;
// use deno_embed::DenoInitOptions;
// use deno_embed::ModuleCodeString;

fn main() {
  // deno_current_thread(async move {
  //   let mut main_worker = init_deno(DenoInitOptions{
  //     ..Default::default()
  //   }).await;

  //   main_worker
  //     .execute_main_module(&deno_embed::ModuleSpecifier::from_file_path("/home/dalsh/Development/alshdavid/mach/pkg/index.js")
  //     .unwrap())
  //     .await
  //     .unwrap();

  //   // main_worker.execute_main_module(module_specifier)

  //   // main_worker.execute_script("main.js", ModuleCodeString::from_static(r#"
  //   //   (async () => {
  //   //     console.log("num", 42)
  //   //     const value = await new Promise(res => setTimeout(() => res('hi'), 1000))
  //   //     console.log("value", value)
  //   //     const result = await fetch('http://icanhazip.com').then(r => r.text())
  //   //     console.log("result", result)
  //   //   })()
  //   // "#)).unwrap();

  //   main_worker.run_event_loop(false).await.unwrap();
  // });
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
