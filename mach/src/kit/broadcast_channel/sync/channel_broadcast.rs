use std::{sync::mpsc::{channel, Receiver, SendError, Sender}, thread};

#[derive(Clone)]
pub struct Subscribable<T: Clone + Send> {
  tx_subscribe: Sender<Sender<T>>
}

impl<T: Clone + Send> Subscribable<T> {
  pub fn subscribe(&self) -> Receiver<T> {
    let (tx_subscription, rx_subscription) = channel::<T>();
    self.tx_subscribe.send(tx_subscription).unwrap();
    rx_subscription
  }
}

pub fn channel_broadcast<T: Clone + Send + 'static>() -> (Sender<T>, Subscribable<T>){
  let (tx_t, rx_t) = channel::<T>();
  let (tx_subscribe, rx_subscribe) = channel::<Sender<T>>();

  thread::spawn(move || {
    let mut senders = Vec::<Option<Sender<T>>>::new();
    while let Ok(data) = rx_t.recv() {
      for sender_opt in senders.iter_mut() {
        let Some(sender) = sender_opt else {
          continue;
        };
        if sender.send(data.clone()).is_err() {
          sender_opt.take();
        }
      }
      while let Ok(sender) = rx_subscribe.try_recv() {
        if sender.send(data.clone()).is_ok() {
          senders.push(Some(sender));
        }
      }
    }
  });

  return (tx_t, Subscribable{ tx_subscribe });
}
