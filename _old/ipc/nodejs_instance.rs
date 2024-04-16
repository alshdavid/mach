// use std::process::Command;
// use std::process::Stdio;
// use std::sync::Arc;

// use ipc_channel::ipc::IpcOneShotServer;
// use ipc_channel::ipc::IpcReceiver;
// use ipc_channel::ipc::channel as ipc_channel;
// use ipc_channel::ipc::IpcSender;

// use super::nodejs_worker::NodejsWorkerIpc;
// use super::types as ipc;

// #[derive(Clone)]
// pub struct NodejsInstanceIpc {}

// impl NodejsInstanceIpc {
//   pub fn new() -> Self {
//     return Self {};
//   }

//   pub fn spawn<DataTo: ipc::DataTo, DataFrom: ipc::DataFrom> (&self) -> Arc<NodejsWorkerIpc<DataTo, DataFrom>> {
//     let (ipcout_init, server_name) = IpcOneShotServer::<(IpcReceiver<DataFrom>, String)>::new().unwrap();

//     let entry = std::env::current_exe()
//       .unwrap()
//       .parent()
//       .unwrap()
//       .parent()
//       .unwrap()
//       .join("nodejs")
//       .join("lib")
//       .join("main.js");

//     let mut command = Command::new("node");
//     command.arg("--title");
//     command.arg("nodejs_mach");
//     command.arg(entry);
//     command.env("MACH_IPC_CHANNEL", server_name);

//     command.stderr(Stdio::inherit());
//     command.stdout(Stdio::inherit());
//     command.stdin(Stdio::piped());

//     command.spawn().unwrap();
//     let (_, (ipcout, ipcin_server_name)) = ipcout_init.accept().unwrap();
//     let (ipcin, ipcin_rx) = ipc_channel::<DataTo>().unwrap();
//     let ipcin_init = IpcSender::<IpcReceiver<DataTo>>::connect(ipcin_server_name).unwrap();
//     ipcin_init.send(ipcin_rx).unwrap();

//     Arc::new(NodejsWorkerIpc::new(
//       ipcin,
//       ipcout,
//     ))
//   }
// }
