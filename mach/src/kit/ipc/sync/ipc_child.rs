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

#[derive(Clone)]
pub struct IpcChild<TWrite, TRead>
where
  TWrite: Clone + Send + Serialize + DeserializeOwned + 'static,
  TRead: Clone + Send + Serialize + DeserializeOwned + 'static,
{
  outgoing: Sender<TWrite>,
  incoming: Sender<Sender<TRead>>,
}

impl<TWrite, TRead> IpcChild<TWrite, TRead>
where
  TWrite: Clone + Send + Serialize + DeserializeOwned + 'static,
  TRead: Clone + Send + Serialize + DeserializeOwned + 'static,
{
  pub fn new(host_server_name: &str) -> Self {
    // Proxies
    let (tx_child_outgoing, rx_child_outgoing) = channel::<TWrite>();
    let (tx_child_incoming, rx_child_incoming) = channel::<Sender<TRead>>();

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
      let mut senders = Vec::<Option<Sender<TRead>>>::new();

      while let Ok(data) = irx_child_incoming.recv() {
        for sender_opt in senders.iter_mut() {
          let Some(sender) = sender_opt else {
            continue;
          };
          if sender.send(data.clone()).is_err() {
            sender_opt.take();
          }
        }
        while let Ok(sender) = rx_child_incoming.try_recv() {
          if sender.send(data.clone()).is_ok() {
            senders.push(Some(sender));
          }
        }
      }
    });

    child_outgoing_init
      .send((irx_child_outgoing, itx_child_incoming))
      .unwrap();

    Self {
      incoming: tx_child_incoming,
      outgoing: tx_child_outgoing,
    }
  }

  pub fn send(
    &self,
    data: TWrite,
  ) {
    self.outgoing.send(data).unwrap();
  }

  pub fn subscribe(&self) -> Receiver<TRead> {
    let (tx, rx) = channel::<TRead>();
    self.incoming.send(tx).unwrap();
    rx
  }
}
