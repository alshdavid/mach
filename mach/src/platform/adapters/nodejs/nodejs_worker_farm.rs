// use std::sync::mpsc::channel;
// use std::sync::mpsc::Receiver;
// use std::sync::Arc;
// use std::sync::Mutex;
// use std::thread;

// use crate::platform::adapters::nodejs::NodejsInstance;

// #[derive(Clone)]
// pub struct NodejsWorkerFarm {
//   // TODO replace with AtomicU8 can use Relaxed for both orderings
//   counter: Arc<Mutex<u8>>,
//   children: Arc<Vec<NodejsInstance>>,
// }

// impl NodejsWorkerFarm {
//   pub fn new(instances: usize) -> Self {
//     let mut children = vec![];

//     for _ in 0..instances {
//       let nodejs_instance = NodejsInstance::spawn();
//       children.push(nodejs_instance);
//     }

//     Self {
//       children: Arc::new(children),
//       counter: Arc::new(Mutex::new(0)),
//     }
//   }

//   pub fn send(
//     &self,
//     bytes: Vec<u8>,
//   ) {
//     let mut i = self.counter.lock().unwrap();
//     self.children.get(*i as usize).unwrap().send(bytes);
//     *i += 1;
//     if *i as usize == self.children.len() {
//       *i = 0;
//     }
//   }

//   pub fn subscribe(&self) -> Receiver<Vec<u8>> {
//     let (tx_combined, rx_combined) = channel::<Vec<u8>>();

//     for nodejs_instance in self.children.iter() {
//       let nodejs_instance = nodejs_instance.clone();
//       let tx_combined = tx_combined.clone();

//       thread::spawn(move || {
//         let rx = nodejs_instance.subscribe();
//         while let Ok(bytes) = rx.recv() {
//           tx_combined.send(bytes).unwrap();
//         }
//       });
//     }

//     return rx_combined;
//   }
// }
