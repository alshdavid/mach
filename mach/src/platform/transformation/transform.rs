use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::JoinHandle;

use crate::platform::config::PluginContainer;
use crate::platform::config::TransformerTarget;
use crate::public;
use crate::public::Asset;
use crate::public::AssetGraph;
use crate::public::AssetMap;
use crate::public::Dependency;
use crate::public::DependencyMap;
use crate::public::DependencyOptions;
use crate::public::MutableAsset;
use crate::public::ENTRY_ASSET;

pub fn link_and_transform(
  config: &public::Config,
  plugins: &mut PluginContainer,
  asset_map: &mut AssetMap,
  dependency_map: &mut DependencyMap,
  asset_graph: &mut AssetGraph,
) -> Result<(), String> {
  // Take ownership of the bundling state while we transform the files.
  // We know they cannot be used elsewhere so this is safe to
  let config_local = Arc::new(config.clone());
  let plugins_local = Arc::new(std::mem::take(plugins));
  let asset_map_local = Arc::new(Mutex::new(std::mem::take(asset_map)));
  let dependency_map_local = Arc::new(Mutex::new(std::mem::take(dependency_map)));
  let asset_graph_local = Arc::new(Mutex::new(std::mem::take(asset_graph)));
  let in_progress = Arc::new(Mutex::new(HashSet::<PathBuf>::new()));
  let active_threads = Arc::new(AtomicUsize::new(0));
  let queue = Arc::new(Mutex::new(vec![]));

  let mut handles = Vec::<JoinHandle<Result<(), String>>>::new();
  let mut senders = Vec::<Sender<bool>>::new();
  let mut receivers = Vec::<Option<Receiver<bool>>>::new();

  queue.lock().unwrap().push(Dependency {
    specifier: config.entry_point.to_str().unwrap().to_string(),
    is_entry: true,
    source_path: ENTRY_ASSET.clone(),
    resolve_from: ENTRY_ASSET.clone(),
    ..Dependency::default()
  });

  for _ in 0..config.threads {
    let (tx, rx) = channel::<bool>();
    senders.push(tx.clone());
    receivers.push(Some(rx));
  }

  for i in 0..config.threads {
    let config = config_local.clone();
    let plugins = plugins_local.clone();
    let asset_map = asset_map_local.clone();
    let dependency_map = dependency_map_local.clone();
    let asset_graph = asset_graph_local.clone();
    let in_progress = in_progress.clone();
    let active_threads = active_threads.clone();
    let queue = queue.clone();
    let senders = senders.clone();
    let rx = receivers.get_mut(i).unwrap().take().unwrap();

    handles.push(std::thread::spawn(move || -> Result<(), String> {
      loop {
        let Some(dependency) = queue.lock().unwrap().pop() else {
          let Ok(kill) = rx.recv() else {
            break;
          };
          if (queue.lock().unwrap().len() == 0 && active_threads.load(Ordering::Relaxed) == 0)
            || kill
          {
            break;
          }
          continue;
        };

        active_threads.fetch_add(1, Ordering::Relaxed);

        let continue_threads = || {
          active_threads.fetch_sub(1, Ordering::Relaxed);
          for sender in &senders {
            let Ok(_) = sender.send(false) else {
              continue;
            };
          }
        };

        let kill_threads = || {
          for sender in &senders {
            let Ok(_) = sender.send(true) else {
              continue;
            };
          }
        };

        // Resolve Start
        let resolve_result = 'block: {
          for resolver in &plugins.resolvers {
            if let Some(resolve_result) = resolver.resolve(&dependency)? {
              break 'block resolve_result;
            }
          }
          kill_threads();
          return Err("Unable to resolve file".to_string());
        };
        let file_path_rel =
          pathdiff::diff_paths(&resolve_result.file_path, &config.project_root).unwrap();
        // Resolve Done

        // Dependency Graph
        let dependency_source_path = dependency.resolve_from_rel.clone();
        let dependency_bundle_behavior = dependency.bundle_behavior.clone();
        let dependency_id = dependency_map.lock().unwrap().insert(dependency);
        asset_graph.lock().unwrap().add_edge(
          dependency_source_path.clone(),
          (dependency_id, file_path_rel.clone()),
        );
        if asset_map.lock().unwrap().contains_key(&file_path_rel) {
          continue_threads();
          continue;
        }
        if !in_progress.lock().unwrap().insert(file_path_rel.clone()) {
          continue_threads();
          continue;
        }
        // Dependency Graph Done

        // Transformation
        let mut file_target = TransformerTarget::new(&resolve_result.file_path);

        let mut content =
          fs::read(&resolve_result.file_path).map_err(|_| "Unable to read file".to_string())?;
        let mut asset_dependencies = Vec::<DependencyOptions>::new();
        let mut asset_kind = file_target.file_extension.clone();

        let mut mutable_asset = MutableAsset::new(
          resolve_result.file_path.clone(),
          &mut asset_kind,
          &mut content,
          &mut asset_dependencies,
        );

        let (mut pattern, mut transformers) = plugins.transformers.get(&file_target)?;

        let mut i = 0;
        while i != transformers.len() {
          let Some(transformer) = transformers.get(i) else {
            break;
          };

          transformer.transform(&mut mutable_asset, &config)?;

          // If the file type and pattern changes restart transformers
          if *mutable_asset.kind != file_target.file_extension {
            file_target.update(mutable_asset.kind);

            let (new_pattern, new_transformers) = plugins.transformers.get(&file_target)?;
            // Use new transformers if they are different to current ones
            if new_pattern != pattern {
              transformers = new_transformers;
              pattern = new_pattern;
              i = 0;
              continue;
            }
          }

          i += 1;
        }

        asset_map.lock().unwrap().insert(Asset {
          name: file_target.file_stem.clone(),
          file_path_absolute: resolve_result.file_path.clone(),
          file_path_relative: file_path_rel.clone(),
          content,
          kind: asset_kind,
          bundle_behavior: dependency_bundle_behavior,
          ..Default::default()
        });
        // Transformation Done

        let mut new_dependencies = Vec::<Dependency>::new();

        // Add new items to the queue
        while let Some(dependency_options) = asset_dependencies.pop() {
          new_dependencies.push(Dependency {
            content_key: String::new(),
            specifier: dependency_options.specifier.clone(),
            specifier_type: dependency_options.specifier_type,
            is_entry: false,
            source_path: resolve_result.file_path.clone(),
            resolve_from: resolve_result.file_path.clone(),
            resolve_from_rel: file_path_rel.clone(),
            priority: dependency_options.priority,
            imported_symbols: dependency_options.imported_symbols,
            bundle_behavior: dependency_options.bundle_behavior,
            ..Default::default()
          });
        }

        in_progress.lock().unwrap().remove(&file_path_rel);
        queue.lock().unwrap().extend(new_dependencies);
        continue_threads();
      }
      return Ok(());
    }));
  }

  for handle in handles.drain(0..) {
    handle.join().unwrap().unwrap();
  }

  //Put the results of the transformation back into the bundle state
  *plugins = Arc::try_unwrap(plugins_local).unwrap();
  *asset_map = Arc::try_unwrap(asset_map_local)
    .unwrap()
    .into_inner()
    .unwrap();
  *dependency_map = Arc::try_unwrap(dependency_map_local)
    .unwrap()
    .into_inner()
    .unwrap();
  *asset_graph = Arc::try_unwrap(asset_graph_local)
    .unwrap()
    .into_inner()
    .unwrap();

  Ok(())
}
