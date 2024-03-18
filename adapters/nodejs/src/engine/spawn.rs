use std::process::Stdio;

use tokio::io::AsyncWriteExt;
use tokio::process::Child;
use tokio::process::Command;

use base64::engine::general_purpose;
use base64::Engine as _;

const SCRIPT_MAIN: &str = include_str!("./js/main.js");
const SCRIPT_WORKER: &str = include_str!("./js/worker.js");

pub fn get_js(
  port: &u16,
  worker_count: &usize,
) -> String {
  let script_worker = SCRIPT_WORKER.replace("__MACH__PORT__", port.to_string().as_str());
  let script_worker_b64 = general_purpose::STANDARD.encode(&script_worker);
  let script = SCRIPT_MAIN
    .replace("__MACH_WORKER_SCRIPT_B64__", &script_worker_b64)
    .replace("__MACH_WORKER_SCRIPT__", &script_worker)
    .replace("__MACH_WORKER_COUNT__", &(worker_count - 1).to_string());
  return script;
}

pub async fn spawn_node_js(
  port: &u16,
  worker_count: &usize,
) -> Child {
  let mut command = Command::new("node");
  command.arg("--title");
  command.arg("child_process_node");

  command.stderr(Stdio::inherit());
  command.stdout(Stdio::inherit());
  command.stdin(Stdio::piped());

  let mut child = command.spawn().unwrap();

  // Execute the glue code within Node.js
  let mut stdin = child.stdin.take().unwrap();
  let script = get_js(port, worker_count);
  stdin.write(script.as_bytes()).await.unwrap();
  stdin.flush().await.unwrap();
  drop(stdin);

  return child;
}
