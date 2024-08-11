use clap::Parser;
use clap::Subcommand;

use super::BuildCommand;
use super::DevCommand;
use super::VersionCommand;
use super::WatchCommand;

#[derive(Debug, Subcommand)]
pub enum MachCommandType {
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
pub struct MachCommand {
  #[clap(subcommand)]
  pub command: MachCommandType,
}

impl MachCommand {
  /// Parse CLI options form arguments obtained in std::env::args_os()
  pub fn from_os_args() -> Self {
    MachCommand::parse()
  }

  /// Parse CLI options from string vec/slice
  pub fn from_args<T: AsRef<str>>(input: &[T]) -> Result<Self, String> {
    let mut updated_input = vec!["".to_string()];
    for item in input {
      updated_input.push(item.as_ref().to_owned());
    }

    match MachCommand::try_parse_from(updated_input) {
      Ok(command) => Ok(command),
      Err(error) => Err(format!("{}", error)),
    }
  }
}
