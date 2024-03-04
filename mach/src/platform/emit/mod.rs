use std::fs;

use crate::public::Bundles;
use crate::public::Outputs;
use crate::public::{self};

pub fn emit(
  config: &public::Config,
  _bundles: &Bundles,
  outputs: &Outputs,
) -> Result<(), String> {
  fs::create_dir_all(&config.dist_dir).unwrap();

  for output in outputs {
    fs::write(config.dist_dir.join(&output.filepath), output.content.as_slice()).unwrap();
  }
  
  Ok(())
}
