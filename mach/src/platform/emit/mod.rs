use std::fs;

use crate::public::MachConfigSync;
use crate::public::OutputsSync;

pub async fn emit(
  config: MachConfigSync,
  outputs: OutputsSync,
) -> Result<(), String> {
  let outputs = outputs.read().await;

  if config.dist_dir.exists() && config.clean_dist_dir {
    fs::remove_dir_all(&config.dist_dir).unwrap();
  }

  fs::create_dir_all(&config.dist_dir).unwrap();

  for output in outputs.iter() {
    let complete_path = config.dist_dir.join(&output.filepath);
    let basename = complete_path.parent().unwrap();
    fs::create_dir_all(basename).unwrap();
    fs::write(complete_path, output.content.as_slice()).unwrap();
  }

  Ok(())
}
