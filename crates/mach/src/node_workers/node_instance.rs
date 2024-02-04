use rust_embed::RustEmbed;
use std::env;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::process::Child;
use std::process::Command;
use std::process::Stdio;
use kanal::unbounded;
use kanal::Receiver;
use kanal::Sender;
use std::thread;

#[derive(RustEmbed)]
#[folder = "src/node_workers/js/"]
#[include = "*.mjs"]
#[include = "*.js"]
#[exclude = "node_modules/*"]
struct JsFiles;

pub struct NodeInstance {
  _child: Child,
  pub sender: Sender<String>,
  pub receiver: Option<Receiver<String>>,
}

impl NodeInstance {
  pub fn new() -> Self {
    let exe_path = env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let node_worker_scripts = exe_dir.join("js");

    if !node_worker_scripts.exists() {
      fs::create_dir(&node_worker_scripts).unwrap();

      for file_name in JsFiles::iter() {
        let file = JsFiles::get(file_name.as_ref()).unwrap();
        fs::write(
          node_worker_scripts.join(file_name.as_ref()),
          file.data.as_ref(),
        )
        .unwrap();
      }
    }

    let (to_child_tx, to_child_rx) = unbounded::<String>();
    let (from_child_tx, from_child_rx) = unbounded::<String>();

    let mut command = Command::new("node");
    command.arg(node_worker_scripts.join("index.mjs").to_str().unwrap());
    command.current_dir(env::current_dir().unwrap());

    command.stderr(Stdio::piped());
    command.stdout(Stdio::piped());
    command.stdin(Stdio::piped());

    let mut child = command.spawn().unwrap();
    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();

    thread::spawn(move || {
      while let Ok(msg) = to_child_rx.recv() {
        if stdin.write(format!("{}\n", msg).as_bytes()).is_err() {
          break;
        };
      }
    });

    thread::spawn(move || {
      let mut reader = BufReader::new(stdout);
      let mut line = String::new();

      while let Ok(value) = reader.read_line(&mut line) {
        if value == 0 {
          break;
        }
        line.pop();
        let value = std::mem::take(&mut line);
        from_child_tx.send(value.clone()).unwrap();
      }
    });

    NodeInstance {
      _child: child,
      sender: to_child_tx,
      receiver: Some(from_child_rx),
    }
  }
}
