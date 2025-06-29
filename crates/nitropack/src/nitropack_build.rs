use std::sync::mpsc::Sender;
use std::sync::Arc;

use dyn_fs::FileSystem;

pub enum BuildEvent {
  Start,
}

#[derive(Clone, Debug)]
pub struct BuildOptions {}

pub async fn nitropack_build(
  _tx: Sender<BuildEvent>,
  _fs: Arc<dyn FileSystem>,
  _options: BuildOptions,
) {
  todo!()
}
