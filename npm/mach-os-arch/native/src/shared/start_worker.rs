use std::{cell::RefCell, sync::{mpsc::{channel, Receiver, Sender}, Arc, Mutex}, thread};

use napi::{threadsafe_function::{ThreadSafeCallContext, ThreadsafeFunctionCallMode}, Env, JsFunction, JsUndefined};
use napi_derive::napi;
use once_cell::sync::Lazy;

// Take the Sender so the channel can be closed when the application 
// finishes, that will prevent the application hanging.
pub type StartWorkerSender = Arc<Mutex<Option<Sender<usize>>>>;
pub type StartWorkerReceiver = Arc<Mutex<Option<Receiver<usize>>>>;

pub static START_WORKER: Lazy<(StartWorkerSender, StartWorkerReceiver)> = Lazy::new(|| {
  let (tx, rx) = channel::<usize>();
  (Arc::new(Mutex::new(Some(tx))), Arc::new(Mutex::new(Some(rx))))
});

#[napi]
pub fn start_worker(env: Env, callback: JsFunction) -> napi::Result<JsUndefined> {
  let rx_napi = START_WORKER.1.lock().unwrap().take().unwrap();
  
  let tsfn = env.create_threadsafe_function(&callback, 0, |ctx: ThreadSafeCallContext<usize>| {
    let value = ctx.env.to_js_value(&ctx.value);
    Ok(vec![value])
  })?;

  thread::spawn(move || {
    println!("listening");

    while let Ok(id) = rx_napi.recv() {
      println!("recv");
      tsfn.call(Ok(id), ThreadsafeFunctionCallMode::Blocking);
    }
    println!("end");

  });

  env.get_undefined()
}