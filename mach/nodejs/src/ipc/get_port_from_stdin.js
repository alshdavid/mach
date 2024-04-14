export async function getPortsFromStdin() {
  const ports = /** @type {number[]} */ ([])
  
  let worker_count = /** @type {number | undefined} */ (undefined)
  let buf_port = /** @type {number[]} */ ([])

  await new Promise(res => {
    function onStdin(/** @type {Buffer} */ bytes) {
      for (const byte of bytes) {
        if (worker_count === undefined) {
          worker_count = byte
        } else if (byte !== 10) {
          buf_port.push(byte)
        } else {
          const port = JSON.parse(new TextDecoder().decode(new Uint8Array(buf_port)))
          ports.push(port)
          if (ports.length == worker_count) {
            process.stdin.off('data', onStdin)
            res()
          }
          buf_port = []
        }
      }
    }
    process.stdin.on('data', onStdin)
  })

  return ports
}