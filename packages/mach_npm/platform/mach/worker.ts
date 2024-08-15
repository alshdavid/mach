import * as url from 'node:url'
import { Worker } from 'node:worker_threads'

export class MachWorker extends Worker {
  constructor() {
    let workerPath = url.fileURLToPath(import.meta.resolve('#worker'))
    if (workerPath.endsWith('.ts')) {
      super(
        `import('tsx/esm/api').then(({ register }) => { register(); import('${workerPath}') })`,
        { eval: true },
      )
    } else {
      super(workerPath)
    }
  }
}
