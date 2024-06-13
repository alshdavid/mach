import { Worker } from 'node:worker_threads'
import path from 'node:path'
import { ROOT, machNapiNew, machNapiBuild, RpcCallbackMain, MachNapi, defaultThreadCount } from '../_napi/index.js'

export type MachOptions = {
  threads?: number
  nodeWorkers?: number
}

export type MachBuildOptions = {
  entries: string[]
  projectRoot?: string
  outFolder?: string
  clean?: boolean
  optimize?: boolean
  bundleSplitting?: boolean
}

export class Mach {
  #internal: MachNapi
  #nodeWorkerCount: number;

  constructor(options: MachOptions = {}) {
    options.threads = options.threads || defaultThreadCount()
    this.#nodeWorkerCount = options.nodeWorkers || options.threads || defaultThreadCount()
    this.#internal = machNapiNew({
      rpc: (...args: any) => this.#rpc(args),
      ...options,
    })
  }

  async #rpc([err, id, data, done]: RpcCallbackMain) {
    // console.log(["M", err, id, data, done])
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

  async build(options: MachBuildOptions) {
    return new Promise(async (res, rej) => {
      const workers = this.#startWorkers();

      let result = await machNapiBuild(
        this.#internal,
        options,
        (err, data) => err ? rej(err) : res(data),
      );

      this.#stopWorkers(await workers);
      return result;
    })
  }

  async #startWorkers() {
    const workersOnLoad = [];
    const workers = [];

    for (let i = 0; i < this.#nodeWorkerCount; i++) {
      let worker = new Worker(path.join(ROOT, 'bin', 'index.js'));
      workers.push(worker);
      workersOnLoad.push(
        new Promise(resolve => worker.once('message', resolve)),
      );
    }

    await Promise.all(workersOnLoad);
    return workers;
  }

  #stopWorkers(workers: Worker[]) {
    for (const worker of workers) {
      worker.terminate();
    }
  }
}
