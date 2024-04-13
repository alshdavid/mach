use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::process::Command;
use std::process::Stdio;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;

// TODO https://crates.io/crates/arrow
// TODO https://github.com/mtth/avsc

#[derive(Clone)]
pub struct NodejsInstance {
  tx_stdin: Sender<Vec<u8>>,
  tx_stdout: Sender<Sender<Vec<u8>>>,
}

impl NodejsInstance {
  pub fn spawn() -> Self {
    let entry = std::env::current_exe()
      .unwrap()
      .parent()
      .unwrap()
      .parent()
      .unwrap()
      .join("nodejs")
      .join("src")
      .join("main.js");

    let mut command = Command::new("node");
    command.arg("--title");
    command.arg("nodejs_mach");
    command.arg(entry);

    command.stderr(Stdio::inherit());
    command.stdout(Stdio::piped());
    command.stdin(Stdio::piped());

    let mut child = command.spawn().unwrap();
    let mut stdin = child.stdin.take().unwrap();
    let (tx_stdin, rx_stdin) = channel::<Vec<u8>>();

    thread::spawn(move || {
      while let Ok(bytes) = rx_stdin.recv() {
        stdin.write(&bytes).unwrap();
      }
    });

    let stdout = child.stdout.take().unwrap();
    let (tx_stdout, rx_stdout) = channel::<Sender<Vec<u8>>>();

    thread::spawn(move || {
      let mut reader = BufReader::new(stdout);
      let mut buffer = Vec::<u8>::new();
      let mut senders = Vec::<Option<Sender<Vec<u8>>>>::new();
      let rx_stdout = rx_stdout;

      while let Ok(value) = reader.read_until(10, &mut buffer) {
        if value == 0 {
          break;
        }
        buffer.pop();
        let l = &buffer;

        for sender_opt in senders.iter_mut() {
          let Some(sender) = sender_opt else {
            continue;
          };
          if sender.send(l.clone()).is_err() {
            sender_opt.take();
          }
        }
        while let Ok(sender) = rx_stdout.try_recv() {
          if sender.send(l.clone()).is_ok() {
            senders.push(Some(sender));
          }
        }
        buffer.clear();
      }
    });

    Self {
      tx_stdin,
      tx_stdout,
    }
  }

  pub fn request(
    &self,
    bytes: Vec<u8>,
  ) -> Vec<u8> {
    self.tx_stdin.send(bytes).unwrap();
  }

  pub fn send(
    &self,
    bytes: Vec<u8>,
  ) {
    self.tx_stdin.send(bytes).unwrap();
  }

  pub fn subscribe(&self) -> Receiver<Vec<u8>> {
    let (tx, rx) = channel::<Vec<u8>>();
    self.tx_stdout.send(tx).unwrap();
    rx
  }
}
