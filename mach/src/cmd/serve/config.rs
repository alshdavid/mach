use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct ServeCommand {
  /// The folder to serve
  pub target: PathBuf,

  /// Host address
  #[arg(long = "host", default_value = "127.0.0.1")]
  pub host: String,

  /// Port to serve on
  #[arg(short = 'p', long = "port")]
  pub port: usize,

  /// Allow cors requests
  #[arg(long = "cors")]
  pub cors: bool,

  /// SSL Cert
  #[arg(short = 'S', long = "ssl-cert")]
  pub ssl_cert: Option<String>,

  /// SSL Key
  #[arg(short = 's', long = "ssl-key")]
  pub ssl_key: Option<String>,
}
