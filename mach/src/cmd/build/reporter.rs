#![allow(unused_must_use)]

use std::collections::HashMap;

use crate::public::AssetGraphSync;
use crate::public::AssetMapSync;
use crate::public::BundleGraphSync;
use crate::public::BundleMapSync;
use crate::public::DependencyMapSync;
use crate::public::MachConfigSync;
use crate::public::OutputsSync;

pub struct AppReporter {
  config: MachConfigSync,
  dependency_map: DependencyMapSync,
  asset_map: AssetMapSync,
  asset_graph: AssetGraphSync,
  bundles: BundleMapSync,
  bundle_graph: BundleGraphSync,
  outputs: OutputsSync,
  time_init: f64,
  time_transform: f64,
  time_bundle: f64,
  time_package: f64,
}

impl AppReporter {
  pub fn new(
    config: MachConfigSync,
    dependency_map: DependencyMapSync,
    asset_map: AssetMapSync,
    asset_graph: AssetGraphSync,
    bundles: BundleMapSync,
    bundle_graph: BundleGraphSync,
    outputs: OutputsSync,
  ) -> Self {
    return Self {
      config,
      dependency_map,
      asset_map,
      asset_graph,
      bundles,
      bundle_graph,
      outputs,
      time_init: 0.0,
      time_transform: 0.0,
      time_bundle: 0.0,
      time_package: 0.0,
    };
  }

  pub fn log(&self, v: &str) {
    println!("{}", v);
  }

  pub fn print_config(&self) {
    self.log(&format!(
      "Entry:         {}",
      self.config.entry_point.to_str().unwrap()
    ));
    self.log(&format!(
      "Root:          {}",
      self.config.project_root.to_str().unwrap()
    ));
    if !&self.config.machrc.is_default {
      self.log(&format!(
        "Mach Config:   {}",
        self.config.machrc.file_path.to_str().unwrap()
      ));
    } else {
      self.log(&format!("Mach Config:   Default"));
    }
    self.log(&format!("Out Dir:       {}", self.config.dist_dir.to_str().unwrap()));
    self.log(&format!("Optimize:      {}", self.config.optimize));
    self.log(&format!("Threads:       {}", self.config.threads));
    self.log(&format!("Splitting:     {}", self.config.bundle_splitting));
  }

  pub fn print_init_stats(&mut self) {
    let time_init = self.config.time_elapsed();
    self.log(&format!("  Init:          {:.3}s", time_init));
    self.time_init = time_init;
  }

  pub fn print_transform_stats(&mut self) {
    let time_transform = self.config.time_elapsed();
    self.log(&format!(
      "  Transform:     {:.3}s  (Assets {})",
      time_transform - self.time_init,
      self.asset_map.read().unwrap().len()
    ));
    self.time_transform = time_transform;

    if self.config.debug {
      dbg!(self.asset_map.read().unwrap());
      dbg!(self.dependency_map.read().unwrap());
      dbg!(self.asset_graph.read().unwrap());
    }
  }

  pub fn print_bundle_stats(&mut self) {
    let bundles = self.bundles.read().unwrap();
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
    self.log(&format!(")"));
    self.time_bundle = time_bundle;

    if self.config.debug {
      dbg!(self.bundles.read().unwrap());
      dbg!(self.bundle_graph.read().unwrap());
    }
  }

  pub fn print_package_stats(&mut self) {
    let time_package = self.config.time_elapsed();
    self.log(&format!("  Package:       {:.3}s", time_package - self.time_bundle));
    self.time_package = time_package;

    if self.config.debug {
      dbg!(self.outputs.read().unwrap());
    }
  }

  pub fn print_emit_stats(&mut self) {
    let time_emit = self.config.time_elapsed();
    self.log(&format!("  Emit:          {:.3}s", time_emit - self.time_package));
  }

  pub fn print_finished_stats(&mut self) {
    self.log(&format!("Finished in:   {:.3}s", self.config.time_elapsed()));
  }
}
