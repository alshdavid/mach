use deno_core::futures::FutureExt;
use deno_core::unsync::MaskFutureAsSend;

#[inline(always)]
pub fn deno_current_thread<F, R>(future: F) -> R
where
  F: std::future::Future<Output = R> + 'static,
  R: Send + 'static,
{
  let tokio_runtime = tokio::runtime::Builder::new_current_thread()
    .enable_io()
    .enable_time()
    .event_interval(61)
    .global_queue_interval(31)
    .max_io_events_per_tick(1024)
    .max_blocking_threads(32)
    .build()
    .unwrap();

  let future = async move {
    deno_core::unsync::spawn(async move { future.await }.boxed_local())
      .await
      .unwrap()
  };

  #[cfg(debug_assertions)]
  let future = Box::pin(unsafe { MaskFutureAsSend::new(future) });

  #[cfg(not(debug_assertions))]
  let future = unsafe { MaskFutureAsSend::new(future) };

  let join_handle = tokio_runtime.spawn(future);

  tokio_runtime.block_on(join_handle).unwrap().into_inner()
}
