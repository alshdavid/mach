import avsc from '../vendor/avsc/index.js'
import * as process from 'node:process'

// let buff = []

// process.stdin.on('data', bytes => {
//   for (const byte of bytes) {
//     if (byte === 10) {
//       const collect = buff
//       buff = []
//       setTimeout(() => start(collect), 0)
//     } else {
//       buff.push(byte)
//     }
//   }
// });

const type = avsc.Type.forSchema('string')

const bytes = type.toBuffer("o w")
console.log(new Uint8Array(bytes)) // [ 6, 111, 32, 119 ]
console.log(new TextDecoder().decode(new Uint8Array(bytes))) // "o w"

const val = type.fromBuffer(Buffer.from([ 6, 111, 32, 119 ])) 
console.log(val) // "o w"

function start(bytes) {
  console.log(bytes)
  console.log(new TextDecoder().decode(new Uint8Array(bytes)))
}

process.stdin.on('end', () => process.exit())
process.stdin.on('close', () => process.exit());