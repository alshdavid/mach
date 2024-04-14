import * as process from 'node:process'
import * as path from 'node:path'
import * as url from 'node:url';
import { Worker } from 'node:worker_threads'

const __dirname = url.fileURLToPath(new URL('.', import.meta.url));
const WORKER_PATH = path.join(__dirname, 'worker.js')

process.stdin.on('end', () => process.exit())
process.stdin.on('close', () => process.exit());

const workers = []
let buf_body = []

process.stdin.on('data', async bytes => {
  for (const byte of bytes) {
    if (byte == 10) {
      const str = new TextDecoder().decode(new Uint8Array(buf_body))
      const port = JSON.parse(str)
      if (workers.length === 0) {
        globalThis.MACH_PORT = port
        await import('./worker.js')
        workers.push(globalThis)
      } else {
        const worker = new Worker(WORKER_PATH, { workerData: { port }})
        workers.push(worker)
      }
      buf_body = []
      continue
    }
    buf_body.push(byte)
  }
});
