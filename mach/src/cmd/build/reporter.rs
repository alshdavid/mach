use std::collections::HashMap;

use crate::public::AssetMap;
use crate::public::BundleMap;
use crate::public::Config;

pub struct AppReporter<'a> {
  config: &'a Config,
  time_transform: f64,
  time_bundle: f64,
  time_package: f64,
}

impl<'a> AppReporter<'a> {
  pub fn new(config: &'a Config) -> Self {
    return Self {
      config,
      time_transform: 0.0,
      time_bundle: 0.0,
      time_package: 0.0,
    };
  }
  pub fn print_config(&'a self) {
    println!(
      "Entry:         {}",
      self.config.entry_point.to_str().unwrap()
    );
    println!(
      "Root:          {}",
      self.config.project_root.to_str().unwrap()
    );
    if !&self.config.machrc.is_default {
      println!(
        "Mach Config:   {}",
        self.config.machrc.file_path.to_str().unwrap()
      );
    } else {
      println!("Mach Config:   Default");
    }
    println!("Out Dir:       {}", self.config.dist_dir.to_str().unwrap());
    println!("Optimize:      {}", self.config.optimize);
    println!("Threads:       {}", self.config.threads);
    println!("Splitting:     {}", self.config.bundle_splitting);
  }

  pub fn print_transform_stats(
    &mut self,
    asset_map: &AssetMap,
  ) {
    let time_transform = self.config.time_elapsed();
    println!(
      "  Transform:     {:.3}s  (Assets {})",
      time_transform,
      asset_map.len()
    );
    self.time_transform = time_transform;
  }

  pub fn print_bundle_stats(
    &mut self,
    bundles: &BundleMap,
  ) {
    let time_bundle = self.config.time_elapsed();
    let mut bundle_kinds = HashMap::<String, usize>::new();
    for bundle in bundles.iter() {
      if !bundle_kinds.contains_key(&bundle.kind) {
        bundle_kinds.insert(bundle.kind.clone(), 0);
      }
      *bundle_kinds.get_mut(&bundle.kind).unwrap() += 1;
    }
    print!(
      "  Bundle:        {:.3}s  (Bundles {}",
      time_bundle - self.time_transform,
      bundles.len()
    );
    for (bundle_kind, count) in bundle_kinds.iter() {
      print!(", {} {}", bundle_kind, count);
    }
    println!(")");
    self.time_bundle = time_bundle;
  }

  pub fn print_package_stats(&mut self) {
    let time_package = self.config.time_elapsed();
    println!("  Package:       {:.3}s", time_package - self.time_bundle);
    self.time_package = time_package;
  }

  pub fn print_emit_stats(&mut self) {
    let time_emit = self.config.time_elapsed();
    println!("  Emit:          {:.3}s", time_emit - self.time_package);
  }

  pub fn print_finished_stats(&mut self) {
    println!("Finished in:   {:.3}s", self.config.time_elapsed());
  }
}
