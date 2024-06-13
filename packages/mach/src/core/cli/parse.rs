use clap::Parser;
use clap::Subcommand;

use super::BuildCommand;
use super::DevCommand;
use super::VersionCommand;
use super::WatchCommand;

#[derive(Parser, Debug)]
pub struct Commands {
  #[clap(subcommand)]
  pub command: CommandType,
}

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

pub fn parse_options<T: AsRef<str>>(input: &[T]) -> Result<Commands, String> {
  let mut updated_input = vec!["".to_string()];
  for item in input {
    updated_input.push(item.as_ref().to_owned());
  }

  match Commands::try_parse_from(updated_input) {
    Ok(command) => Ok(command),
    Err(error) => Err(format!("{}", error)),
  }
}

pub fn parse_options_from_cli() -> Commands {
  Commands::parse()
}
