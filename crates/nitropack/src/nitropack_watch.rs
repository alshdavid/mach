use std::sync::mpsc::Sender;
use std::sync::Arc;

use dyn_fs::FileSystem;

pub enum WatchEvent {
  Start,
}

#[derive(Clone, Debug)]
pub struct WatchOptions {}

pub async fn nitropack_watch(
  _tx: Sender<WatchEvent>,
  _fs: Arc<dyn FileSystem>,
  _options: WatchOptions,
) {
  todo!()
}
