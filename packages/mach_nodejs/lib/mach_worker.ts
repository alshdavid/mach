import { Worker, parentPort } from 'node:worker_threads'
import path from 'node:path'
import { RpcCallbackMain, RpcCallbackWorker, workerCallback } from '../_napi/index.js'
import { ROOT } from '../_napi/index.js'

export class MachWorker {
  #rpcCallback: any

  constructor() {
    this.#rpcCallback = (...args: any) => {this.#rpc(args)}
    workerCallback(this.#rpcCallback)
    parentPort?.postMessage(null);
  }

  async #rpc([err, id, data, done]: RpcCallbackWorker) {
    console.log(["W", err, id, data, done])
    if (err) {
      return done({ Err: err })
    }
    switch (id) {
      case 0:
        done({ Ok: undefined })
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
