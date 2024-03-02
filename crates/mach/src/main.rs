mod cmd;

use clap::Subcommand;
use clap::Parser;
use cmd::build::BuildCommand;
use cmd::dev::DevCommand;
use cmd::watch::WatchCommand;
use cmd::version::VersionCommand;

#[derive(Debug, Subcommand)]
pub enum CommandType {
  Build(BuildCommand),
  Dev(DevCommand),
  Watch(WatchCommand),
  Version(VersionCommand),
}

#[derive(Parser, Debug)]
struct Commands {
  #[clap(subcommand)]
  command: CommandType
}

fn main() {
  let command = Commands::parse();

  match command.command {
    CommandType::Build(command) => {
      cmd::build::main(command);
    },
    CommandType::Dev(command) => {
      cmd::dev::main(command);
    },
    CommandType::Watch(command) => {
      cmd::watch::main(command);
    },
    CommandType::Version(command) => {
      cmd::version::main(command);
    },
  }
}