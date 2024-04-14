use ipc_channel::ipc::channel;
use ipc_channel::ipc::IpcReceiver;
use ipc_channel::ipc::IpcSender;
use std::sync::{mpsc, Mutex};

fn main() {
    let ttx = IpcSender::<IpcReceiver<String>>::connect("f0359592-cb66-4919-be82-d48d084b40b7".to_string()).unwrap();
    let (tx, rx) = channel::<String>().unwrap();
    ttx.send(rx).unwrap();
    tx.send("hello-world".to_string()).unwrap();
}
