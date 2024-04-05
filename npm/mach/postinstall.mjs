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

// Might be cached by pnpm or yarn
if (ARCH && OS && !fs.existsSync(path.join(__dirname, `mach-${OS}-${ARCH}`))) {
  let bin_pkg_path = path.dirname(require.resolve(`@alshdavid/mach-${OS}-${ARCH}/package.json`))
  fs.cpSync(bin_pkg_path, path.join(__dirname, `mach-${OS}-${ARCH}`), { recursive: true })
}

if (OS === 'windows') {
  fs.appendFileSync(
    path.join(__dirname, 'bin.cmd'),
    fs.readFileSync(path.join(__dirname, 'bin', 'windows.cmd')),
    'utf8',
  )
} else {
  fs.appendFileSync(
    path.join(__dirname, 'bin.cmd'),
    fs.readFileSync(path.join(__dirname, 'bin', 'unix.bash')),
    'utf8',
  )
}
