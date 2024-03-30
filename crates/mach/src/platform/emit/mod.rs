use std::fs;

use libmach::MachConfig;
use libmach::BundleMap;
use libmach::Outputs;
use libmach::{self};

pub fn emit(
  config: &MachConfig,
  _bundles: &BundleMap,
  outputs: &Outputs,
) -> Result<(), String> {
  if config.dist_dir.exists() && config.clean_dist_dir {
    fs::remove_dir_all(&config.dist_dir).unwrap();
  }

  fs::create_dir_all(&config.dist_dir).unwrap();

  for output in outputs {
    let complete_path = config.dist_dir.join(&output.filepath);
    let basename = complete_path.parent().unwrap();
    fs::create_dir_all(basename).unwrap();
    fs::write(complete_path, output.content.as_slice()).unwrap();
  }

  Ok(())
}
