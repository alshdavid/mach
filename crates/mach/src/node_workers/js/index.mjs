import { stdin } from "node:process";

let buffer = ''
process.stdin.on('data', (d) => {
  for (const char of d.toString()) {
    if (char === '\n') {
      on_stdin(buffer)
      buffer = ''
      continue
    }
    buffer += char
  }
})

stdin.on('close', () => process.exit());
stdin.on('end', () => process.exit());

function on_stdin(/** @type {string} */ data) {
  process.stdout.write(`${data}\n`)
}
