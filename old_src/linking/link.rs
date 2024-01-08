use core::panic;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;

use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;
use std::thread;
use std::thread::JoinHandle;

use swc_core::common::source_map;
use swc_core::common::sync::Lrc;
use swc_core::common::SourceMap;

use crate::app_config::AppConfig;
use crate::public::Asset;
use crate::public::AssetId;
use crate::public::AssetMap;
use crate::public::AssetMapRef;
use crate::public::Dependency;
use crate::public::DependencyIndexRef;
use crate::public::DependencyKind;
use crate::public::DependencyMap;
use crate::public::DependencyMapRef;
use crate::public::SourceMapRef;

use super::ImportReadResult;
use super::generate_dependency_index;
use super::parse;
use super::read_imports;
use super::resolve;
use super::DependencyIndex;

pub fn link(
  config: &AppConfig,
  asset_map_ref: &mut AssetMapRef,
  dependency_map_ref: &mut DependencyMapRef,
  dependency_index_ref: &mut DependencyIndexRef,
  source_map_ref: &mut SourceMapRef,
) -> Result<(), String> {
  let assets_map = Arc::new(Mutex::new(asset_map_ref.take().unwrap()));
  let dependency_map = Arc::new(Mutex::new(dependency_map_ref.take().unwrap()));
  let source_map = Arc::new(source_map_ref.take().unwrap());
  let mut dependency_index = dependency_index_ref.take().unwrap();

  let queue = Arc::new(Mutex::new(VecDeque::<(AssetId, ImportReadResult)>::new()));
  let (senders, mut receivers) = create_channels(config.threads);
  
  let in_flight = Arc::new(AtomicUsize::new(0));
  let assets_done = Arc::new(Mutex::new(HashSet::<PathBuf>::new()));

  in_flight.fetch_add(1, Ordering::Relaxed);
  queue.lock().unwrap().push_back((
    Asset::generate_id(&config.project_root, &config.project_root),
    ImportReadResult{
      specifier: config.entry_point.to_str().unwrap().to_string(),
      kind: DependencyKind::Static,
    }
  ));

  let mut thread_handles = Vec::<JoinHandle<Result<(), String>>>::new();

  for i in 0..config.threads {
    let config = config.clone();
    let assets_map = assets_map.clone();
    let dependency_map = dependency_map.clone();
    let assets_done = assets_done.clone();
    let in_flight = in_flight.clone();
    let source_map = source_map.clone();
    let queue = queue.clone();
    let senders = senders.clone();
    let receiver = receivers[i].take().unwrap();

    thread_handles.push(thread::spawn(move || {
      let send = |data: Option<()>| {
        for sender in &senders {
          let Ok(_) = sender.send(data) else {continue;};
        }
      };

      let kill_threads = || {
        send(None);
      };

      let job_complete = || {
        if in_flight.fetch_sub(1, Ordering::Relaxed) == 1 {
          kill_threads();
        };
      };

      let job_add = |asset_id: AssetId, import_read_result: ImportReadResult| {
        in_flight.fetch_add(1, Ordering::Relaxed);
        queue.lock().unwrap().push_back((asset_id, import_read_result));
        send(Some(()));
      };

      loop {
        let Some(item) = queue.lock().unwrap().pop_front() else {
          if let Ok(msg) = receiver.recv() { if let Some(_) = msg {
            continue;
          }}
          break;
        };
        let (from_asset_id, import_read_result) = item; 

        let from_path = {
          let assets_map = &assets_map.lock().unwrap();

          if let Some(asset) = assets_map.get(&from_asset_id) {
            asset.file_path.clone()
          } else if assets_map.len() == 0 {
            config.project_root.clone()
          } else {
            kill_threads();
            return Err("Failed to lookup asset in map".to_string());
          }
        };

        let new_absolute_path = match resolve(&from_path, &import_read_result.specifier) {
          Ok(v) => v,
          Err(err) => {
            kill_threads();
            return Err(err);
          }
        };

        let mut new_asset = Asset::new(&config.project_root, &new_absolute_path);

        {
          let dependency_map = &mut dependency_map.lock().unwrap();

          if !dependency_map.contains_key(&from_asset_id) {
            dependency_map.insert(from_asset_id.clone(), HashMap::new());
          }

          if let Some(dependencies) = dependency_map.get_mut(&from_asset_id) {
            dependencies.insert(
              import_read_result.specifier.clone(),
              Dependency {
                specifier: import_read_result.specifier,
                asset_id: new_asset.id.clone(),
                kind: import_read_result.kind,
              },
            );
          }
        }

        if !assets_done
          .lock()
          .unwrap()
          .insert(new_absolute_path.clone())
        {
          job_complete();
          continue;
        }

        let contents = match fs::read_to_string(&new_absolute_path) {
          Ok(v) => v,
          Err(err) => {
            kill_threads();
            return Err(format!("File Read Error: {}", err));
          }
        };

        let (mut module, _comments) = match parse(&new_absolute_path, &contents, source_map.clone())
        {
          Ok(v) => v,
          Err(err) => {
            kill_threads();
            return Err(format!("File Parse Error: {}", err));
          }
        };

        let dependencies = read_imports(&mut module);
        new_asset.ast = module;
        let new_asset_id = new_asset.id.clone();

        if let Some(exists) = assets_map
          .lock()
          .unwrap()
          .insert(new_asset.id.clone(), new_asset.clone()) {
          return Err(format!(
            "Key Conflict:\n\tTRIED: {}: {:?}\n\tFOUND: {}: {:?}", 
            new_asset.id, new_asset.file_path_relative, exists.id, exists.file_path_relative
          ));
        };

        for dependency in dependencies {
          job_add(new_asset_id.clone(), dependency);
        }

        job_complete();
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

  //
  // Put values back into containers
  //
  let assets_map = {
    let Ok(asset_map) = Arc::try_unwrap(assets_map) else { panic!() };
    asset_map.into_inner().unwrap()
  };
  
  let dependency_map = {
    let Ok(dependency_map) = Arc::try_unwrap(dependency_map) else { panic!() };
    dependency_map.into_inner().unwrap()
  };

  let source_map = {
    let Ok(source_map) = Arc::try_unwrap(source_map) else { panic!() };
    source_map
  };

  generate_dependency_index(&mut dependency_index, &assets_map, &dependency_map);

  asset_map_ref.replace(assets_map);
  dependency_map_ref.replace(dependency_map);
  dependency_index_ref.replace(dependency_index);
  source_map_ref.replace(source_map);
  return Ok(());
}

pub fn create_channels(n: usize) -> (Vec<Sender<Option<()>>>, Vec<Option<Receiver<Option<()>>>>) {
  let mut senders = Vec::<Sender<Option<()>>>::new();
  let mut receivers = Vec::<Option<Receiver<Option<()>>>>::new();

  for _ in 0..n {
    let (s, r) = channel::<Option<()>>();
    senders.push(s);
    receivers.push(Some(r));
  }

  return (senders, receivers);
}