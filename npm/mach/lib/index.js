const OS = {
  win32: 'windows',
  darwin: 'macos',
  linux: 'linux',
}[process.platform]

const ARCH = {
  arm64: 'arm64',
  x64: 'amd64',
}[process.arch]

let exports = undefined

try {
  exports = await import(`@alshdavid/mach-${OS}-${ARCH}/lib/index.js`)
} catch (error) {
  const fs = await import('node:fs/promises')
  const path = await import('node:path')
  const url = await import('node:url')
  
  const __dirname = url.fileURLToPath(new URL('.', import.meta.url))

  const package_json = JSON.parse(await fs.readFile(path.join(__dirname, '..', 'package.json'), 'utf-8'))
  if (package_json.version !== '0.0.0-local') {
    throw error
  }

  exports = await import('@alshdavid/mach-os-arch/lib/index.js')
}

class MachInitError extends Error {
  constructor() {
    super('Mach is not initialized')
    throw this
  }
}

export const Mach = exports.Mach || globalThis.Mach?.Mach || MachInitError
export const Resolver =
  exports.Resolver || globalThis.Mach?.Resolver || MachInitError
export const Transformer =
  exports.Transformer || globalThis.Mach?.Transformer || MachInitError
export const Dependency =
  exports.Dependency || globalThis.Mach?.Dependency || MachInitError
export const MutableAsset =
  exports.MutableAsset || globalThis.Mach?.MutableAsset || MachInitError
