use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;

use swc_core::common::SourceMap;

use crate::default_plugins::resolver::resolve;
use crate::default_plugins::transformers::javascript::transformer;
use crate::platform::Subject;
use crate::public;
use crate::public::Asset;
use crate::public::AssetId;
use crate::public::AssetMap;
use crate::public::Dependency;
use crate::public::DependencyKind;
use crate::public::DependencyMap;

type ImportSpecifier = String;

pub fn transform(
  config: &public::Config,
  asset_map: &mut AssetMap,
  dependency_map: &mut DependencyMap,
  source_map: Arc<SourceMap>,
) -> Result<(), String> {
  // Take the mut state so it can be owned and in threads
  let og_asset_map = asset_map;
  let og_dependency_map = dependency_map;
  let asset_map = std::mem::take(og_asset_map);
  let dependency_map = std::mem::take(og_dependency_map);

  let asset_map = Arc::new(Mutex::new(asset_map));
  let dependency_map = Arc::new(Mutex::new(dependency_map));
  let asset_filepath_reference = Arc::new(Mutex::new(HashMap::<PathBuf, AssetId>::new()));
  let queue = Arc::new(Mutex::new(Vec::<(
    AssetId,
    (ImportSpecifier, DependencyKind),
  )>::new()));
  let in_flight = Arc::new(AtomicUsize::new(0));
  let mut on_queue_add = Subject::<bool>::new(config.threads.clone());
  let mut thread_handles = Vec::<JoinHandle<Result<(), String>>>::new();

  let entry_specifier = ImportSpecifier::from(config.entry_point.to_str().unwrap());
  queue.lock().unwrap().push((
    AssetId::default(),
    (entry_specifier, DependencyKind::Static),
  ));
  in_flight.fetch_add(1, Ordering::Relaxed);

  for i in 0..config.threads {
    let on_queue_add_receiver = on_queue_add.receivers[i].take().unwrap();
    let on_queue_add = on_queue_add.clone();
    let queue = queue.clone();
    let in_flight = in_flight.clone();
    let config = config.clone();
    let source_map = source_map.clone();
    let asset_map = asset_map.clone();
    let dependency_map = dependency_map.clone();
    let asset_filepath_reference = asset_filepath_reference.clone();

    thread_handles.push(thread::spawn(move || {
      'main_loop: loop {
        // Pop the queue, otherwise wait for a notification to try again
        let Some((parent_asset_id, (import_specifier, dependency_kind))) =
          queue.lock().unwrap().pop()
        else {
          let should_continue = on_queue_add_receiver.recv().unwrap();
          if !should_continue {
            break 'main_loop;
          }
          continue;
        };

        // Get filepath to parent asset
        let parent_asset_path = 'block: {
          let asset_map = asset_map.lock().unwrap();
          // If it's the first asset then the parent is the root path
          if asset_map.len() == 0 {
            break 'block config.project_root.clone();
          }
          // Use the asset if we find the parent's ID
          if let Some(parent_asset) = asset_map.get(&parent_asset_id) {
            break 'block parent_asset.file_path.clone();
          }
          return Err(format!(
            "Could not find parent with ID: {:?}",
            parent_asset_id
          ));
        };

        // Get filepath to current asset
        let Ok(new_asset_absolute_path) = resolve(&parent_asset_path, &import_specifier) else {
          return Err(format!(
            "Could not resolve specifier {} from {:?}",
            import_specifier, parent_asset_path
          ));
        };

        // If the asset already exists then link the dependency and continue on
        if let Some(existing_asset_id) = asset_filepath_reference
          .lock()
          .unwrap()
          .get(&new_asset_absolute_path)
        {
          dependency_map.lock().unwrap().insert(
            existing_asset_id,
            Dependency {
              parent_asset_id,
              target_asset_id: existing_asset_id.clone(),
              import_specifier,
              kind: dependency_kind,
            },
          );
          continue;
        }

        // Read the contents of the asset
        let Ok(asset_contents) = fs::read_to_string(&new_asset_absolute_path) else {
          return Err(format!("File Read Error: {:?}", new_asset_absolute_path));
        };

        // Parse JavaScript
        let Ok((program, mut dependencies)) = transformer(
          &new_asset_absolute_path,
          &asset_contents,
          source_map.clone(),
          &config,
        ) else {
          return Err(format!("File Parse Error: {:?}", new_asset_absolute_path));
        };

        // Create and commit new Asset
        let new_asset = Asset::new(
          &config.project_root,
          &new_asset_absolute_path,
          &asset_contents,
          program,
        );

        dependency_map.lock().unwrap().insert(
          &parent_asset_id.clone(),
          Dependency {
            parent_asset_id,
            target_asset_id: new_asset.id.clone(),
            import_specifier,
            kind: dependency_kind,
          },
        );

        let new_asset_id = new_asset.id.clone();
        asset_map.lock().unwrap().insert(new_asset);

        let mut queue = queue.lock().unwrap();

        if dependencies.len() != 0 {
          in_flight.fetch_add(dependencies.len(), Ordering::Relaxed);

          while let Some(dependencies) = dependencies.pop() {
            queue.push((
              new_asset_id.clone(),
              (dependencies.specifier, dependencies.kind),
            ));
          }
        }

        // If there are no more jobs in flight, send kill signal
        if in_flight.fetch_sub(1, Ordering::Relaxed) == 1 {
          on_queue_add.send(false).unwrap();
        } else {
          on_queue_add.send(true).unwrap();
        };
      }
      return Ok(());
    }));
  }

  for handle in thread_handles {
    let Err(err) = handle.join().unwrap() else {
      continue;
    };
    return Err(err);
  }

  std::mem::swap(og_asset_map, &mut asset_map.lock().unwrap());
  std::mem::swap(og_dependency_map, &mut dependency_map.lock().unwrap());
  Ok(())
}
