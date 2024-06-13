use std::path::Path;
use std::sync::Arc;

use petgraph::visit::EdgeRef;
use petgraph::visit::NodeRef;

use super::super::build_asset_graph;
use super::snapshot::GraphSnapshot;
use super::utils::test_project_snapshot;
use super::utils::FIXTURES;
use crate::platform::config::load_plugins;
use crate::platform::config::PluginContainerSync;
use crate::platform::transformation::testing::snapshot::DependencySnapshot;
use crate::platform::transformation::testing::snapshot::GraphSnapshotAsset;
use crate::platform::transformation::testing::snapshot::GraphSnapshotConfig;
use crate::platform::transformation::testing::snapshot::GraphSnapshotImport;
use crate::public::Compilation;
use crate::public::MachConfig;
use crate::public::MachConfigSync;
use crate::public::Machrc;
use crate::rpc::RpcHosts;

// Retries to try catch determinism issues
const RETRIES: usize = 5;

#[test]
fn test_js_cjs_a() {
  let project_name = "test_js_cjs_a";
  for _ in 0..RETRIES {
    test_project_snapshot(project_name, &FIXTURES.join("test_js_cjs_a"))
  }
}

#[test]
fn test_js_cjs_b() {
  let project_name = "test_js_cjs_b";
  for _ in 0..RETRIES {
    test_project_snapshot(project_name, &FIXTURES.join("test_js_cjs_b"))
  }
}

#[test]
fn test_js_esm_a() {
  let project_name = "test_js_esm_a";
  for _ in 0..RETRIES {
    test_project_snapshot(project_name, &FIXTURES.join(project_name))
  }
}

#[test]
fn test_js_esm_a2() {
  let project_name = "test_js_esm_a2";
  for _ in 0..RETRIES {
    test_project_snapshot(project_name, &FIXTURES.join(project_name))
  }
}

#[test]
fn test_js_esm_b() {
  let project_name = "test_js_esm_b";
  for _ in 0..RETRIES {
    test_project_snapshot(project_name, &FIXTURES.join(project_name))
  }
}

#[test]
fn test_js_esm_c() {
  let project_name = "test_js_esm_c";
  for _ in 0..RETRIES {
    test_project_snapshot(project_name, &FIXTURES.join(project_name))
  }
}

#[test]
fn test_js_esm_d() {
  let project_name = "test_js_esm_d";
  for _ in 0..RETRIES {
    test_project_snapshot(project_name, &FIXTURES.join(project_name))
  }
}

#[test]
fn test_js_esm_e() {
  let project_name = "test_js_esm_e";
  for _ in 0..RETRIES {
    test_project_snapshot(project_name, &FIXTURES.join(project_name))
  }
}

#[test]
fn test_js_esm_f() {
  let project_name = "test_js_esm_f";
  for _ in 0..RETRIES {
    test_project_snapshot(project_name, &FIXTURES.join(project_name))
  }
}

#[test]
fn test_js_esm_g() {
  let project_name = "test_js_esm_g";
  for _ in 0..RETRIES {
    test_project_snapshot(project_name, &FIXTURES.join(project_name))
  }
}

#[test]
fn test_js_esm_h() {
  let project_name = "test_js_esm_h";
  for _ in 0..RETRIES {
    test_project_snapshot(project_name, &FIXTURES.join(project_name))
  }
}

#[test]
fn test_js_esm_i() {
  let project_name = "test_js_esm_i";
  for _ in 0..RETRIES {
    test_project_snapshot(project_name, &FIXTURES.join(project_name))
  }
}
