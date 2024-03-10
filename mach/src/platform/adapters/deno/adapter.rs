use std::path::PathBuf;

use tokio::sync::mpsc::unbounded_channel;

use super::internal::deno_current_thread;



pub struct DenoAdapter {}

impl DenoAdapter {
  pub fn new() -> Self {
    // let (tx, rx) = unbounded_channel::<()>();

    std::thread::spawn(move || {
      deno_current_thread(async {
        
      });
    });

    return DenoAdapter{};
  }
}