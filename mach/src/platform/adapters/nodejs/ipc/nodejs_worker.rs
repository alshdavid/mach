use std::sync::{mpsc::Receiver, Arc};

/* 
  This is a wrapper around Nodejs that allows consumers to communicate
  with Nodejs regardless of how that's achieved

  This could represent a Nodejs process or Nodejs worker

  Windows consumers will talk to the Nodejs child using TCP while Unix
  consumers will use named sockets or named pipes

  In future I will investigate alternative approaches to communicate
  with the Nodejs child, like shared memory
*/
pub trait NodejsWorker: Send + Sync {
  fn send(&self, bytes: Vec<u8>);
  fn subscribe(&self) -> Receiver<u8>;
}

pub trait NodejsWorkerFactory: Send+ Sync {
  fn spawn(&self) -> Arc<dyn NodejsWorker>;
}
