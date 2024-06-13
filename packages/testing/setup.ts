import * as path from 'node:path'
import * as url from 'node:url'
import { Mach } from '@alshdavid/mach'

const __dirname = url.fileURLToPath(new URL('.', import.meta.url))

const mach = new Mach({
  nodeWorkers: 16,
})

const result = await mach.build({
  entries: ['./src/index.js'],
  projectRoot: path.join(__dirname, 'example'),
  outFolder: path.join(__dirname, 'example', 'dist'),
})

console.log(result)

// await new Promise(res => setTimeout(res, 2000))
// console.log({ result })
