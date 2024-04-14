use std::{io::{BufReader, Read, Write}, net::TcpStream, sync::{mpsc::{channel, Sender}, Arc, Mutex}, thread};

#[derive(Clone)]
pub struct NodejsWorker {
  tx_write: Sender<Vec<u8>>,
  messages: Arc<Mutex<[Option<Sender<Vec<u8>>>; 255]>>,
  stream: Arc<TcpStream>
}

impl NodejsWorker {
  pub fn new(stream: TcpStream) -> Self {
    let stream = Arc::new(stream);

    // Send bytes to worker
    let (tx_write, rx_write) = channel::<Vec<u8>>();
    let stream_write = stream.clone();
    thread::spawn(move || {
      while let Ok(bytes) = rx_write.recv() {
        if stream_write.as_ref().write(&bytes).is_err() {
          break;
        }
      }
    });
    
    // Get bytes from worker
    let messages: Arc<Mutex<[Option<Sender<Vec<u8>>>; 255]>> = Arc::new(Mutex::new(core::array::from_fn(|_| None)));
    let stream_read = stream.clone();
    let stream_messages = messages.clone();
    thread::spawn(move || {
      let reader = BufReader::new(stream_read.as_ref());
      let mut buf_id = None::<u8>;
      let mut buf_body = Vec::<u8>::new();

      for byte in reader.bytes() {
        let Ok(byte) = byte else {
          break;
        };
        if buf_id.is_none() {
          buf_id.replace(byte);
        } else if byte != 10 {
          buf_body.push(byte);
        } else {
          let id = buf_id.take().unwrap();
          let body = std::mem::take(&mut buf_body);
          let mut messages = stream_messages.lock().unwrap();
          let Some(sender) = messages[id as usize].take() else {
            panic!("Sender not there");
          };
          sender.send(body).unwrap();
        }
      }
    });
    
    Self {
      tx_write,
      stream, 
      messages,
    }
  }
}
