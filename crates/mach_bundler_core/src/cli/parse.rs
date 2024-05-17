use clap::Parser;
use clap::Subcommand;

use crate::mach::BuildOptions;
use crate::mach::DevOptions;
use crate::mach::WatchOptions;
use crate::mach::VersionOptions;

#[derive(Parser, Debug)]
pub struct Commands {
  #[clap(subcommand)]
  pub command: CommandType,
}

#[derive(Debug, Subcommand)]
pub enum CommandType {
  /// Build a project
  Build(BuildOptions),
  /// Start a web server and reload when changes are detected
  Dev(DevOptions),
  /// Build and rebuild when changes are detected
  Watch(WatchOptions),
  /// Print version information
  Version(VersionOptions),
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
