use std::collections::HashMap;
use std::process::Command;
use std::process::Stdio;
use std::rc::Rc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use ipc_channel::ipc::channel as ipc_channel;
use ipc_channel::ipc::IpcReceiver;

use crate::kit::ipc::sync::IpcHost;
use crate::public::nodejs::NodejsClientRequest;
use crate::public::nodejs::NodejsClientResponse;
use crate::public::nodejs::NodejsRequestContext;
use crate::public::nodejs::NodejsResponseContext;

#[derive(Clone)]
pub struct NodejsWorker {
  counter: Arc<AtomicUsize>,
  messages: Arc<Mutex<HashMap<usize, Sender<NodejsClientResponse>>>>,
  ipc_host: IpcHost<NodejsRequestContext, NodejsResponseContext>,
}

impl NodejsWorker {
  pub fn new() -> Self {
    let ipc_host = IpcHost::<NodejsRequestContext, NodejsResponseContext>::new();

    let entry = std::env::current_exe()
      .unwrap()
      .parent()
      .unwrap()
      .parent()
      .unwrap()
      .join("nodejs")
      .join("lib")
      .join("main.js");

    let mut command = Command::new("node");
    command.arg("--title");
    command.arg("nodejs_mach");
    command.arg(entry);
    command.env("MACH_IPC_CHANNEL", &ipc_host.server_name);

    command.stderr(Stdio::inherit());
    command.stdout(Stdio::inherit());
    command.stdin(Stdio::piped());

    command.spawn().unwrap();

    let messages = Arc::new(Mutex::new(
      HashMap::<usize, Sender<NodejsClientResponse>>::new(),
    ));

    let m1 = messages.clone();
    let rx = ipc_host.subscribe();
    thread::spawn(move || {
      while let Ok(data) = rx.recv() {
        let Some(sender) = m1.lock().unwrap().remove(&data.0) else {
          panic!();
        };
        sender.send(data.1).unwrap();
      }
    });

    Self {
      ipc_host,
      counter: Arc::new(AtomicUsize::new(0)),
      messages,
    }
  }

  pub fn send_ping(&self) -> Receiver<NodejsClientResponse> {
    let count = self.counter.fetch_add(1, Ordering::Relaxed);
    let (tx, rx) = channel::<NodejsClientResponse>();
    self.messages.lock().unwrap().insert(count.clone(), tx);
    self.ipc_host.send(NodejsRequestContext(
      count.clone(),
      NodejsClientRequest::Ping,
    ));
    rx
  }
}
