use std::{process::Child, sync::{atomic::{AtomicU8, Ordering}, Arc}};

#[derive(Clone)]
pub struct NodejsWorkerFarm {
  counter: Arc<AtomicU8>,
  children: Arc<Vec<Child>>,
}

impl NodejsWorkerFarm {
  pub fn new(instances: usize) -> Self {
    Self {
      children: Arc::new(vec![]),
      counter: Arc::new(AtomicU8::new(0))
    }
  }

  pub fn send(&self, msg: &[u8]) -> u8 {
    let i = self.counter.fetch_add(1, Ordering::Relaxed);
    let _ = self.counter.compare_exchange(3, 0, Ordering::AcqRel, Ordering::Relaxed);
    return i;
  }
}
 