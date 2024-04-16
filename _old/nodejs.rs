// use std::sync::mpsc::Receiver;
// use std::sync::Arc;
// use std::sync::Mutex;

// use serde::Serialize;

// use super::ipc::nodejs_worker::DataTo;
// use super::ipc::NodejsInstanceIpc;
// use super::ipc::NodejsWorkerFactory;
// use super::NodejsInstance;
// use super::ipc::nodejs_worker as ipc;

// pub struct NodejsOptions {
//   pub workers: usize,
// }

// #[derive(Clone, Serialize)]
// pub struct NodejsRequest<DataTo: ipc::DataTo> {
//   pub mode: u8,
//   pub id: usize,
//   pub data: DataTo,
// }

// pub struct NodejsResponse<DataFrom: ipc::DataFrom> {
//   pub mode: u8,
//   pub id: usize,
//   pub data: DataFrom,
// }

// #[derive(Clone)]
// pub struct Nodejs<DataTo: ipc::DataTo, DataFrom: ipc::DataFrom> {
//   counter: Arc<Mutex<u8>>,
//   workers: Vec<NodejsInstance<DataTo, DataFrom>>,
//   worker_count: usize,
// }

// impl<DataTo: ipc::DataTo, DataFrom: ipc::DataFrom> Nodejs<DataTo, DataFrom> {
//   pub fn new(options: NodejsOptions) -> Self {
//     let mut workers = Vec::<NodejsInstance<NodejsRequest<DataTo>, DataFrom>>::new();
//     let nodejs_worker_factory = NodejsInstanceIpc::new();
    
//     for _ in 0..options.workers {
//       let nodejs_instance = NodejsInstance::new(nodejs_worker_factory.spawn::<DataTo, DataFrom>());
//       workers.push(nodejs_instance);
//     }
    
//     Self {
//       counter: Arc::new(Mutex::new(0)),
//       workers: workers,
//       worker_count: options.workers,
//     }
//   }

//   pub fn request(
//     &self,
//     bytes: Vec<u8>,
//   ) -> Receiver<Vec<u8>> {
//     let next = {
//       let mut i = self.counter.lock().unwrap();
//       let next = i.clone();
//       *i += 1;
//       if *i as usize == self.worker_count {
//         *i = 0;
//       }
//       next
//     };
//     self.workers[next as usize].request(bytes)
//   }
// }