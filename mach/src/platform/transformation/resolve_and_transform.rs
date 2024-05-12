use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::thread::JoinHandle;

use super::resolve_and_transform_worker::resolve_and_transform_worker;
use crate::platform::config::PluginContainerSync;
use crate::platform::config::ENTRY_ASSET;
use crate::public::AssetGraphSync;
use crate::public::AssetMapSync;
use crate::public::Dependency;
use crate::public::DependencyMapSync;
use crate::public::MachConfigSync;

pub fn resolve_and_transform(
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

  // This schedules work on the configured number of worker threads.
  // Currently this is using a thread parking approach, but this is quite
  // messy so I will probably revisit this eventually
  //
  // It's hard to believe, but this is the fastest method I have tested
  // despite the aggressive use of locks
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
        // If there are no dependencies in the queue then park the thread
        let Some(dependency) = queue.write().unwrap().pop() else {
          // Wake up when thread gets a signal
          let Ok(should_exit) = rx.recv() else {
            break;
          };
          // Exit if kill signal received
          if should_exit {
            break;
          }
          // Exit if there are no more items in the queue or no threads are active
          if queue.read().unwrap().len() == 0 && active_threads.load(Ordering::Relaxed) == 0 {
            break;
          }
          // Otherwise park thread again
          continue;
        };

        // Mark this thread as active
        active_threads.fetch_add(1, Ordering::Relaxed);

        if let Err(msg) = resolve_and_transform_worker(
          &config,
          &plugins,
          &asset_map,
          &asset_graph,
          &dependency_map,
          &queue,
          dependency,
        ) {
          // Send kill signal to running threads on error
          for sender in &senders {
            let Ok(_) = sender.send(true) else {
              continue;
            };
          }
          return Err(msg);
        }

        // Mark thread as inactive
        active_threads.fetch_sub(1, Ordering::Relaxed);

        // Send wake signal to parked threads
        for sender in &senders {
          let Ok(_) = sender.send(false) else {
            continue;
          };
        }
      }

      Ok(())
    }));
  }

  for handle in handles.drain(0..) {
    handle.join().unwrap().unwrap();
  }

  Ok(())
}
