import * as fs from 'node:fs'
import * as path from 'node:path'
import { createRequire } from 'node:module';
import * as url from "node:url"

const __dirname = url.fileURLToPath(new URL(".", import.meta.url));
const require = createRequire(import.meta.url);

if (JSON.parse(fs.readFileSync(path.join(__dirname, 'package.json'), 'utf8')).version === "0.0.0-local") {
  process.exit(0)
}

const OS_TYPE = {
  'win32': 'windows',
  'darwin': 'macos',
  'linux': 'linux'
}

const ARCH_TYPE = {
  'arm64': 'arm64',
  'x64': 'amd64',
}

const OS = OS_TYPE[process.platform]
const ARCH = ARCH_TYPE[process.arch]

if (process.env.MACH_EXEC_PATH_OVERRIDE) {
  fs.rmSync(path.join(__dirname, 'bin.exe'))
  fs.linkSync(
    process.env.MACH_EXEC_PATH_OVERRIDE,
    path.join(__dirname, 'bin.exe'), 
  )
  process.exit(0)
}

let bin_pkg_json_path = require.resolve(`@alshdavid/mach-${OS}-${ARCH}/package.json`)
let bin_pkg_dir = path.dirname(bin_pkg_json_path)

let bin_pkg_json = JSON.parse(fs.readFileSync(bin_pkg_json_path, 'utf8'))

if (OS === 'windows' && fs.existsSync(path.join(__dirname, `mach-${OS}-${ARCH}`))) {
  fs.rmSync(path.join(__dirname, 'bin.exe'))
  fs.linkSync(
    path.join(bin_pkg_dir, bin_pkg_json.bin),
    path.join(__dirname, 'bin.exe'), 
  )
} else {
  fs.rmSync(path.join(__dirname, 'bin.exe'))
  fs.linkSync(
    path.join(bin_pkg_dir, bin_pkg_json.bin),
    path.join(__dirname, 'bin.exe'), 
  )
}
