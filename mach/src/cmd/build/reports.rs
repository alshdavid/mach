use std::collections::HashMap;

use crate::public::AssetMap;
use crate::public::Bundles;
use crate::public::Config;

pub fn config(config: &Config) {
  println!("Entry:         {}", config.entry_point.to_str().unwrap());
  println!("Root:          {}", config.project_root.to_str().unwrap());
  if !&config.machrc.is_default {
    println!(
      "Mach Config:   {}",
      config.machrc.file_path.to_str().unwrap()
    );
  } else {
    println!("Mach Config:   Default");
  }
  println!("Out Dir:       {}", config.dist_dir.to_str().unwrap());
  println!("Optimize:      {}", config.optimize);
  println!("Threads:       {}", config.threads);
  println!("Splitting:     {}", config.bundle_splitting);
}

pub fn transform_stats(
  config: &Config,
  asset_map: &AssetMap,
) -> f64 {
  let time_transform = config.time_elapsed();
  println!(
    "  Transform:     {:.3}s  (Assets {})",
    time_transform,
    asset_map.len()
  );
  return time_transform;
}

pub fn bundle_stats(
  time_transform: f64,
  config: &Config,
  bundles: &Bundles,
) -> f64 {
  let time_bundle = config.time_elapsed();
  let mut bundle_kinds = HashMap::<String, usize>::new();
  for bundle in bundles.iter() {
    if !bundle_kinds.contains_key(&bundle.kind) {
      bundle_kinds.insert(bundle.kind.clone(), 0);
    }
    *bundle_kinds.get_mut(&bundle.kind).unwrap() += 1;
  }
  print!(
    "  Bundle:        {:.3}s  (Bundles {}",
    time_bundle - time_transform,
    bundles.len()
  );
  for (bundle_kind, count) in bundle_kinds.iter() {
    print!(", {} {}", bundle_kind, count);
  }
  println!(")");
  return time_bundle;
}

pub fn package_stats(
  time_bundle: f64,
  config: &Config,
) -> f64 {
  let time_package = config.time_elapsed();
  println!("  Package:       {:.3}s", time_package - time_bundle);
  return time_package;
}

pub fn emit_stats(
  time_package: f64,
  config: &Config,
) {
  let time_emit = config.time_elapsed();
  println!("  Emit:          {:.3}s", time_emit - time_package);
}

pub fn finished_stats(config: &Config) {
  println!("Finished in:   {:.3}s", config.time_elapsed());
}
