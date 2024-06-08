const OS = {
  win32: 'windows',
  darwin: 'macos',
  linux: 'linux',
}[process.platform]

const ARCH = {
  arm64: 'arm64',
  x64: 'amd64',
}[process.arch]

let to_export = undefined

try {
  to_export = require(`@alshdavid/mach-${OS}-${ARCH}`)
} catch (error) {
  const fs = require('node:fs')
  const path = require('node:path')

  const package_json = JSON.parse(
   fs.readFileSync(path.join(__dirname, '..', 'package.json'), 'utf-8'),
  )
  if (package_json.version !== '0.0.0-local') {
    throw error
  }

  to_export = require('@alshdavid/mach-os-arch')
}

class MachInitError extends Error {
  constructor() {
    super('Mach is not initialized')
    throw this
  }
}

module.exports.Mach = (to_export.Mach || globalThis.Mach?.Mach || MachInitError)
module.exports.Resolver = (to_export.Resolver || globalThis.Mach?.Resolver || MachInitError)
module.exports.Transformer = (to_export.Transformer || globalThis.Mach?.Transformer || MachInitError)
module.exports.Dependency = (to_export.Dependency || globalThis.Mach?.Dependency || MachInitError)
module.exports.MutableAsset = (to_export.MutableAsset || globalThis.Mach?.MutableAsset || MachInitError)
