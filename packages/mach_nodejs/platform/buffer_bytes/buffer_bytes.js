export function buffer_bytes(
  /** @type {(value: string) => (any | Promise<any>)} */ callback,
) {
  /** @type {Array<any>} */
  let buffer = []

  return (/** @type {Array<any>} */ bytes) => {
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
