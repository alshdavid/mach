use std::convert::Infallible;
use std::net::SocketAddr;

use bytes::Bytes;
use http_body_util::Full;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use super::ServeCommand;

async fn hello(_: Request<impl hyper::body::Body>) -> Result<Response<Full<Bytes>>, Infallible> {
  Ok(Response::new(Full::new(Bytes::from("Hello World!"))))
}

async fn main_async(_command: ServeCommand) {
  let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();
  let listener = TcpListener::bind(addr).await.unwrap();
  println!("Listening on http://{}", addr);

  loop {
      let (tcp, _) = listener.accept().await.unwrap();
      let io = TokioIo::new(tcp);

      tokio::task::spawn(async move {
          if let Err(err) = http1::Builder::new()
              .serve_connection(io, service_fn(hello))
              .await
          {
              println!("Error serving connection: {:?}", err);
          }
      });
  }
}

pub fn main(command: ServeCommand) {
  tokio::runtime::Builder::new_multi_thread()
    .worker_threads(num_cpus::get_physical())
    .enable_all()
    .build()
    .unwrap()
    .block_on(main_async(command));
}
