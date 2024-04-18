use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use crate::kit::ipc::sync::IpcHost;
use crate::public::nodejs::NodejsClientRequest;
use crate::public::nodejs::NodejsClientRequestContext;
use crate::public::nodejs::NodejsClientResponse;
use crate::public::nodejs::NodejsClientResponseContext;

#[derive(Clone)]
pub struct ChildSender {
  pub server_name: String,
  counter: Arc<AtomicUsize>,
  messages: Arc<Mutex<HashMap<usize, Sender<NodejsClientResponse>>>>,
  ipc_host_client: IpcHost<NodejsClientRequestContext, NodejsClientResponseContext>
}

impl ChildSender {
  pub fn new() -> Self {
    let ipc_host_client = IpcHost::<NodejsClientRequestContext, NodejsClientResponseContext>::new();
    let server_name = ipc_host_client.server_name.clone();

    let messages = Arc::new(Mutex::new(
      HashMap::<usize, Sender<NodejsClientResponse>>::new(),
    ));

    {
      let rx = ipc_host_client.subscribe();
      let messages = messages.clone();
      
      thread::spawn(move || {
        while let Ok(data) = rx.recv() {
          let Some(sender) = messages.lock().unwrap().remove(&data.0) else {
            panic!();
          };
          sender.send(data.1).unwrap();
        }
      });
    }

    Self {
      server_name,
      messages,
      counter: Arc::new(AtomicUsize::new(0)),
      ipc_host_client,
    }
  }

  pub fn send_blocking(&self, req: NodejsClientRequest) -> NodejsClientResponse {
    self.send(req).recv().unwrap()
  }

  pub fn send(&self, req: NodejsClientRequest) -> Receiver<NodejsClientResponse> {
    let count = self.counter.fetch_add(1, Ordering::Relaxed);
    let (tx, rx) = channel::<NodejsClientResponse>();
    self.messages.lock().unwrap().insert(count.clone(), tx);
    self.ipc_host_client.send(NodejsClientRequestContext(
      count.clone(),
      req,
    ));
    rx
  }
}
