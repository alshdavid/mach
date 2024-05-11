use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::error::SendError;

use super::{channel_broadcast, Subscribable};

#[derive(Clone)]
pub struct BroadcastChannel<T: Clone + Send + 'static> {
  tx: UnboundedSender<T>,
  rrx: Subscribable<T>
}

impl<T: Clone + Send + 'static> BroadcastChannel<T> {
  pub fn new() -> Self {
    let (tx, rrx) = channel_broadcast();
    Self {tx, rrx}
  }

  pub fn send(&self, value: T) -> Result<(), SendError<T>> {
    self.tx.send(value)
  }

  pub fn subscribe(&self) -> UnboundedReceiver<T> {
    self.rrx.subscribe()
  }

  pub fn merge(&self, mut receiver: UnboundedReceiver<T>) {
    let tx = self.tx.clone();
    tokio::spawn(async move {
      while let Some(data) = receiver.recv().await {
        tx.send(data).unwrap()
      }
    });
  }
}
