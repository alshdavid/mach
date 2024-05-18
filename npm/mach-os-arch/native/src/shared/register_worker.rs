use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use mach_bundler_core::adapters::nodejs_napi::NapiOutgoingData;
use napi_derive::napi;
use once_cell::sync::Lazy;

pub type RegisterWorkerData = Sender<NapiOutgoingData>;
pub type RegisterWorkerSender = Sender<RegisterWorkerData>;
pub type RegisterWorkerReceiver = Arc<Mutex<Option<Receiver<RegisterWorkerData>>>>;

pub static REGISTER_WORKER: Lazy<(RegisterWorkerSender, RegisterWorkerReceiver)> =
  Lazy::new(|| {
    let (tx, rx) = channel::<RegisterWorkerData>();
    (tx, Arc::new(Mutex::new(Some(rx))))
  });

#[napi]
pub fn register_worker(_id: i32) {
  let (tx_outgoing, rx_outgoing) = channel::<NapiOutgoingData>();
  REGISTER_WORKER.0.send(tx_outgoing).unwrap();

  thread::spawn(move || {
    while let Ok(msg) = rx_outgoing.recv() {
      println!("{:?}", msg);
    }
  });
}
