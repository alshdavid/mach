use std::sync::mpsc::Receiver;
use std::sync::mpsc::SendError;
use std::sync::mpsc::Sender;
use std::thread;

use super::channel_broadcast::channel_broadcast;
use super::channel_broadcast::Subscribable;

#[derive(Clone)]
pub struct BroadcastChannel<T: Clone + Send + 'static> {
  tx: Sender<T>,
  rrx: Subscribable<T>,
}

impl<T: Clone + Send + 'static> BroadcastChannel<T> {
  pub fn new() -> Self {
    let (tx, rrx) = channel_broadcast();
    Self { tx, rrx }
  }

  pub fn send(
    &self,
    value: T,
  ) -> Result<(), SendError<T>> {
    self.tx.send(value)
  }

  pub fn subscribe(&self) -> Receiver<T> {
    self.rrx.subscribe()
  }

  pub fn merge(
    &self,
    receiver: Receiver<T>,
  ) {
    let tx = self.tx.clone();
    thread::spawn(move || {
      while let Ok(data) = receiver.recv() {
        tx.send(data).unwrap()
      }
    });
  }
}
