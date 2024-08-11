// This file selects the correct binary npm package based on the current OS/ARCH
// It falls back to the local version if not found otherwise errors
const OS = process.env.MACH_OS || {
  win32: 'windows',
  darwin: 'macos',
  linux: 'linux',
}[process.platform]

const ARCH = process.env.MACH_ARCH || {
  arm64: 'arm64',
  x64: 'amd64',
}[process.arch]

try {
  module.exports = require(`@alshdavid/mach-${OS}-${ARCH}`)
} catch (error) {
  const fs = require('node:fs')
  const path = require('node:path')

  const package_json_path = path.join(__dirname, '..', '..', 'package.json')
  const package_json = JSON.parse(fs.readFileSync(package_json_path, 'utf8'))

  if (package_json.version !== '0.0.0-local') {
    throw new Error('Could not find Mach binary for your system. Please compile from source')
  }

  module.exports = require('@alshdavid/mach-os-arch')
}