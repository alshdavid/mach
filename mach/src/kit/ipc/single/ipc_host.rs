use ipc_channel::ipc::IpcOneShotServer;
use ipc_channel::ipc::IpcReceiver;
use ipc_channel::ipc::IpcSender;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Display;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;

pub struct IpcHost<TWrite, TRead>
where
  TWrite: Clone + Send + Serialize + DeserializeOwned + 'static,
  TRead: Clone + Send + Serialize + DeserializeOwned + 'static,
{
  server_name: String,
  outgoing: Sender<TWrite>,
  incoming: Receiver<TRead>,
}

impl<TWrite, TRead> IpcHost<TWrite, TRead>
where
  TWrite: Clone + Send + Serialize + DeserializeOwned + 'static,
  TRead: Clone + Send + Serialize + DeserializeOwned + 'static,
{
  pub fn new() -> Self {
    // Proxies
    let (tx_child_incoming, rx_child_incoming) = channel::<TRead>();
    let (tx_child_outgoing, rx_child_outgoing) = channel::<TWrite>();

    // Create a one shot channel that receives the "outgoing" and "incoming" channels
    let (child_incoming_init, child_incoming_server_name) =
      IpcOneShotServer::<(IpcReceiver<TRead>, IpcSender<TWrite>)>::new().unwrap();

    thread::spawn(move || {
      // Receive the "outgoing" and "incoming" channels
      let (_, (itx_child_incoming, itx_child_outgoing)) = child_incoming_init.accept().unwrap();

      // Proxy outgoing
      thread::spawn(move || {
        while let Ok(data) = rx_child_outgoing.recv() {
          itx_child_outgoing.send(data).unwrap();
        }
      });

      // Proxy incoming
      while let Ok(msg) = itx_child_incoming.recv() {
        tx_child_incoming.send(msg).unwrap();
      }
    });

    Self {
      server_name: child_incoming_server_name,
      outgoing: tx_child_outgoing,
      incoming: rx_child_incoming,
    }
  }

  pub fn send(
    &self,
    data: TWrite,
  ) {
    self.outgoing.send(data).unwrap();
  }
}
