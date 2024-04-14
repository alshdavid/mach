import * as net from 'node:net'
import * as url from 'node:url';
import { Worker, workerData, isMainThread } from 'node:worker_threads';
import * as ACTION_TYPE from './action_type.js'
import * as handlers from './handlers/index.js';
import { getPortsFromStdin } from './ipc/get_port_from_stdin.js';

const __filename = url.fileURLToPath(import.meta.url);
let port = /** @type {number | undefined} */ (workerData)

if (isMainThread) {
  const ports = await getPortsFromStdin()

  for (let i = 1; i < ports.length; i++) {
    new Worker(__filename, { workerData: ports.pop() })
  }

  port = ports.pop()
}

const client = new net.Socket();

let buf_id = null
let buf_header = null
let buf_body = []

client.on('data', bytes => {
  for (const byte of bytes) {
    if (buf_id === null) {
      buf_id = byte
    } else if (buf_header === null) {
      buf_header = byte
    } else if (byte !== 10) {
      buf_body.push(byte)
    } else {
      const id = buf_id
      const header = buf_header
      const body = buf_body
      buf_id = null
      buf_header = null
      buf_body = []

      let json = undefined
      if (body.length !== 0) {
        const str = new TextDecoder().decode(new Uint8Array(body))
        json = JSON.parse(str)
      }
      
      const res = (/** @type {any} */ data) => {
        client.write(new Uint8Array([id]))
        client.write(JSON.stringify(data))
        client.write(new Uint8Array([10]))
      }
      
      switch (header) {
        case ACTION_TYPE.PING:
          handlers.ping(json, res)
          break;
        case ACTION_TYPE.RESOLVER_REGISTER:
          handlers.resolver_register(json, res)
          break;
        case ACTION_TYPE.RESOLVER_LOAD_CONFIG:
          handlers.resolver_load_config(json, res)
          break;
        case ACTION_TYPE.RESOLVER_RESOLVE:
          handlers.resolver_resolve(json, res)
          break;
        default:
          console.error('ERROR_NO_ACTION')
      }
    }
  }
});

client.connect(port, '127.0.0.1');

process.stdin.on('end', () => process.exit())
process.stdin.on('close', () => process.exit());
