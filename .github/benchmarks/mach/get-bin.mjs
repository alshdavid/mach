import * as path from 'node:path'
import * as fs from 'node:fs'
import * as url from 'node:url';

const __dirname = url.fileURLToPath(new URL('.', import.meta.url));

let PROFILE = process.env.profile || 'release-lto'
let PROFILE_BIN = path.join(PROFILE, 'bin', 'mach')

let current_dir = __dirname

while (true) {
  if (fs.existsSync(path.join(current_dir, 'Cargo.lock'))) {
    current_dir = current_dir
    break
  }
  current_dir = path.dirname(current_dir)
}

let target_bin = path.join(current_dir, 'target', PROFILE_BIN)
if (!fs.existsSync(target_bin)) {
  console.error('could not find', target_bin)
  process.exit(1)
}

process.stderr.write(`Using ${target_bin}\n`)
process.stdout.write(target_bin)
