use std::fs;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::thread::JoinHandle;

use crate::platform::config::PluginContainerSync;
use crate::platform::config::TransformerTarget;
use crate::platform::config::ENTRY_ASSET;
use crate::public::Asset;
use crate::public::AssetGraphSync;
use crate::public::AssetMapSync;
use crate::public::Dependency;
use crate::public::DependencyMapSync;
use crate::public::DependencyOptions;
use crate::public::MachConfigSync;
use crate::public::MutableAsset;

pub fn link_and_transform(
  config: MachConfigSync,
  plugins: PluginContainerSync,
  asset_map: AssetMapSync,
  asset_graph: AssetGraphSync,
  dependency_map: DependencyMapSync,
) -> Result<(), String> {
  let active_threads = Arc::new(AtomicUsize::new(0));
  let queue = Arc::new(RwLock::new(vec![]));

  let mut handles = Vec::<JoinHandle<Result<(), String>>>::new();
  let mut senders = Vec::<Sender<bool>>::new();
  let mut receivers = Vec::<Option<Receiver<bool>>>::new();

  queue.write().unwrap().push(Dependency {
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

  for t in 0..config.threads {
    let config = config.clone();
    let plugins = plugins.clone();
    let asset_map = asset_map.clone();
    let dependency_map = dependency_map.clone();
    let asset_graph = asset_graph.clone();
    let active_threads = active_threads.clone();
    let queue = queue.clone();
    let senders = senders.clone();
    let rx = receivers.get_mut(t).unwrap().take().unwrap();

    handles.push(thread::spawn(move || {
      loop {
        let Some(dependency) = queue.write().unwrap().pop() else {
          let Ok(kill) = rx.recv() else {
            break;
          };
          if (queue.read().unwrap().len() == 0 && active_threads.load(Ordering::Relaxed) == 0)
            || kill
          {
            break;
          }
          continue;
        };

        active_threads.fetch_add(1, Ordering::Relaxed);

        let wake_threads = || {
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

        // Resolve Done

        // Dependency Graph
        let dependency_bundle_behavior = dependency.bundle_behavior.clone();
        let dependency_id = dependency.id.clone();
        let source_asset = dependency.source_asset.clone();

        dependency_map.write().unwrap().insert(dependency);

        let (asset_id, inserted) = asset_map.write().unwrap().get_or_insert(Asset {
          file_path_absolute: resolve_result.file_path.clone(),
          ..Default::default()
        });

        asset_graph.write().unwrap().add_edge(
          source_asset.clone(),
          asset_id.clone(),
          dependency_id.clone(),
        );

        if !inserted {
          wake_threads();
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

        {
          let mut asset_map = asset_map.write().unwrap();
          let asset = asset_map.get_mut(&asset_id).unwrap();
          asset.name = file_target.file_stem.clone();
          asset.content = content;
          asset.kind = asset_kind;
          asset.file_path_relative =
            pathdiff::diff_paths(&resolve_result.file_path, &config.project_root).unwrap();
          asset.bundle_behavior = dependency_bundle_behavior;
        }

        // Transformation Done

        let mut new_dependencies = Vec::<Dependency>::new();

        // Add new items to the queue
        while let Some(dependency_options) = asset_dependencies.pop() {
          let new_dependency = Dependency {
            specifier: dependency_options.specifier.clone(),
            specifier_type: dependency_options.specifier_type,
            is_entry: false,
            source_path: resolve_result.file_path.clone(),
            source_asset: asset_id.clone(),
            resolve_from: resolve_result.file_path.clone(),
            priority: dependency_options.priority,
            imported_symbols: dependency_options.imported_symbols,
            bundle_behavior: dependency_options.bundle_behavior,
            ..Default::default()
          };

          new_dependencies.push(new_dependency);
        }

        queue.write().unwrap().extend(new_dependencies);
        wake_threads();
      }
      return Ok(());
    }));
  }

  for handle in handles.drain(0..) {
    handle.join().unwrap().unwrap();
  }

  Ok(())
}
