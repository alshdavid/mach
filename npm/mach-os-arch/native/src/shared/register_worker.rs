use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;

use napi_derive::napi;
use once_cell::sync::Lazy;

pub type RegisterWorkerData = ();
pub type RegisterWorkerSender = Sender<RegisterWorkerData>;
pub type RegisterWorkerReceiver = Arc<Mutex<Option<Receiver<RegisterWorkerData>>>>;

pub static REGISTER_WORKER: Lazy<(RegisterWorkerSender, RegisterWorkerReceiver)> =
  Lazy::new(|| {
    let (tx, rx) = channel::<RegisterWorkerData>();
    (tx, Arc::new(Mutex::new(Some(rx))))
  });

#[napi]
pub fn register_worker(id: i32) {}
