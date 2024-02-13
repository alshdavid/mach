use std::collections::HashMap;
use std::sync::Arc;

use serde::de::DeserializeOwned;
use serde::Serialize;

use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::Mutex;
use tokio::sync::oneshot;

#[derive(Debug)]
pub struct NodeWorker {
  pub tx_to_child: UnboundedSender<(String, String, String)>,
  pub pending_messages: Arc<Mutex<HashMap<String, oneshot::Sender<String>>>>,
}

impl NodeWorker {
  pub async fn send<T, U>(
    &self,
    action: &str,
    data: &T,
  ) -> Result<U, ()>
  where
    T: ?Sized + Serialize,
    U: DeserializeOwned,
  {
    let (tx, rx) = oneshot::channel::<String>();
    let msg_ref: String = snowflake::ProcessUniqueId::new().to_string();
    self.pending_messages.lock().await.insert(msg_ref.clone(), tx);

    let data = serde_json::to_string::<T>(data).unwrap();
    self
      .tx_to_child
      .send((msg_ref, action.to_string(), data))
      .unwrap();

    let Ok(result) = rx.await else {
      return Err(());
    };

    let result = serde_json::from_str::<U>(&result).unwrap();

    return Ok(result);
  }
}
