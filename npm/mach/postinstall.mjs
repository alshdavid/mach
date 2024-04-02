import * as fs from 'node:fs'
import * as path from 'node:path'
import { createRequire } from 'node:module';
import * as url from "node:url"

const __dirname = url.fileURLToPath(new URL(".", import.meta.url));
const require = createRequire(import.meta.url);

if (JSON.parse(fs.readFileSync(path.join(__dirname, 'package.json'), 'utf8')).version === "0.0.0-local") {
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
  console.warn('Override the built in binary by setting the $MACH_BIN_OVERRIDE environment variable')
}

const pwsh = `
@echo off
SET TARGET_PATH=
FOR /F %%I IN ('node -e "const { dirname } = require(\\"path\\"); process.stdout.write(dirname(require.resolve(\\"@alshdavid/mach-${OS}-${ARCH}/package.json\\")))"') DO @SET "TARGET_PATH=%%I"
"%TARGET_PATH%\\bin\\mach.exe" %*
`

const bash = `
#!/bin/sh
set -e

BIN_PATH=$MACH_BIN_OVERRIDE

if [ "$MACH_BIN_OVERRIDE" = "" ]; then
  BIN_PATH="$(node -e \"const { dirname } = require('node:path'); process.stdout.write(dirname(require.resolve('@alshdavid/mach-${OS}-${ARCH}/package.json')))\")"
  BIN_PATH="$BIN_PATH/bin/mach"
fi

"$BIN_PATH" $@
`

if (OS === 'windows') {
  fs.appendFileSync(path.join(__dirname, 'bin.cmd'), pwsh.trim(),  'utf8')
} else {
  fs.appendFileSync(path.join(__dirname, 'bin.cmd'), bash.trim(),  'utf8')
}
