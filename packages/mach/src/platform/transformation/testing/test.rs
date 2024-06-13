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
  for _ in 0..RETRIES {
    test_project_snapshot("", &FIXTURES.join("test_js_cjs_a"))
  }
}

#[test]
fn test_js_cjs_b() {

}

#[test]
fn test_js_esm_a() {

}

#[test]
fn test_js_esm_a2() {

}

#[test]
fn test_js_esm_b() {

}

#[test]
fn test_js_esm_c() {

}

#[test]
fn test_js_esm_d() {

}

#[test]
fn test_js_esm_e() {

}

#[test]
fn test_js_esm_f() {

}

#[test]
fn test_js_esm_g() {

}

#[test]
fn test_js_esm_h() {

}

#[test]
fn test_js_esm_i() {

}
