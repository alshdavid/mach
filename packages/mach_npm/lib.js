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
  to_export = await import(`@alshdavid/mach-${OS}-${ARCH}/lib/index.js`)
} catch (error) {
  const fs = await import('node:fs/promises')
  const path = await import('node:path')
  const url = await import('node:url')

  const __dirname = url.fileURLToPath(new URL('.', import.meta.url))
  const package_json_path = path.join(__dirname, 'package.json')
  const package_json = JSON.parse(await fs.readFile(package_json_path, 'utf8'))
  
  if (package_json.version !== '0.0.0-local') {
    throw error
  }

  to_export = await import('@alshdavid/mach-os-arch')
}

class MachInitError extends Error {
  constructor() {
    super('Mach is not initialized')
    throw this
  }
}

export const Mach = (to_export.Mach || globalThis.Mach?.Mach || MachInitError)
export const Resolver = (to_export.Resolver || globalThis.Mach?.Resolver || MachInitError)
export const Transformer = (to_export.Transformer || globalThis.Mach?.Transformer || MachInitError)
export const Dependency = (to_export.Dependency || globalThis.Mach?.Dependency || MachInitError)
export const MutableAsset = (to_export.MutableAsset || globalThis.Mach?.MutableAsset || MachInitError)
