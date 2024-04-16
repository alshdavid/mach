// use std::sync::mpsc::channel;
// use std::sync::mpsc::Receiver;
// use std::sync::mpsc::Sender;
// use std::thread;

// use ipc_channel::ipc::IpcReceiver;
// use ipc_channel::ipc::IpcSender;

// use super::types as ipc;

// #[derive(Clone)]
// pub struct NodejsWorkerIpc<DataTo: ipc::DataTo, DataFrom: ipc::DataFrom> {
//   tx_write: Sender<DataTo>,
//   tx_read: Sender<Sender<DataFrom>>,
// }

// impl<DataTo: ipc::DataTo, DataFrom: ipc::DataFrom> NodejsWorkerIpc<DataTo, DataFrom> {
//   pub fn new(
//     ipcin: IpcSender<DataTo>,
//     ipcout: IpcReceiver<DataFrom>,
//   ) -> Self {
//     // Send bytes to worker
//     let (tx_write, rx_write) = channel::<DataTo>();
//     thread::spawn(move || {
//       while let Ok(bytes) = rx_write.recv() {
//         ipcin.send(bytes).unwrap();
//       }
//     });

//     // Get bytes from worker
//     let (tx_read, rx_read) = channel::<Sender<DataFrom>>();
//     thread::spawn(move || {
//       let mut senders = Vec::<Option<Sender<DataFrom>>>::new();

//       while let Ok(data) = ipcout.recv() {
//         for sender_opt in senders.iter_mut() {
//           let Some(sender) = sender_opt else {
//             continue;
//           };
//           if sender.send(data.clone()).is_err() {
//             sender_opt.take();
//           }
//         }
//         while let Ok(sender) = rx_read.try_recv() {
//           if sender.send(data.clone()).is_ok() {
//             senders.push(Some(sender));
//           }
//         }
//       }
//     });

//     Self {
//       tx_write,
//       tx_read,
//     }
//   }
// }

// impl<DataTo: ipc::DataTo, DataFrom: ipc::DataFrom> NodejsWorkerIpc<DataTo, DataFrom> {
//   fn send(
//     &self,
//     data: DataTo,
//   ) {
//     self.tx_write.send(data).unwrap();
//   }

//   fn subscribe(&self) -> Receiver<DataFrom> {
//     let (tx, rx) = channel::<DataFrom>();
//     self.tx_read.send(tx).unwrap();
//     rx
//   }
// }
