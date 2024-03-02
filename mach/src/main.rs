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

#[derive(Parser, Debug)]
struct Commands {
  #[clap(subcommand)]
  command: CommandType,
}

fn main() {
  let command = Commands::parse();

  match command.command {
    CommandType::Build(command) => {
      cmd::build::main(command);
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
