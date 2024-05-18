use std::io::Write;
use std::process::Child;
use std::process::Command;
use std::process::Stdio;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use ipc_channel_adapter::host::sync::ChildReceiver;
use ipc_channel_adapter::host::sync::ChildSender;
use oxc_resolver::ResolveOptions;

use super::NodejsWorker;
use crate::plugins::resolver_javascript::resolve_oxc;
use crate::public::AdapterIncomingRequest;
use crate::public::AdapterIncomingResponse;
use crate::public::AdapterOutgoingRequest;
use crate::public::AdapterOutgoingResponse;

#[derive(Clone)]
pub struct NodejsInstance {
  tx_stdin: Sender<Option<Vec<u8>>>,
  child: Arc<Mutex<Child>>,
}

/// NodejsInstance wraps the Node.js Process.
///
/// This wrapper uses stdin to instruct the child process to spawn
/// additional Nodejs worker threads.
///
/// Worker threads each individually have their own IPC channel pair
///
/// On the other end, the Nodejs workers import a napi module with the
/// IPC client channels, where types are sent into JavaScript using
/// the built-in napi-rs serialization
impl NodejsInstance {
  pub fn new() -> Result<Self, String> {
    let exe_path = std::env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();

    let entry = 'block: {
      let local_path = exe_dir
        .parent()
        .unwrap()
        .join("nodejs")
        .join("cmd")
        .join("ipc")
        .join("main.js");

      if local_path.exists() {
        break 'block local_path;
      }

      if let Ok(resolved) = resolve_oxc(
        exe_dir,
        "@alshdavid/mach/cmd/nodejs/cmd/ipc/main.js",
        ResolveOptions {
          symlinks: false,
          ..Default::default()
        },
      ) {
        break 'block resolved;
      };

      return Err("Nodejs entry not found".to_string());
    };

    let mut command = Command::new("node");
    command.arg("--title");
    command.arg("mach_bundler_nodejs_adapter_worker");
    command.arg(entry);

    command.stderr(Stdio::inherit());
    command.stdout(Stdio::inherit());
    command.stdin(Stdio::piped());

    let mut child = command.spawn().unwrap();

    let (tx_stdin, rx_stdin) = channel::<Option<Vec<u8>>>();

    let mut stdin = child.stdin.take().unwrap();

    thread::spawn(move || {
      while let Ok(bytes) = rx_stdin.recv() {
        if let Some(mut bytes) = bytes {
          bytes.push(10);
          stdin.write(&bytes).unwrap();
        } else {
          return;
        }
      }
    });

    Ok(Self {
      tx_stdin,
      child: Arc::new(Mutex::new(child)),
    })
  }

  pub fn spawn_worker(&self) -> NodejsWorker {
    let child_sender =
      ChildSender::<AdapterOutgoingRequest, AdapterOutgoingResponse>::new().unwrap();
    let (child_receiver, rx_child_receiver) =
      ChildReceiver::<AdapterIncomingRequest, AdapterIncomingResponse>::new().unwrap();

    let msg = format!(
      "{}&{}",
      child_sender.server_name, child_receiver.server_name
    );
    self.tx_stdin.send(Some(msg.as_bytes().to_vec())).unwrap();

    NodejsWorker {
      child_sender,
      child_receiver: rx_child_receiver,
    }
  }
}

impl Drop for NodejsInstance {
  fn drop(&mut self) {
    let mut child = self.child.lock().unwrap();
    if self.tx_stdin.send(None).is_err() || child.kill().is_err() || child.wait().is_err() {
      return;
    };
  }
}
