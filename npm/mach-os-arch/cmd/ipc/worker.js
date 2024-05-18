import { workerData } from 'node:worker_threads'
import { worker_callback } from '../../platform/worker/worker.js'
import { error_handler } from '../../platform/worker/error_handler.js'
import napi from '../../platform/native/index.cjs'

napi.worker(
  workerData.child_sender,
  workerData.child_receiver,
  error_handler(worker_callback),
)
