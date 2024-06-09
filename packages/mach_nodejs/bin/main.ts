import { Worker } from 'node:worker_threads'
const path = await import('node:path')
const url = await import('node:url')

const __dirname = url.fileURLToPath(new URL('.', import.meta.url))
new Worker(path.join(__dirname, './index.js'))
