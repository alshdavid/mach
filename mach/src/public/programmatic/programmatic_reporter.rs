use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::thread;

use serde_json::to_string;

use super::BuildReport;
use super::ProgrammaticAction;
use crate::public::AssetMapSync;
use crate::public::BundleManifestSync;
use crate::public::BundleMapSync;
use crate::public::MachConfigSync;

pub struct ProgrammaticReporter {
  asset_map: AssetMapSync,
  bundles: BundleMapSync,
  bundle_manifest: BundleManifestSync,
  tx: Sender<(Vec<u8>, Sender<()>)>,
  active: bool,
}

impl ProgrammaticReporter {
  pub fn new(
    config: MachConfigSync,
    asset_map: AssetMapSync,
    bundles: BundleMapSync,
    bundle_manifest: BundleManifestSync,
  ) -> Self {
    let (tx, rx) = channel::<(Vec<u8>, Sender<()>)>();
    let active = config.diagnostic_port.is_some();

    thread::spawn(move || {
      let Some(diagnostic_port) = config.diagnostic_port else {
        return;
      };
      let mut stream = TcpStream::connect(format!("127.0.0.1:{}", diagnostic_port)).unwrap();
      while let Ok((msg, tx)) = rx.recv() {
        stream.write_all(&msg).unwrap();
        tx.send(()).unwrap();
      }
    });

    Self {
      asset_map,
      bundles,
      bundle_manifest,
      tx,
      active,
    }
  }

  pub fn emit(
    &self,
    action: ProgrammaticAction,
  ) {
    if !self.active {
      return;
    }
    let (tx, rx) = channel::<()>();
    let mut bytes = to_string(&action).unwrap().as_bytes().to_vec();
    bytes.push(10);
    self.tx.send((bytes, tx)).unwrap();
    rx.recv().unwrap();
  }

  pub fn emit_build_report(&self) {
    if !self.active {
      return;
    }
    let asset_map = self.asset_map.read().unwrap();
    let bundles = self.bundles.read().unwrap();
    let bundle_manifest = self.bundle_manifest.read().unwrap();

    let mut build_report = BuildReport {
      bundle_manifest: HashMap::new(),
      entries: HashMap::new(),
    };

    for (key, value) in bundle_manifest.iter() {
      build_report
        .bundle_manifest
        .insert(key.clone(), value.clone());
    }

    for (_bundle_id, bundle) in bundles.iter() {
      let Some(asset_id) = &bundle.entry_asset else {
        continue;
      };
      let Some(asset) = asset_map.get(asset_id) else {
        continue;
      };
      let asset_file_path = asset.file_path_relative.to_str().unwrap().to_string();

      build_report
        .entries
        .insert(asset_file_path, bundle.name.clone());
    }

    self.emit(ProgrammaticAction::BuildReport { data: build_report })
  }
}
