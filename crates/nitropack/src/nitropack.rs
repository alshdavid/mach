use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;

use dyn_fs::sync::os::OsFileSystem;
use dyn_fs::sync::FileSystem;

use crate::nitropack_build;
use crate::nitropack_watch;
use crate::BuildEvent;
use crate::BuildOptions;
use crate::WatchEvent;
use crate::WatchOptions;

pub struct Nitropack {
  runtime: Arc<tokio::runtime::Runtime>,
  fs: Arc<dyn FileSystem>,
}

#[derive(Default)]
pub struct NitropackOptions {
  /// The number of threads to use for the build
  pub threads: Option<usize>,
  pub fs: Option<Arc<dyn FileSystem>>,
}

impl Nitropack {
  pub fn new(options: &NitropackOptions) -> std::io::Result<Self> {
    let runtime = Arc::new(match &options.threads {
      Some(1) => tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?,
      n => tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(n.unwrap_or_else(num_cpus::get))
        .build()?,
    });

    let fs = match &options.fs {
      Some(fs) => Arc::clone(fs),
      None => Arc::new(OsFileSystem),
    };

    Ok(Self { runtime, fs })
  }

  pub fn build(
    &self,
    options: &BuildOptions,
  ) -> impl Iterator<Item = BuildEvent> {
    let (tx, rx) = channel::<BuildEvent>();
    let options = options.clone();
    let rt = Arc::clone(&self.runtime);
    let fs = Arc::clone(&self.fs);
    thread::spawn(move || rt.block_on(nitropack_build(tx, fs, options)));
    rx.into_iter()
  }

  pub fn watch(
    &self,
    options: &WatchOptions,
  ) -> impl Iterator<Item = WatchEvent> {
    let (tx, rx) = channel::<WatchEvent>();
    let options = options.clone();
    let rt = Arc::clone(&self.runtime);
    let fs = Arc::clone(&self.fs);
    thread::spawn(move || rt.block_on(nitropack_watch(tx, fs, options)));
    rx.into_iter()
  }
}
