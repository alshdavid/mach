use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use mach::kit::ipc::sync::IpcChild;
use mach::public::nodejs::NodejsHostRequest;
use mach::public::nodejs::NodejsHostResponse;
use mach::public::nodejs::NodejsHostRequestContext;
use mach::public::nodejs::NodejsHostResponseContext;

pub struct HostSender {
  counter: Arc<AtomicUsize>,
  messages: Arc<Mutex<HashMap<usize, Sender<NodejsHostResponse>>>>,
  ipc_child_host: IpcChild<NodejsHostRequestContext, NodejsHostResponseContext>
}

impl HostSender {
  pub fn new() -> Self {
    let ipc_child_host = std::env::var("MACH_IPC_CHANNEL_2").unwrap().to_string();
    let ipc_child_host = IpcChild::<NodejsHostRequestContext, NodejsHostResponseContext>::new(&ipc_child_host);

    let messages = Arc::new(Mutex::new(
      HashMap::<usize, Sender<NodejsHostResponse>>::new(),
    ));

    {
      let rx = ipc_child_host.subscribe();
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
      messages,
      counter: Arc::new(AtomicUsize::new(0)),
      ipc_child_host,
    }
  }

  pub fn send_blocking(&self, req: NodejsHostRequest) -> NodejsHostResponse {
    self.send(req).recv().unwrap()
  }

  pub fn send(&self, req: NodejsHostRequest) -> Receiver<NodejsHostResponse> {
    let count = self.counter.fetch_add(1, Ordering::Relaxed);
    let (tx, rx) = channel::<NodejsHostResponse>();
    self.messages.lock().unwrap().insert(count.clone(), tx);
    self.ipc_child_host.send(NodejsHostRequestContext(
      count.clone(),
      req,
    ));
    rx
  }
}
