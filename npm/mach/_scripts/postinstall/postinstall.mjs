import * as fs from "node:fs";
import * as path from "node:path";
import * as url from "node:url";
import * as child_process from "node:child_process";
import { http_get, http_get_json } from './utils.mjs'
import * as semver from "./vendor/semver/index.js";

const __dirname = url.fileURLToPath(new URL(".", import.meta.url));

const packageJson = JSON.parse(fs.readFileSync(path.join(__dirname, '..', '..', 'package.json'), 'utf8'))
const { bin: BUILD_TAG } = packageJson.mach || {}

if (BUILD_TAG == '' || process.env.MACH_SKIP_INSTALL === 'true') {
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

const RELEASE_MANIFEST = "https://api.github.com/repos/alshdavid/mach/releases"
const DOWNLOAD_TO = path.join(__dirname, "..", '..', "mach.tar.gz")

if (process.platform === 'win32') {
  fs.appendFileSync(path.join(__dirname, '..', '..', 'bin', 'bin.cmd'), fs.readFileSync(path.join(__dirname, 'bin.cmd')))
} else {
  fs.appendFileSync(path.join(__dirname, '..', '..', 'bin', 'bin.cmd'), fs.readFileSync(path.join(__dirname, 'bin.bash')))
}

fs.rmSync(path.join(__dirname, "..", '..', "mach"), { force: true });
fs.rmSync(path.join(__dirname, "..", '..', "mach.tar.gz"), { force: true });
const github_manifest = await http_get_json(RELEASE_MANIFEST)

let bin_url = ''
for (const gh_release of github_manifest) {
  if (semver.satisfies(gh_release.tag_name, BUILD_TAG, { includePrerelease: true })) {
    bin_url = `https://github.com/alshdavid/mach/releases/download/${gh_release.tag_name}/mach-${OS}-${ARCH}.tar.gz`
    break
  }
}
if (!bin_url) {
  console.error('Could not find Mach binary for specified version. Please compile from source')
  process.exit(0)
}

const buffer = await http_get(bin_url)

fs.writeFileSync(DOWNLOAD_TO, Buffer.from(buffer));
fs.writeFileSync(path.join(__dirname, 'bin_details.txt'), bin_url, 'utf8');

try {
  child_process.execSync(`tar -xzf mach.tar.gz`, { cwd: path.resolve(__dirname,  '..', '..'), stdio: 'inherit' })
  fs.rmSync(DOWNLOAD_TO, { force: true })
} catch (err) {
  console.error('Error: "tar" command is not installed')
}
