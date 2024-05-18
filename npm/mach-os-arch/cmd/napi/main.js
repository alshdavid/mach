import { Worker } from 'node:worker_threads'
import * as path from 'node:path'
import * as url from 'node:url'
import napi from '../../platform/native/index.cjs'

const __dirname = url.fileURLToPath(new URL('.', import.meta.url))

napi.startWorker((/** @type {any} */ _, /** @type {number} */ id) => {
  console.log('hi')
  // new Worker(path.join(__dirname, 'worker.js'), {
  //   workerData: {
  //     id,
  //   },
  // })
})

napi.exec(process.argv.slice(2))
