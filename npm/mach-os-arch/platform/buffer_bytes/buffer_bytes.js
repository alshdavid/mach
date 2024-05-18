export function buffer_bytes(callback) {
  let buffer = []
  return (bytes) => {
    for (const byte of bytes) {
      // Characters
      if (byte !== 10) {
        buffer.push(byte)
      }
  
      // Newline
      if (byte === 10) {
        const str = new TextDecoder().decode(new Uint8Array(buffer))
        callback(str)
        buffer = []
      }
    }
  }
}
