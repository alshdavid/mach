import * as process from 'node:process'
import * as ACTION_TYPE from './action_type.js'
import * as handlers from './handlers/index.js';

let buf_id = null
let buf_header = null
let buf_body = []

process.stdin.on('data', bytes => {
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

      setTimeout(() => {
        const str = new TextDecoder().decode(new Uint8Array(body))
        const json = JSON.parse(str)
        
        const res = (/** @type {any} */ data) => {
          process.stdout.write(new Uint8Array([id]))
          process.stdout.write(JSON.stringify(data))
          process.stdout.write(new Uint8Array([10]))
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
      }, 0)
    }
  }
});

process.stdin.on('end', () => process.exit())
process.stdin.on('close', () => process.exit());