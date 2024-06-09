use std::sync::mpsc::Sender;

pub type RpcResult<T> = Result<T, String>;

#[derive(Debug)]
pub enum RpcMessage {
  Ping { response: Sender<RpcResult<()>> },
  Init { response: Sender<RpcResult<()>> },
}
