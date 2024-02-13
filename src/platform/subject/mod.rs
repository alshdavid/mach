#![allow(dead_code)]
use kanal::unbounded;
use kanal::Receiver;
use kanal::SendError;
use kanal::Sender;

#[derive(Clone)]
pub struct Subject<T: Send + Clone> {
  senders: Vec<Sender<T>>,
  pub receivers: Vec<Option<Receiver<T>>>,
}

impl<T: Send + Clone> Subject<T> {
  pub fn new(listeners: usize) -> Self {
    let mut senders = Vec::<Sender<T>>::new();
    let mut receivers = Vec::<Option<Receiver<T>>>::new();

    for _ in 0..listeners {
      let (tx, rx) = unbounded::<T>();
      senders.push(tx);
      receivers.push(Some(rx));
    }

    return Subject { senders, receivers };
  }

  pub fn send(
    &self,
    data: T,
  ) -> Result<(), SendError> {
    for sender in &self.senders {
      if let Err(err) = sender.send(data.clone()) {
        return Err(err);
      }
    }
    return Ok(());
  }
}
