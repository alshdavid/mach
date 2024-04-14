use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::process::Command;
use std::process::Stdio;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

// TODO https://crates.io/crates/arrow
// TODO https://github.com/mtth/avsc
// TODO Use TCP for windows and UNIX Domain (or named) sockets for MacOS and Linux

#[derive(Clone)]
pub struct NodejsInstanceStdio {
  tx_stdin: Sender<Vec<u8>>,
  messages: Arc<Mutex<[Option<Sender<Vec<u8>>>; 255]>>,
}

impl NodejsInstanceStdio {
  pub fn spawn() -> Self {
    let entry = std::env::current_exe()
      .unwrap()
      .parent()
      .unwrap()
      .parent()
      .unwrap()
      .join("nodejs")
      .join("src")
      .join("main_stdio.js");

    let mut command = Command::new("node");
    command.arg("--title");
    command.arg("nodejs_mach");
    command.arg(entry);

    command.stderr(Stdio::inherit());
    command.stdout(Stdio::piped());
    command.stdin(Stdio::piped());

    let messages: Arc<Mutex<[Option<Sender<Vec<u8>>>; 255]>> =
      Arc::new(Mutex::new(core::array::from_fn(|_| None)));
    let mut child = command.spawn().unwrap();
    let mut stdin = child.stdin.take().unwrap();
    let (tx_stdin, rx_stdin) = channel::<Vec<u8>>();

    thread::spawn(move || {
      while let Ok(bytes) = rx_stdin.recv() {
        stdin.write(&bytes).unwrap();
      }
    });

    let stdout = child.stdout.take().unwrap();
    let messages1 = messages.clone();
    thread::spawn(move || {
      let reader = BufReader::new(stdout);
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
          let mut messages = messages1.lock().unwrap();
          let Some(sender) = messages[id as usize].take() else {
            panic!("Sender not there");
          };
          sender.send(body).unwrap();
        }
      }
    });

    thread::spawn(move || {
      child.wait().unwrap();
    });

    Self { tx_stdin, messages }
  }

  pub fn request(
    &self,
    bytes: Vec<u8>,
  ) -> Vec<u8> {
    let (tx, rx) = channel::<Vec<u8>>();
    let id = 'block: {
      let mut messages = self.messages.lock().unwrap();
      for (id, msg) in messages.iter().enumerate() {
        if msg.is_none() {
          messages[id.clone()] = Some(tx);
          break 'block id.clone() as u8;
        }
      }
      todo!("Does not support more than 255 messages to Nodejs");
    };
    let mut msg = vec![id];
    msg.extend(bytes);
    self.tx_stdin.send(msg).unwrap();
    rx.recv().unwrap()
  }
}
