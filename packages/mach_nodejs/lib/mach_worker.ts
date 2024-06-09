import { Worker, parentPort } from 'node:worker_threads'
import path from 'node:path'
import { RpcWorkerCallbackData, MachWorkerNapi } from '../_napi/index.js'
import { ROOT } from '../_napi/index.js'

export class MachWorker {
  #internal: MachWorkerNapi

  constructor() {
    parentPort?.postMessage(null);
    this.#internal = new MachWorkerNapi({
      rpc: (...args: any) => this.#rpc(args),
    })
  }

  async #rpc([err, id, data, done]: RpcWorkerCallbackData) {
    console.log(["W", err, id, data])
    if (err) {
      return done({ Err: err })
    }
    switch (id) {
      case 0:
        done({ Ok: null })
        break
      default:
        done({ Err: 'No handler specified' })
    }
  }

  // Start a Nodejs worker thread
  static async init() {
    const worker = new Worker(path.join(ROOT, 'bin', 'index.js'))
    let resolve: any
    const onReady = new Promise(res => { resolve = res })
    worker.addListener("message", resolve)
    await onReady
    worker.removeListener("message", resolve)
    return worker
  }
}
