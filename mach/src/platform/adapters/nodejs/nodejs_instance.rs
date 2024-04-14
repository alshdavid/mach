use std::collections::HashMap;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use super::ipc::NodejsWorker;

#[derive(Clone)]
pub struct NodejsInstance {
  _nodejs_worker: Arc<dyn NodejsWorker>,
  tx_write: Sender<Vec<u8>>,
  messages: Arc<Mutex<HashMap<u32, Sender<Vec<u8>>>>>,
  message_count: Arc<AtomicU32>
}

impl NodejsInstance {
  pub fn new(nodejs_worker: Arc<dyn NodejsWorker>) -> Self {
    let messages = Arc::new(Mutex::new(HashMap::<u32, Sender<Vec<u8>>>::new()));

    let (tx_write, rx_write) = channel::<Vec<u8>>();
    let worker_write = nodejs_worker.clone();
    thread::spawn(move || {
      while let Ok(bytes) = rx_write.recv() {
        worker_write.send(bytes);
      }
    });

    let rx_read = nodejs_worker.subscribe();
    let messages_read = messages.clone();
    thread::spawn(move || {
      let mut stage = 0;
      let mut id: [u8; 4] = [0; 4];
      let mut buf = Vec::<u8>::new();

      while let Ok(byte) = rx_read.recv() {
        if stage == 0 {
          id[0] = byte;
          stage += 1;
          continue;
        }

        if stage == 1 {
          id[1] = byte;
          stage += 1;
          continue;
        }

        if stage == 2 {
          id[2] = byte;
          stage += 1;
          continue;
        }

        if stage == 3 {
          id[3] = byte;
          stage += 1;
          continue;
        }

        if byte != 10 {
          buf.push(byte);
          continue;
        }

        
        stage = 0;
        let id = u32::from_ne_bytes(std::mem::take(&mut id));
        let body = std::mem::take(&mut buf);
        
        let mut messages = messages_read.lock().unwrap();
        let Some(sender) = messages.remove(&id) else {
          panic!("Sender not there {}", id);
        };
        sender.send(body).unwrap();
      }
    });

    Self {
      _nodejs_worker: nodejs_worker,
      tx_write,
      messages,
      message_count: Arc::new(AtomicU32::new(0)),
    }
  }

  pub fn request(
    &self,
    bytes: Vec<u8>,
  ) -> Receiver<Vec<u8>> {
    let (tx, rx) = channel::<Vec<u8>>();
    let next = self.message_count.fetch_add(1, Ordering::Relaxed);
    {
      let mut messages = self.messages.lock().unwrap();
      messages.insert(next.clone(), tx);
    };
    
    let mut id = next.to_ne_bytes().to_vec();
    id.extend(bytes);
    id.push(10);

    self.tx_write.send(id).unwrap();
    rx
  }
}