import * as fs from 'node:fs'
import * as path from 'node:path'
import { createRequire } from 'node:module';
import * as url from "node:url"

const __dirname = url.fileURLToPath(new URL(".", import.meta.url));
const require = createRequire(import.meta.url);

if (JSON.parse(fs.readFileSync(path.join(__dirname, 'package.json'), 'utf8')).version === "0.0.0-local") {
  process.exit(0)
}

// If you BYO Mach binary, this install script will use that
if (process.env.MACH_BIN_OVERRIDE) {
  fs.rmSync(path.join(__dirname, 'bin.exe'))
  fs.linkSync(
    process.env.MACH_BIN_OVERRIDE,
    path.join(__dirname, 'bin.exe'), 
  )
  process.exit(0)
}

// Infer the binary based on the OS and Arch
const OS = {
  'win32': 'windows',
  'darwin': 'macos',
  'linux': 'linux'
}[process.platform]

const ARCH = {
  'arm64': 'arm64',
  'x64': 'amd64',
}[process.arch]

// If no binary is selected, gracefully exit
if (!OS && !ARCH) {
  console.warn('Could not find Mach binary for your system. Please compile from source')
  console.warn('Override the built in binary by setting the $MACH_BIN_OVERRIDE environment variable')
  process.exit(0)
}

let bin_pkg_json_path = require.resolve(`@alshdavid/mach-${OS}-${ARCH}/package.json`)
let bin_pkg_dir = path.dirname(bin_pkg_json_path)

fs.rmSync(path.join(__dirname, 'cmd'), {recursive : true, force: true })
fs.symlinkSync(
  path.relative(__dirname, bin_pkg_dir),
  path.join(__dirname, 'cmd'),
  'dir'
)