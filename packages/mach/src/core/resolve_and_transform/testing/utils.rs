use std::path::Path;
use std::path::PathBuf;

use once_cell::sync::Lazy;
use petgraph::visit::EdgeRef;
use petgraph::visit::NodeRef;

use super::super::resolve_and_transform;
use super::snapshot::GraphSnapshot;
use crate::core::plugins::load_plugins;
use crate::types::Compilation;
use crate::types::MachConfig;

pub static SNAPSHOT_FILENAME: &str = "__graph.json";
pub static CARGO_DIR: Lazy<PathBuf> = Lazy::new(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")));
pub static FIXTURES: Lazy<PathBuf> = Lazy::new(|| {
  CARGO_DIR
    .join("src")
    .join("platform")
    .join("transformation")
    .join("testing")
    .join("__fixtures")
});

pub fn setup_root<T: AsRef<str>>(
  project_root: &Path,
  entries: &[T],
) -> Compilation {
  let mut compilation = Compilation {
    machrc: Default::default(),
    rpc_hosts: Default::default(),
    config: MachConfig {
      entries: entries
        .iter()
        .map(|e| PathBuf::from(e.as_ref().to_string()))
        .collect::<Vec<PathBuf>>(),
      project_root: project_root.to_owned(),
      threads: 1,
      env: Default::default(),
      out_folder: Default::default(),
    },
    asset_contents: Default::default(),
    asset_graph: Default::default(),
    plugins: Default::default(),
  };

  load_plugins(&mut compilation).unwrap();

  compilation
}

pub fn test_project_snapshot(
  project_name: &str,
  project_root: &Path,
) {
  let snapshot_path = project_root.join(SNAPSHOT_FILENAME);
  if !snapshot_path.exists() {
    return;
  }

  let snapshot = GraphSnapshot::load(&snapshot_path);

  let mut c = setup_root(&project_root, &snapshot.get_entries());

  if let Err(msg) = resolve_and_transform(&mut c) {
    println!("{msg}");
    panic!()
  };

  let graph = c.asset_graph.as_graph();

  for node_index in graph.node_indices().into_iter() {
    let mut edges = c.asset_graph.get_dependencies(&node_index);
    let source_asset = c.asset_graph.get_asset(node_index).unwrap();

    let Some(snap_asset) = snapshot.config.get(&source_asset.file_path_relative) else {
      panic!(
        "Error: {}\n\tMissing Asset\n\tExpected Asset: {:?}\nGot: null",
        project_name, source_asset.file_path_relative
      )
    };

    while let Some(edge) = edges.next() {
      let dest_asset = c.asset_graph.get_asset(edge.target().id()).unwrap();
      let dependency = edge.weight();

      let Some(snap_dependency) = snap_asset.imports.get(&dependency.specifier) else {
        panic!(
          "Error: {}\n\tMissing Specifier\n\tExpected Asset: {:?}\n\tWith Specifier {}\n\tGot: null",
          project_name, source_asset.file_path_relative, dependency.specifier
        )
      };

      assert!(
        snap_dependency.resolves_to == dest_asset.file_path_relative,
        "Error: {}\n\tIncorrect Dependency Resolved Path\n\tImport: {:?}\n\tSpecifier: {}\n\tExpected: {:?}\n\tGot: {:?}",
        project_name,
        source_asset.file_path_relative,
        dependency.specifier,
        snap_dependency.resolves_to,
        dest_asset.file_path_relative
      );

      assert!(
        snap_dependency.specifier == dependency.specifier,
        "Error: {}\n\tIncorrect Dependency Specifier\n\tImport: {:?}\n\tExpected: {:?}\n\tGot: {:?}",
        project_name,
        source_asset.file_path_relative,
        snap_dependency.specifier,
        dependency.specifier,
      );

      assert!(
        snap_dependency.specifier_type == dependency.specifier_type,
        "Error: {}\n\tIncorrect Specifier Type\n\tImport: {:?}\n\tSpecifier: {:?}\n\tExpected: {:?}\n\tGot: {:?}",
        project_name,
        source_asset.file_path_relative,
        dependency.specifier,
        snap_dependency.specifier_type,
        dependency.specifier_type,
      );

      assert!(
            snap_dependency.priority == dependency.priority,
            "Error: {}\n\tIncorrect Priority\n\tImport: {:?}\n\tSpecifier: {:?}\n\tExpected: {:?}\n\tGot: {:?}",
            project_name,
            source_asset.file_path_relative,
            dependency.specifier,
            snap_dependency.priority,
            dependency.priority,
          );

      // assert!(
      //       snap_dependency.linking_symbols.len() == dependency.linking_symbols.len(),
      //       "Error: {}\n\tIncorrect Imported Symbols Length\n\tImport: {:?}\n\tSpecifier: {:?}\n\tExpected: {:?}\n\tGot: {:?}",
      //       project_name,
      //       source_asset.file_path_relative,
      //       dependency.specifier,
      //       snap_dependency.linking_symbols.len(),
      //       dependency.linking_symbols.len(),
      //     );

      // for imported_symbol in dependency.linking_symbol.iter() {
      //   assert!(
      //       snap_dependency.linking_symbols.contains(imported_symbol),
      //       "Error: {}\n\tMissing Import Symbol\n\tImport: {:?}\n\tSpecifier: {:?}\n\tExpected: {:?}\n\tGot: {:?}",
      //       project_name,
      //       source_asset.file_path_relative,
      //       dependency.specifier,
      //       snap_dependency.linking_symbols,
      //       dependency.linking_symbol,
      //     )
      // }

      assert!(
          snap_dependency.bundle_behavior == dependency.bundle_behavior,
          "Error: {}\n\tIncorrect Bundle Behavior\n\tImport: {:?}\n\tSpecifier: {:?}\n\tExpected: {:?}\n\tGot: {:?}",
          project_name,
          source_asset.file_path_relative,
          dependency.specifier,
          snap_dependency.bundle_behavior,
          dependency.bundle_behavior,
        );
    }

    assert!(
      snap_asset.kind == source_asset.kind,
      "Error: {}\n\tIncorrect Asset Kind\n\tAsset: {:?}\n\tExpected: {:?}\n\tGot: {:?}",
      project_name,
      source_asset.file_path_relative,
      snap_asset.kind,
      source_asset.kind,
    );

    assert!(
      snap_asset.bundle_behavior == source_asset.bundle_behavior,
      "Error: {}\n\tIncorrect Asset Bundle Behavior\n\tAsset: {:?}\n\tExpected: {:?}\n\tGot: {:?}",
      project_name,
      source_asset.file_path_relative,
      snap_asset.bundle_behavior,
      source_asset.bundle_behavior,
    );

    assert!(
      snap_asset.linking_symbols.len() == source_asset.linking_symbols.len(),
      "Error: {}\n\tIncorrect Exported Symbols Length\n\tAsset: {:?}\n\tExpected: {:?}\n\tGot: {:?}",
      project_name,
      source_asset.file_path_relative,
      snap_asset.linking_symbols.len(),
      source_asset.linking_symbols.len(),
    );

    for linking_symbols in source_asset.linking_symbols.iter() {
      assert!(
        snap_asset.linking_symbols.contains(linking_symbols),
        "Error: {}\n\tMissing Linking Symbol\n\tAsset: {:?}\n\tExpected: {:?}\n\tGot: {:?}",
        project_name,
        source_asset.file_path_relative,
        snap_asset.linking_symbols,
        source_asset.linking_symbols,
      )
    }
  }
}
