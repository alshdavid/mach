/*
  The "main" entrypoint is used to spawn workers
  
  The host process sends a message via stdin containing
  the IPC channel names for the worker to use
*/
import { Worker } from 'node:worker_threads'
import * as path from 'node:path'
import * as url from 'node:url'
import { buffer_bytes } from '../../platform/buffer_bytes/index.js'

const __dirname = url.fileURLToPath(new URL('.', import.meta.url))

process.stdin.on('data', buffer_bytes(str => {
  const [child_sender, child_receiver] = str.split('&')

  new Worker(path.join(__dirname, 'worker.js'), {
    workerData: {
      child_sender,
      child_receiver,
    },
  })
}))

process.stdin.on('close', () => {
  process.exit()
})
