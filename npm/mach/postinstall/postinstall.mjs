import './vendor/node-fetch.cjs' // For Node 16
import * as fs from "node:fs"
import * as path from "node:path"
import * as url from "node:url"
import * as child_process from "node:child_process"

const __dirname = url.fileURLToPath(new URL(".", import.meta.url));

const packageJson = JSON.parse(fs.readFileSync(path.join(__dirname, '..', '..', 'package.json'), 'utf8'))
const BRANCH_NAME = process.env.MACH_NPM_INSTALL_BIN 
  ? process.env.MACH_NPM_INSTALL_BIN
  : packageJson.mach?.bin

if (BRANCH_NAME == '' || process.env.MACH_SKIP_INSTALL === 'true') {
  process.exit(0) 
}

const OS = {
  'win32': 'windows',
  'darwin': 'macos',
  'linux': 'linux'
}[process.platform]

const ARCH = {
  'arm64': 'arm64',
  'x64': 'amd64',
}[process.arch]

if (!ARCH || !OS) {
  console.warn('Could not find Mach binary for your system. Please compile from source')
  console.warn('Override the built in binary by setting the $MACH_BINARY_PATH environment variable')
  process.exit(0)
}

// 
// Populate the bin target to support either Windows or Unix OSes
//
if (process.platform === 'win32') {
  fs.appendFileSync(path.join(__dirname, '..', '..', 'bin', 'bin.cmd'), fs.readFileSync(path.join(__dirname, 'bin.cmd')))
} else {
  fs.appendFileSync(path.join(__dirname, '..', '..', 'bin', 'bin.cmd'), fs.readFileSync(path.join(__dirname, 'bin.bash')))
}

try {
  child_process.execSync(`tar -xzf mach.tar.gz`, { cwd: path.resolve(__dirname,  '..', '..'), stdio: 'inherit' })
  fs.rmSync(DOWNLOAD_TO, { force: true })
} catch (err) {
  console.error('Error: "tar" command is not installed')
}
