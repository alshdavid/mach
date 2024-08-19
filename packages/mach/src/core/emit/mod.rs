use crate::types::Compilation;

pub fn emit(_c: &mut Compilation) -> anyhow::Result<()> {
  // let outputs = outputs.read().unwrap();

  // if config.dist_dir.exists() && config.clean_dist_dir {
  //   fs::remove_dir_all(&config.dist_dir).unwrap();
  // }

  // fs::create_dir_all(&config.dist_dir).unwrap();

  // for output in outputs.iter() {
  //   let complete_path = config.dist_dir.join(&output.filepath);
  //   let basename = complete_path.parent().unwrap();
  //   fs::create_dir_all(basename).unwrap();
  //   fs::write(complete_path, output.content.as_slice()).unwrap();
  // }

  Ok(())
}
