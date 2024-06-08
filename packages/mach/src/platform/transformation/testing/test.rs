use std::path::Path;
use std::sync::Arc;

use petgraph::visit::EdgeRef;
use petgraph::visit::NodeRef;

use super::super::build_asset_graph;
use super::fixtures::GraphSnapshot;
use super::fixtures::FIXTURES;
use crate::platform::config::load_plugins;
use crate::platform::config::PluginContainerSync;
use crate::platform::transformation::testing::fixtures::DependencySnapshot;
use crate::platform::transformation::testing::fixtures::GraphSnapshotAsset;
use crate::platform::transformation::testing::fixtures::GraphSnapshotConfig;
use crate::platform::transformation::testing::fixtures::GraphSnapshotImport;
use crate::platform::transformation::testing::fixtures::SNAPSHOT_FILENAME;
use crate::public::RpcHosts;
use crate::public::Compilation;
use crate::public::MachConfig;
use crate::public::MachConfigSync;
use crate::public::Machrc;

fn setup_root<T: AsRef<str>>(
  project_root: &Path,
  entries: &[T],
) -> (MachConfigSync, PluginContainerSync, Compilation) {
  let mach_config = Arc::new(MachConfig {
    entries: entries
      .iter()
      .map(|e| e.as_ref().to_string())
      .collect::<Vec<String>>(),
    project_root: project_root.to_owned(),
    ..Default::default()
  });

  let plugins = load_plugins(&mach_config, &Machrc::default(), &RpcHosts::new()).unwrap();
  let compilation = Compilation::new();
  (mach_config, plugins, compilation)
}

// #[test]
fn should_produce_correct_graphs_for_fixtures() {
  // Retries to try catch determinism issues
  let retries = 5;

  // Generate snapshot
  //
  // TODO create an dedicated stable snapshot generator outside of the bundler
  // so it can be used for snapshot generation while allowing for the implementation
  // within the bundler to be updated
  if let Ok(target) = std::env::var("UPDATE_SNAPSHOT") {
    if target == "*" {
      for dir in std::fs::read_dir(&*FIXTURES).unwrap() {
        let project_root = dir.unwrap().path();
        generate_project_snapshot(&project_root)
      }
    } else {
      let project_root = FIXTURES.join(target);
      generate_project_snapshot(&project_root);
    }
    return;
  };

  if let Ok(target) = std::env::var("RUN_SNAPSHOT") {
    let project_root = FIXTURES.join(target);
    let project_name = project_root
      .file_name()
      .unwrap()
      .to_str()
      .unwrap()
      .to_string();

    test_project_snapshot(&project_name, &project_root);
    return;
  }

  for dir in std::fs::read_dir(&*FIXTURES).unwrap() {
    let project_root = dir.unwrap().path();

    let project_name = project_root
      .file_name()
      .unwrap()
      .to_str()
      .unwrap()
      .to_string();

    println!("{}", project_name);
    for _ in 0..retries {
      test_project_snapshot(&project_name, &project_root)
    }
  }
}

fn test_project_snapshot(
  project_name: &str,
  project_root: &Path,
) {
  let snapshot_path = project_root.join(SNAPSHOT_FILENAME);
  if !snapshot_path.exists() {
    return;
  }

  let snapshot = GraphSnapshot::load(&snapshot_path);

  let (mach_config, plugins, mut c) = setup_root(&project_root, &snapshot.get_entries());

  if let Err(msg) = build_asset_graph(mach_config.clone(), plugins, &mut c) {
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
        snap_dependency.is_entry == dependency.is_entry,
        "Error: {}\n\tIncorrect Is Entry\n\tImport: {:?}\n\tExpected: {:?}\n\tGot: {:?}",
        project_name,
        source_asset.file_path_relative,
        snap_dependency.is_entry,
        dependency.is_entry,
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

      assert!(
            snap_dependency.imported_symbols.len() == dependency.imported_symbols.len(),
            "Error: {}\n\tIncorrect Imported Symbols Length\n\tImport: {:?}\n\tSpecifier: {:?}\n\tExpected: {:?}\n\tGot: {:?}",
            project_name,
            source_asset.file_path_relative,
            dependency.specifier,
            snap_dependency.imported_symbols.len(),
            dependency.imported_symbols.len(),
          );

      for imported_symbol in dependency.imported_symbols.iter() {
        assert!(
            snap_dependency.imported_symbols.contains(imported_symbol),
            "Error: {}\n\tMissing Import Symbol\n\tImport: {:?}\n\tSpecifier: {:?}\n\tExpected: {:?}\n\tGot: {:?}",
            project_name,
            source_asset.file_path_relative,
            dependency.specifier,
            snap_dependency.imported_symbols,
            dependency.imported_symbols,
          )
      }

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
      snap_asset.exported_symbols.len() == source_asset.exported_symbols.len(),
      "Error: {}\n\tIncorrect Exported Symbols Length\n\tAsset: {:?}\n\tExpected: {:?}\n\tGot: {:?}",
      project_name,
      source_asset.file_path_relative,
      snap_asset.exported_symbols.len(),
      source_asset.exported_symbols.len(),
    );

    for exported_symbol in source_asset.exported_symbols.iter() {
      assert!(
        snap_asset.exported_symbols.contains(exported_symbol),
        "Error: {}\n\tMissing Import Symbol\n\tAsset: {:?}\n\tExpected: {:?}\n\tGot: {:?}",
        project_name,
        source_asset.file_path_relative,
        snap_asset.exported_symbols,
        source_asset.exported_symbols,
      )
    }
  }
}

fn generate_project_snapshot(project_root: &Path) {
  let snapshot_path = project_root.join(SNAPSHOT_FILENAME);
  println!("{:?}", snapshot_path);

  let (mach_config, plugins, mut c) = setup_root(&project_root, &["./index.js"]);

  if let Err(msg) = build_asset_graph(mach_config.clone(), plugins, &mut c) {
    println!("{msg}");
    panic!()
  };

  let mut snapshot = GraphSnapshotConfig::default();
  let graph = c.asset_graph.as_graph();

  for node_index in graph.node_indices().into_iter() {
    let source_asset = c.asset_graph.get_asset(node_index).unwrap();

    let mut snapshot_imports = GraphSnapshotImport::default();

    let mut edges = c.asset_graph.get_dependencies(&node_index);

    while let Some(edge) = edges.next() {
      let dest_asset = c.asset_graph.get_asset(edge.target().id()).unwrap();
      let dependency = edge.weight();

      snapshot_imports.insert(
        dependency.specifier.clone(),
        DependencySnapshot {
          resolves_to: dest_asset.file_path_relative.clone(),
          specifier: dependency.specifier.clone(),
          specifier_type: dependency.specifier_type.clone(),
          is_entry: dependency.is_entry.clone(),
          priority: dependency.priority.clone(),
          imported_symbols: dependency.imported_symbols.clone(),
          reexported_symbols: dependency.reexported_symbols.clone(),
          bundle_behavior: dependency.bundle_behavior.clone(),
        },
      );
    }

    let snapshot_entry = GraphSnapshotAsset {
      file_path: source_asset.file_path_relative.clone(),
      exported_symbols: source_asset.exported_symbols.clone(),
      kind: source_asset.kind.clone(),
      bundle_behavior: source_asset.bundle_behavior.clone(),
      imports: snapshot_imports,
    };

    snapshot.insert(source_asset.file_path_relative.clone(), snapshot_entry);
  }

  GraphSnapshot::save(&snapshot_path, snapshot);
}
