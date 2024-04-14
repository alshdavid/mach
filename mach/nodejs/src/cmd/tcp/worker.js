import * as net from 'node:net'
import { workerData } from 'node:worker_threads'
import { ACTION_TYPE } from '../../handlers/index.js';
import * as handlers from '../../handlers/index.js';

const PORT = globalThis.MACH_PORT || workerData.port
const client = new net.Socket()

globalThis.Mach = globalThis.Mach || {}
globalThis.Mach.ops = globalThis.Mach.ops || {}

let buf_mode = null
let buf_id = [null, null, null, null]
let buf_body = []

client.on('data', bytes => {
  for (const byte of bytes) {
    if (buf_mode === null) {
      buf_mode = byte
      continue
    }

    if (buf_id[0] === null) {
      buf_id[0] = byte
      continue
    }

    if (buf_id[1] === null) {
      buf_id[1] = byte
      continue
    }

    if (buf_id[2] === null) {
      buf_id[2] = byte
      continue
    }

    if (buf_id[3] === null) {
      buf_id[3] = byte
      continue
    }

    if (byte !== 10) {
      buf_body.push(byte)
      continue
    }

    const mode_local = buf_mode
    const id_local = buf_id
    const buf_body_local = buf_body

    buf_mode = null
    buf_id = [null, null, null, null]
    buf_body = []
  
    setTimeout(() => {
      if (!buf_body_local.length) {
        client.write(new Uint8Array([mode_local]))
        client.write(new Uint8Array(id_local))
        client.write(new Uint8Array([10]))
        return
      }

      const res = (/** @type {any} */ data) => {
        client.write(new Uint8Array([mode_local]))
        client.write(new Uint8Array(id_local))
        if (data) {
          client.write(JSON.stringify(data))
        }
        client.write(new Uint8Array([10]))
      }
  
      const data = JSON.parse(new TextDecoder().decode(new Uint8Array(buf_body_local)))
      const [header, body] = data
      
      switch (header) {
        case ACTION_TYPE.PING:
          handlers.ping(body, res)
          break;
        case ACTION_TYPE.RESOLVER_REGISTER:
          handlers.resolver_register(body, res)
          break;
        case ACTION_TYPE.RESOLVER_LOAD_CONFIG:
          handlers.resolver_load_config(body, res)
          break;
        case ACTION_TYPE.RESOLVER_RESOLVE:
          handlers.resolver_resolve(body, res)
          break;
        default:
          console.error('ERROR_NO_ACTION')
      }
    })
  }
});

globalThis.Mach.ops.ping = () => {

}

client.connect(PORT, '127.0.0.1')
