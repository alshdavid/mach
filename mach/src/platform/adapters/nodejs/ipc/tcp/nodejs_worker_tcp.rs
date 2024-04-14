use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread;

use super::super::NodejsWorker;

#[derive(Clone)]
pub struct NodejsWorkerTcp {
  tx_write: Sender<Vec<u8>>,
  tx_read: Sender<Sender<u8>>,
  stream: Arc<TcpStream>,
}

impl NodejsWorkerTcp {
  pub fn new(stream: TcpStream) -> Self {
    let stream = Arc::new(stream);

    // Send bytes to worker
    let (tx_write, rx_write) = channel::<Vec<u8>>();
    let stream_writer = stream.clone();
    thread::spawn(move || {
      while let Ok(bytes) = rx_write.recv() {
        if stream_writer.as_ref().write(&bytes).is_err() {
          break;
        }
      }
    });

    // Get bytes from worker
    let (tx_read, rx_read) = channel::<Sender<u8>>();
    let stream_reader = stream.clone();
    thread::spawn(move || {
      let mut senders = Vec::<Option<Sender<u8>>>::new();
      let reader = BufReader::new(stream_reader.as_ref());

      for byte in reader.bytes() {
        let Ok(byte) = byte else {
          break;
        };
        for sender_opt in senders.iter_mut() {
          let Some(sender) = sender_opt else {
            continue;
          };
          if sender.send(byte.clone()).is_err() {
            sender_opt.take();
          }
        }
        while let Ok(sender) = rx_read.try_recv() {
          if sender.send(byte.clone()).is_ok() {
            senders.push(Some(sender));
          }
        }
      }
    });

    Self {
      tx_write,
      tx_read,
      stream,
    }
  }
}

impl NodejsWorker for NodejsWorkerTcp {
  fn send(
    &self,
    bytes: Vec<u8>,
  ) {
    self.tx_write.send(bytes).unwrap();
  }

  fn subscribe(&self) -> Receiver<u8> {
    let (tx, rx) = channel::<u8>();
    self.tx_read.send(tx).unwrap();
    rx
  }
}
