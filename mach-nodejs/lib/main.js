/*
  The "main" entrypoint is used to spawn workers
  
  The host process sends a message via stdin containing
  the IPC channel names for the worker to use
*/
import { Worker } from 'node:worker_threads'
import * as path from 'node:path'
import * as url from 'node:url'

const __dirname = url.fileURLToPath(new URL('.', import.meta.url))

/** @type {Array<any>} */
let buf_body = []

// Buffer until newline
process.stdin.on('data', (bytes) => {
  for (const byte of bytes) {
    // Characters
    if (byte !== 10) {
      buf_body.push(byte)
    }

    // Newline
    if (byte === 10) {
      const str = new TextDecoder().decode(new Uint8Array(buf_body))
      const [child_sender, child_receiver] = str.split('&')

      new Worker(path.join(__dirname, 'worker.js'), {
        workerData: {
          child_sender,
          child_receiver,
        },
      })

      buf_body = []
    }
  }
})

process.stdin.on('close', () => {
  process.exit()
})
