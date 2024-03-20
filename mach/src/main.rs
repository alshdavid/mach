#![deny(unused_crate_dependencies)]

mod cmd;
mod kit;
mod platform;
mod public;

use clap::Parser;
use clap::Subcommand;
use cmd::build::BuildCommand;
use cmd::dev::DevCommand;
use cmd::version::VersionCommand;
use cmd::watch::WatchCommand;

/*
  Main just acts as a router to run CLI commands
*/

#[derive(Debug, Subcommand)]
pub enum CommandType {
  /// Build a project
  Build(BuildCommand),
  /// Start a web server and reload when changes are detected
  Dev(DevCommand),
  /// Build and rebuild when changes are detected
  Watch(WatchCommand),
  /// Print version information
  Version(VersionCommand),
}

fn main() {
  let command = Commands::parse();

  match command.command {
    CommandType::Build(command) => {
      if let Err(msg) = cmd::build::main(command) {
        println!("Build Error\n{}", msg);
      };
    }
    CommandType::Dev(command) => {
      cmd::dev::main(command);
    }
    CommandType::Watch(command) => {
      cmd::watch::main(command);
    }
    CommandType::Version(_) => {
      cmd::version::main();
    }
  }
}

#[derive(Parser, Debug)]
struct Commands {
  #[clap(subcommand)]
  command: CommandType,
}
