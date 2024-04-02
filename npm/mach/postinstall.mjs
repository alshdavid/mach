import * as fs from 'node:fs'
import * as path from 'node:path'
import { createRequire } from 'node:module';
import * as url from "node:url"

const __dirname = url.fileURLToPath(new URL(".", import.meta.url));
const require = createRequire(import.meta.url);

let bin_path = process.env.MACH_BIN_PATH

if (!bin_path) {
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

  const package_json_path = require.resolve(path.join('@alshdavid', `mach-${OS}-${ARCH}`, 'package.json'))
  const package_path = path.dirname(package_json_path)
  
  const json = JSON.parse(fs.readFileSync(package_json_path, 'utf8'))
  bin_path = path.join(package_path, json.bin.mach)
}

const pwsh = `
@echo off
"${bin_path}" %*
`

const bash = `
#!/bin/sh
"${bin_path}" $@
`

if (OS === 'windows') {
  fs.appendFileSync(path.join(__dirname, 'bin.cmd'), pwsh.trim(),  'utf8')
} else {
  fs.appendFileSync(path.join(__dirname, 'bin.cmd'), bash.trim(),  'utf8')
}
