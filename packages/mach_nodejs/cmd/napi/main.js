import { Worker } from 'node:worker_threads'
import * as path from 'node:path'
import * as url from 'node:url'
import napi from '../../platform/native/index.cjs'

const __dirname = url.fileURLToPath(new URL('.', import.meta.url))

napi.startWorker(() => {
  new Worker(path.join(__dirname, 'worker.js'))
})

napi.exec(process.argv.slice(2))
