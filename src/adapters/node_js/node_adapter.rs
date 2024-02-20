use std::collections::HashMap;
use std::sync::Arc;

use serde::de::DeserializeOwned;
use serde::Serialize;

use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::net::TcpListener;
use tokio::sync::mpsc::unbounded_channel;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::oneshot;
use tokio::sync::Mutex;

use super::spawn::spawn_node_js;
use super::NodeWorker;

#[derive(Debug)]
pub struct NodeAdapter {
  send_to: Arc<Mutex<usize>>,
  tx_shutdown: UnboundedSender<()>,
  workers: Vec<NodeWorker>,
}

impl NodeAdapter {
  pub async fn new(worker_count: usize) -> NodeAdapter {
    // Don't initialize Nodejs if no workers are needed
    if worker_count == 0 {
      let (tx, _) = unbounded_channel::<()>();
      return NodeAdapter {
        send_to: Arc::new(Mutex::new(0)),
        tx_shutdown: tx,
        workers: vec![],
      };
    }
    // Create socket for Node.js to connect to
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    // Create Node.js child process and pipe JS into
    let mut child = spawn_node_js(&port, &worker_count).await;

    // Spawn a thread to communicate with each Node.js worker
    let mut workers = Vec::<NodeWorker>::new();

    for _ in 0..worker_count {
      // Wait for the Node.js worker thread to connect to the socket
      let Ok((stream, _)) = listener.accept().await else {
        panic!("Unable to connect");
      };

      let (stream_read, mut stream_write) = tokio::io::split(stream);

      let (tx_to_child, mut rx_to_child) = unbounded_channel::<(String, String, String)>();

      // Messages going to Node.js worker
      // Thread to manage sending messages to the Node.js worker
      tokio::task::spawn(async move {
        while let Some((msg_ref, action, data)) = rx_to_child.recv().await {
          let msg = format!("{}\n{}\n{}\n", msg_ref, action, data);
          if stream_write.write(msg.as_bytes()).await.is_err() {
            break;
          };
          stream_write.flush().await.unwrap();
        }
      });

      // Messages coming back from Node.js worker
      // This holds messages that are in-flight
      let pending_messages =
        Arc::new(Mutex::new(HashMap::<String, oneshot::Sender<String>>::new()));
      let pending_messages_thread = pending_messages.clone();

      // Thread to manage messages coming back from Node.js worker
      tokio::task::spawn(async move {
        let mut reader = BufReader::new(stream_read);
        let mut line = String::new();
        let mut incoming_msg_ref = String::new();

        // Read incoming message until /n character
        while let Ok(value) = reader.read_line(&mut line).await {
          if value == 0 {
            break;
          }
          line.pop();
          let value = std::mem::take(&mut line);

          if incoming_msg_ref == "" {
            incoming_msg_ref = value;
            continue;
          }

          let incoming_msg_ref = std::mem::take(&mut incoming_msg_ref);

          let Some(listener) = pending_messages_thread
            .lock()
            .await
            .remove(&incoming_msg_ref)
          else {
            todo!();
          };

          if value == "undefined" {
            listener.send("{}".to_string()).unwrap();
          } else {
            listener.send(value).unwrap();
          }
        }
      });

      workers.push(NodeWorker {
        tx_to_child,
        pending_messages,
      })
    }

    let (tx_shutdown, mut rx_shutdown) = unbounded_channel::<()>();

    // Thread to listen for the shutdown event
    tokio::task::spawn(async move {
      if rx_shutdown.recv().await.is_none() {
        return;
      }
      drop(listener);
      child.kill().await.unwrap();
      child.wait().await.unwrap();
    });

    return NodeAdapter {
      send_to: Arc::new(Mutex::new(0)),
      tx_shutdown,
      workers,
    };
  }

  pub async fn send<T, U>(
    &self,
    action: &str,
    data: &T,
  ) -> Result<U, ()>
  where
    T: ?Sized + Serialize,
    U: DeserializeOwned,
  {
    // Pick the worker to send the message to using round robin
    let send_to = {
      let mut send_to = self.send_to.lock().await;

      if *send_to >= self.workers.len() {
        *send_to = 0;
      }

      let send_index = (*send_to).clone();
      *send_to += 1;
      send_index
    };

    self.workers[send_to].send(action, data).await
  }

  pub async fn send_all<T>(
    &self,
    action: &str,
    data: &T,
  ) -> Result<(), ()>
  where
    T: ?Sized + Serialize,
  {
    for worker in &self.workers {
      if worker
        .send::<T, serde_json::Value>(action, data)
        .await
        .is_err()
      {
        return Err(());
      }
    }
    return Ok(());
  }

  pub fn shutdown(&self) -> Result<(), ()> {
    if self.tx_shutdown.send(()).is_err() {
      return Err(());
    }
    return Ok(());
  }
}

impl Drop for NodeAdapter {
  fn drop(&mut self) {
    self.shutdown().ok();
  }
}
