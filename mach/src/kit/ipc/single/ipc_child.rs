use ipc_channel::ipc::channel as ipc_channel;
use ipc_channel::ipc::IpcReceiver;
use ipc_channel::ipc::IpcSender;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Display;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;

pub struct IpcChild<TWrite, TRead>
where
  TWrite: Clone + Send + Sync + Serialize + DeserializeOwned + Display + 'static,
  TRead: Clone + Send + Sync + Serialize + DeserializeOwned + Display + 'static,
{
  incoming: Receiver<TRead>,
  outgoing: Sender<TWrite>,
}

impl<TWrite, TRead> IpcChild<TWrite, TRead>
where
  TWrite: Clone + Send + Sync + Serialize + DeserializeOwned + Display + 'static,
  TRead: Clone + Send + Sync + Serialize + DeserializeOwned + Display + 'static,
{
  pub fn new(host_server_name: &str) -> Self {
    // Proxies
    let (tx_child_outgoing, rx_child_outgoing) = channel::<TWrite>();
    let (tx_child_incoming, rx_child_incoming) = channel::<TRead>();

    let (itx_child_outgoing, irx_child_outgoing) = ipc_channel::<TWrite>().unwrap();
    let (itx_child_incoming, irx_child_incoming) = ipc_channel::<TRead>().unwrap();

    // Receive a one shot channel to send back the "outgoing" and "incoming" channels
    let child_outgoing_init =
      IpcSender::<(IpcReceiver<TWrite>, IpcSender<TRead>)>::connect(host_server_name.to_string())
        .unwrap();

    // Proxy outgoing
    thread::spawn(move || {
      while let Ok(data) = rx_child_outgoing.recv() {
        itx_child_outgoing.send(data).unwrap();
      }
    });

    // Proxy incoming
    thread::spawn(move || {
      while let Ok(data) = irx_child_incoming.recv() {
        tx_child_incoming.send(data).unwrap();
      }
    });

    child_outgoing_init
      .send((irx_child_outgoing, itx_child_incoming))
      .unwrap();

    Self {
      incoming: rx_child_incoming,
      outgoing: tx_child_outgoing,
    }
  }

  pub fn send(
    &self,
    data: TWrite,
  ) {
    self.outgoing.send(data).unwrap();
  }
}
