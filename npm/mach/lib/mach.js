import * as child_process from 'node:child_process'
import * as url from 'node:url'
import * as path from 'node:path'
import * as net from 'node:net'
import * as types from '../types/index.js'

const __dirname = url.fileURLToPath(new URL('.', import.meta.url))
const BIN_PATH = path.resolve(__dirname, '..', 'cmd', 'bin', 'mach.exe')

/**
 * @class
 * @implements {types.Mach}
 */
export class Mach {
  /** @type {Set<(value: any) => Promise<any>>} */
  #subscribers

  constructor(/** @type {types.MachOptions} */ options) {
    this.#subscribers = new Set()
  }

  /** @return {ReturnType<types.Mach['build']>} */
  async build(/** @type {Parameters<types.Mach['build']>[0]} */ options) {
    let trigger_error
    let trigger_complete

    /** @type {Promise<types.BuildReport>} */
    let on_complete = new Promise((resolve, reject) => {
      trigger_complete = resolve
      trigger_error = reject
    })

    const server = net.createServer((socket) => {
      let buff_output = []
      socket.on('data', (bytes) => {
        for (const byte of bytes) {
          // Characters
          if (byte !== 10) {
            buff_output.push(byte)
          }

          // Newline
          if (byte === 10) {
            const message = JSON.parse(
              new TextDecoder().decode(new Uint8Array(buff_output)),
            )
            buff_output = []

            if ('buildReport' in message) {
              const { data } = message['buildReport']
              trigger_complete(data)
            }

            if ('buildEvent' in message) {
              const { data } = message['buildEvent']
              this.#next(data)
            }

            if ('buildError' in message) {
              const { data } = message['buildEvent']
              this.#next(data)
            }
          }
        }
      })
    })

    await new Promise((res) => server.listen(9494, '127.0.0.1', () => res()))

    const cli_args = /** @type {Array<string>} */ (['build'])

    if (options.bundleSplitting)
      cli_args.push('--bundle-splitting', `${options.bundleSplitting}`)
    if (options.clean) cli_args.push('--clean')
    if (options.nodeWorkers)
      cli_args.push('--node-workers', `${options.nodeWorkers}`)
    if (options.optimize) cli_args.push('--no-optimize', `${!options.optimize}`)
    if (options.outFolder) cli_args.push('--dist', options.outFolder)
    if (options.threads) cli_args.push('--threads', `${options.threads}`)

    for (const entry of options.entries) {
      cli_args.push(entry)
    }

    const child = child_process.spawn(BIN_PATH, cli_args, {
      shell: true,
      cwd: options.projectRoot ?? process.cwd(),
      env: {
        // @ts-expect-error
        MACH_DIAGNOSTIC_PORT: `${server.address().port}`,
        ...process.env,
      },
    })

    let buff_error = ''
    child.stderr.on('data', (data) => {
      buff_error += data
    })

    child.on('exit', function (code, signal) {
      if (code === 1 && buff_error) trigger_error(buff_error)
    })

    let result = await on_complete
    server.close()
    return result
  }

  /** @return {ReturnType<types.Mach['dev']>} */
  dev(/** @type {Parameters<types.Mach['dev']>[0]} */ options) {
    throw new Error('Method not implemented.')
  }

  /** @return {ReturnType<types.Mach['watch']>} */
  watch(/** @type {Parameters<types.Mach['watch']>[0]} */ options) {
    throw new Error('Method not implemented.')
  }

  /** @return {ReturnType<types.Mach['serve']>} */
  serve(/** @type {Parameters<types.Mach['serve']>[0]} */ options) {
    throw new Error('Method not implemented.')
  }

  /** @return {ReturnType<types.Mach['subscribe']>} */
  // @ts-expect-error
  subscribe(
    /** @type {Parameters<types.Mach['subscribe']>[0]} */ event_name,
    /** @type {Parameters<types.Mach['subscribe']>[1]} */ callback,
  ) {
    const fn = async (event) =>
      event.action === event_name && callback(event.data)
    this.#subscribers.add(fn)
    return () => {
      this.#subscribers.delete(fn)
    }
  }

  #next(/** @type {any} */ event) {
    for (const subscriber of this.#subscribers) {
      setTimeout(() => subscriber(event), 0)
    }
  }

  static build(options) {
    const mach = new Mach(options)
    return mach.build(options)
  }
}
