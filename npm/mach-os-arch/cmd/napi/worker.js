import { worker_callback } from '../../platform/worker/worker.js'
import { error_handler } from '../../platform/worker/error_handler.js'
import napi from '../../platform/native/index.cjs'

napi.registerWorker(error_handler(worker_callback))
