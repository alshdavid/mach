// mod adapter;
// mod deno_cli;
// mod deno_embed;
// mod deno_snapshots;
// mod mach_extensions;

// use std::sync::mpsc::channel;
// use std::sync::mpsc::Sender;
// use std::sync::Arc;

// use adapter::DenoResponse;
// use adapter::DenoWorkerFarm;
// use adapter::DenoWorkerRequest;
// use deno_core::error::AnyError;
// use deno_core::v8::Function;
// use deno_core::v8::Local;
// use deno_core::v8::Value;
// use deno_core::JsRuntime;

// use adapter::DenoAction;
// use adapter::DenoAdapter;
// use deno_core::serde_v8;
// use deno_core::v8::Global;
// use deno_core::PollEventLoopOptions;
// use deno_embed::deno_current_thread;
// use libmach::Adapter;
// use libmach::AdapterBootstrapOptions;
// use libmach::AdapterBootstrapResult;

// use libmach::DependencyMapSync;
// use libmach::ResolveResult;
// use mach_extensions::LoadResolverState;
// use mach_extensions::RunResolverResolveState;
// use serde::Deserialize;
// use serde::Serialize;

// #[no_mangle]
// pub extern "C" fn bootstrap(config: AdapterBootstrapOptions) -> AdapterBootstrapResult {
//   let mut senders = vec![];

//   let tx = spawn_deno_worker(config.config.threads.clone(), config.dependency_map.clone());    
//   senders.push(tx);

//   let adapter: Box<dyn Adapter> = Box::new(DenoAdapter { 
//     worker_farm: Arc::new(DenoWorkerFarm::new(senders)),
//   });
//   return Box::new(Box::new(Ok(adapter)));
// }


// fn spawn_deno_worker(
//   threads: usize,
//   dependency_map: DependencyMapSync,
// ) -> Sender<DenoWorkerRequest> {
//   let (tx, rx) = channel::<DenoWorkerRequest>();

//   std::thread::spawn(move || {
//     deno_current_thread(async move {
//       let options = deno_embed::DenoInitOptions {
//         script: "/home/dalsh/Development/alshdavid/mach/adapters/deno/javascript/index.js"
//           .to_string(),
//         args: vec![format!("{}", threads)],
//         ..Default::default()
//       };
//       let mut worker = deno_embed::run_script(options, dependency_map.clone()).await.unwrap();

//       worker.run().await.unwrap();

//       let fn_load_resolver = {
//         let state: LoadResolverState = worker.worker.js_runtime.op_state().borrow_mut().take();
//         state.load_resolver_callback.unwrap()
//       };

//       let fn_run_resolver_resolve = {
//         let state: RunResolverResolveState =
//           worker.worker.js_runtime.op_state().borrow_mut().take();
//         state.run_resolver_resolve_callback.unwrap()
//       };

//       while let Ok((action, next)) = rx.recv() {
//         match action {
//           DenoAction::LoadResolver(file_path) => {
//             run_js_callback(
//               &mut worker.worker.js_runtime,
//               &fn_load_resolver,
//               file_path.to_str().unwrap(),
//             )
//             .await
//             .unwrap();

//             next.send(DenoResponse::LoadResolver(())).unwrap();
//           }
//           DenoAction::RunResolverResolve(specifier, dependency_id) => {
//             let value: Option<ResolveResult> = run_js_callback_with_return(
//               &mut worker.worker.js_runtime,
//               &fn_run_resolver_resolve,
//               &(specifier, dependency_id),
//             )
//             .await
//             .unwrap();

//             next.send(DenoResponse::RunResolverResolve(value)).unwrap();
//           }
//         }
//       }
//     });
//   });

//   return tx;
// }

// async fn run_js_callback<T: Serialize>(
//   js_runtime: &mut JsRuntime,
//   function: &Global<Function>,
//   input: T,
// ) -> Result<Global<Value>, AnyError> {
//   let request = {
//     let mut scope = &mut js_runtime.handle_scope();
//     let request = serde_v8::to_v8(scope, input).unwrap();
//     Global::new(&mut scope, request)
//   };

//   let call = js_runtime.call_with_args(function, &[request]);

//   js_runtime
//     .with_event_loop_promise(call, PollEventLoopOptions::default())
//     .await
// }

// async fn run_js_callback_with_return<'de, T: Serialize, R: Deserialize<'de>>(
//   js_runtime: &mut JsRuntime,
//   function: &Global<Function>,
//   input: T,
// ) -> Result<R, AnyError> {
//   let value = run_js_callback(
//     js_runtime,
//     function,
//     input,
//   )
//   .await
//   .unwrap();

//   let scope = &mut js_runtime.handle_scope();
//   let value = Local::new(scope, value);
//   Ok(serde_v8::from_v8::<R>(scope, value).unwrap())
// }
