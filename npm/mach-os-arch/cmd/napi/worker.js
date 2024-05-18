console.log('thread started')
import { workerData } from 'node:worker_threads'
import napi from '../../platform/native/index.cjs'

napi.registerWorker(workerData.id)
