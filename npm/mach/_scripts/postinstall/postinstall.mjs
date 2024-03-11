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

const GH_API_URL = "https://api.github.com/repos/alshdavid/mach"

const OS_MAP = {
  'win32': 'windows',
  'darwin': 'macos',
  'linux': 'linux'
}

const ARCH_MAP = {
  'arm64': 'arm64',
  'x64': 'amd64',
}

const ARCHIVE_TYPE_MAP = {
  'gzip': 'gzip',
  'lzma': 'lzma',
}

const ARCHIVE_TARGET_MAP = {
  [`${OS_MAP.linux}-${ARCH_MAP.x64}`]: [`mach-${OS_MAP.linux}-${ARCH_MAP.x64}.tar.xz`, ARCHIVE_TYPE_MAP.lzma],
  [`${OS_MAP.linux}-${ARCH_MAP.arm64}`]: [`mach-${OS_MAP.linux}-${ARCH_MAP.arm64}.tar.xz`, ARCHIVE_TYPE_MAP.lzma],
  [`${OS_MAP.darwin}-${ARCH_MAP.x64}`]: [`mach-${OS_MAP.darwin}-${ARCH_MAP.x64}.tar.xz`, ARCHIVE_TYPE_MAP.lzma],
  [`${OS_MAP.darwin}-${ARCH_MAP.arm64}`]: [`mach-${OS_MAP.darwin}-${ARCH_MAP.arm64}.tar.xz`, ARCHIVE_TYPE_MAP.lzma],
  [`${OS_MAP.win32}-${ARCH_MAP.x64}`]: [`mach-${OS_MAP.win32}-${ARCH_MAP.x64}.tar.gz`, ARCHIVE_TYPE_MAP.gzip],
  // [`${OS_MAP.win32}-${ARCH_MAP.arm64}`]: [`mach-${OS_MAP.win32}-${ARCH_MAP.arm64}.tar.gz`, ARCHIVE_TYP_MAP.gzip],
}

const OS = OS_MAP[process.platform]
const ARCH = ARCH_MAP[process.arch]

const [ARCHIVE_TARGET, ARCHIVE_TYPE] = ARCHIVE_TARGET_MAP[`${OS}-${ARCH}`]

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

// 
// Find the binary with the latest tag
//

let bin_url = undefined

for await (const release of get_gh_releases()) {
  if (release.tag_name.startsWith(`${BRANCH_NAME}.`)) {
    bin_url = `https://github.com/alshdavid/mach/releases/download/${release.tag_name}/${ARCHIVE_TARGET}`
    break
  }
}

if (!bin_url) {
  console.error('Could not find Mach binary for specified version. Please compile from source')
  process.exit(0)
}

// 
// Download and extract the latest version
//
const DOWNLOAD_TO = path.join(__dirname, "..", '..', ARCHIVE_TARGET)

fs.rmSync(path.join(__dirname, "..", '..', "mach"), { force: true });
fs.rmSync(path.join(__dirname, "..", '..', ARCHIVE_TARGET), { force: true });

const buffer = await fetch(bin_url).then(r => r.arrayBuffer())

fs.writeFileSync(DOWNLOAD_TO, Buffer.from(buffer));
fs.writeFileSync(path.join(__dirname, 'bin_details.txt'), bin_url, 'utf8');

try {
  if (ARCHIVE_TYPE === ARCHIVE_TYPE_MAP.gzip) {
    child_process.execSync(`tar -xzf ${ARCHIVE_TARGET}`, { cwd: path.resolve(__dirname,  '..', '..'), stdio: 'inherit' })
  }
  if (ARCHIVE_TYPE === ARCHIVE_TYPE_MAP.lzma) {
    child_process.execSync(`tar -Jxvf ${ARCHIVE_TARGET}`, { cwd: path.resolve(__dirname,  '..', '..'), stdio: 'inherit' })
  }
  fs.rmSync(DOWNLOAD_TO, { force: true })
} catch (err) {
  console.error('Error: "tar" command is not installed')
}









//
// UTILS
//
export async function* get_gh_releases() {
  let page = 1
  while (true) {
    const response = await fetch(`${GH_API_URL}/releases?per_page=100&page=${page}`)
    if (!response.ok) {
      throw new Error(await response.text())
    }
    const results = await response.json()
    if (!results.length) {
      break
    }
    for (const result of results) {
      yield result
    }
    page += 1
  }
}
