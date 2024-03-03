use std::convert::Infallible;
use std::net::SocketAddr;

use bytes::Bytes;
use http_body_util::{Full};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::{TokioIo, TokioTimer};
use tokio::net::TcpListener;

use super::ServeCommand;

async fn hello(_: Request<impl hyper::body::Body>) -> Result<Response<Full<Bytes>>, Infallible> {
  Ok(Response::new(Full::new(Bytes::from("Hello World!"))))
}

async fn main_async(command: ServeCommand) {
  // This address is localhost
  let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();

  // Bind to the port and listen for incoming TCP connections
  let listener = TcpListener::bind(addr).await.unwrap();
  println!("Listening on http://{}", addr);
  loop {
      // When an incoming TCP connection is received grab a TCP stream for
      // client<->server communication.
      //
      // Note, this is a .await point, this loop will loop forever but is not a busy loop. The
      // .await point allows the Tokio runtime to pull the task off of the thread until the task
      // has work to do. In this case, a connection arrives on the port we are listening on and
      // the task is woken up, at which point the task is then put back on a thread, and is
      // driven forward by the runtime, eventually yielding a TCP stream.
      let (tcp, _) = listener.accept().await.unwrap();
      // Use an adapter to access something implementing `tokio::io` traits as if they implement
      // `hyper::rt` IO traits.
      let io = TokioIo::new(tcp);

      // Spin up a new task in Tokio so we can continue to listen for new TCP connection on the
      // current task without waiting for the processing of the HTTP1 connection we just received
      // to finish
      tokio::task::spawn(async move {
          // Handle the connection from the client using HTTP1 and pass any
          // HTTP requests received on that connection to the `hello` function
          if let Err(err) = http1::Builder::new()
              .timer(TokioTimer)
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
