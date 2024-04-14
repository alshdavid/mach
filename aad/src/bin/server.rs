use ipc_channel::ipc::bytes_channel;
use ipc_channel::ipc::IpcOneShotServer;
use ipc_channel::ipc::IpcReceiver;
use std::sync::{mpsc, Mutex};

fn main() {
    let (server, server_name) = IpcOneShotServer::<IpcReceiver<String>>::new().unwrap();
    println!("Hello, world! {}", server_name);
    let (_, rx) = server.accept().unwrap();

    while let Ok(bytes) = rx.recv() {
        println!("{:?}", bytes);
    }
}
