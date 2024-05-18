use std::sync::{mpsc::{channel, Receiver, Sender}, Arc, Mutex};

use once_cell::sync::Lazy;
use napi_derive::napi;

pub type RegisterWorkerData = ();
pub type RegisterWorkerSender = Sender<RegisterWorkerData>;
pub type RegisterWorkerReceiver = Arc<Mutex<Option<Receiver<RegisterWorkerData>>>>;


pub static REGISTER_WORKER: Lazy<(RegisterWorkerSender, RegisterWorkerReceiver)> = Lazy::new(|| {
  let (tx, rx) = channel::<RegisterWorkerData>();
  (tx, Arc::new(Mutex::new(Some(rx))))
});

#[napi]
pub fn register_worker(id: i32)  {

}