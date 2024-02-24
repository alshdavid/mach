import * as fs from "node:fs";
import * as path from "node:path";
import * as http from "node:https";
import * as url from "node:url";
import * as child_process from "node:child_process";

const __dirname = url.fileURLToPath(new URL(".", import.meta.url));
const packageJson = JSON.parse(fs.readFileSync(path.join(__dirname, './package.json'), 'utf8'))

const BUILD_TAG = '{{BUILD_TAG}}'

if (packageJson.version === '0.0.0' || process.env.MACH_SKIP_INSTALL === 'true') {
  console.log('skip download')
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
  process.exit(0)
}

const BIN_URL = `https://github.com/alshdavid/mach/releases/download/${BUILD_TAG}/mach-${OS}-${ARCH}.tar.gz`;
const DOWNLOAD_TO = path.join(__dirname, "..", "mach.tar.gz")

if (process.platform === 'win32') {
  fs.appendFileSync(path.join(__dirname, '..', 'bin.cmd'), fs.readFileSync(path.join(__dirname, 'bin.cmd')))
} else {
  fs.appendFileSync(path.join(__dirname, '..', 'bin.cmd'), fs.readFileSync(path.join(__dirname, 'bin.bash')))
}

fs.rmSync(path.join(__dirname, "..", "mach"), { force: true });
fs.rmSync(path.join(__dirname, "..", "mach.tar.gz"), { force: true });

function download_file_legacy(target_url, download_to) {
  let buffer = undefined

  function _download_file(target_url) {
    return new Promise(res => {
      const request = http.get(target_url, async (response) => {
        const redirect = response.headers.location;

        if (redirect) {
          res(await _download_file(redirect))
        } else {
          response.on('end', () => res(buffer))
          response.on('data', (chunk) => {
            if (!buffer) {
              buffer = chunk;
            } else {
              buffer = Buffer.concat([buffer, chunk]);
            }
          })
        }
      });
      request.end()
    })
  }
    
  return _download_file(target_url)
}

const buffer = globalThis.fetch 
  ? await fetch(BIN_URL).then(res => res.arrayBuffer()) 
  : await download_file_legacy(BIN_URL)

fs.writeFileSync(DOWNLOAD_TO, Buffer.from(buffer));

try {
  child_process.execSync(`tar -xzf mach.tar.gz`, { cwd: path.resolve(__dirname,  '..'), stdio: 'inherit' })
  fs.rmSync(DOWNLOAD_TO, { force: true })
} catch (err) {
  console.error('Error: "tar" command is not installed')
}
