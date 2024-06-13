import { parentPort } from 'node:worker_threads'
import { RpcCallbackWorker, workerCallback } from '../_napi/index.js'

export class MachWorker {
  constructor() {
    workerCallback((...args: any) => this.#rpc(args))
    parentPort?.postMessage(null);
  }

  async #rpc([err, id, data, done]: RpcCallbackWorker) {
    // console.log(["W", (await import('node:worker_threads')).threadId, err, id, data, done])
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
}
