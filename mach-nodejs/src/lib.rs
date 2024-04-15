use ipc_channel::ipc::IpcOneShotServer;
use napi_derive::napi;
use ipc_channel::ipc::IpcSender;
use ipc_channel::ipc::IpcReceiver;
use ipc_channel::ipc::channel;

#[napi]
pub fn start() {
  let ipcout_server_name = std::env::var("MACH_IPC_CHANNEL").unwrap();
  let (ipcin_init, ipcin_server_name) = IpcOneShotServer::<IpcReceiver<String>>::new().unwrap();

  let ipcout_init = IpcSender::<(IpcReceiver<String>, String)>::connect(ipcout_server_name).unwrap();
  let (ipcout, ipcout_rx) = channel::<String>().unwrap();
  
  ipcout_init.send((ipcout_rx, ipcin_server_name)).unwrap();
  ipcout.send("hi from child!".to_string()).unwrap();

  let (_, ipcin) = ipcin_init.accept().unwrap();

  while let Ok(bytes) = ipcin.recv() {
    println!("{:?}", bytes);
  } 
}
