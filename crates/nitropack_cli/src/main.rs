mod build;
mod dev;
mod watch;

use clap::Parser;
use clap::Subcommand;
use nitropack::BuildOptions;
use nitropack::Nitropack;
use nitropack::NitropackOptions;
use nitropack::WatchOptions;

#[derive(Parser, Debug)]
struct NitropackCommand {
  #[clap(subcommand)]
  pub command: NitropackSubcommand,
}

#[derive(Debug, Subcommand)]
enum NitropackSubcommand {
  /// Build a project
  Build(build::BuildCommand),
  /// Start a web server and reload when changes are detected
  Dev(dev::DevCommand),
  /// Build and rebuild when changes are detected
  Watch(watch::WatchCommand),
}

fn main() {
  match NitropackCommand::parse().command {
    NitropackSubcommand::Build(_build_command) => {
      let _nitropack = Nitropack::new(&NitropackOptions {
        ..Default::default()
      })
      .unwrap()
      .build(&BuildOptions {});
    }
    NitropackSubcommand::Watch(_watch_command) => {
      let _nitropack = Nitropack::new(&NitropackOptions {
        ..Default::default()
      })
      .unwrap()
      .watch(&WatchOptions {});
    }
    NitropackSubcommand::Dev(_watch_command) => {
      // Nothing
    }
  };
}
