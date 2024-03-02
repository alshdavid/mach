use std::fs;

use crate::kit::swc::render_module;
use crate::public::Bundles;
use crate::public::Packages;
use crate::public::{self};

pub fn emit(
  config: &public::Config,
  bundles: &mut Bundles,
  packages: &mut Packages,
) -> Result<(), String> {
  fs::create_dir_all(&config.dist_dir).unwrap();

  for (bundle_id, package) in packages {
    match package {
      public::PackageType::JavaScript((module, cm)) => {
        let bundle = bundles.iter().find(|x| x.id == *bundle_id).unwrap();
        let rendered = render_module(&module, cm.clone());
        fs::write(config.dist_dir.join(&bundle.output), rendered).unwrap();
      } // public::PackageType::CSS => todo!(),
        // public::PackageType::HTML => todo!(),
        // public::PackageType::File => todo!(),
    }
  }

  Ok(())
}
