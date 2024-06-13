/** @type {any} */ export let Mach
/** @type {any} */ export let Resolver
/** @type {any} */ export let Transformer
/** @type {any} */ export let Dependency
/** @type {any} */ export let MutableAsset

const OS = {
  win32: 'windows',
  darwin: 'macos',
  linux: 'linux',
}[process.platform]

const ARCH = {
  arm64: 'arm64',
  x64: 'amd64',
}[process.arch]

try {
  _apply_exports(await import(`@alshdavid/mach-${OS}-${ARCH}/lib/index.js`))
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

  _apply_exports(await import('@alshdavid/mach-os-arch'))
}

export function _apply_exports(/** @type {any} */ mach) {
  class MachInitError extends Error {
    constructor() {
      super('Mach is not initialized')
      throw this
    }
  }

  Mach = mach.Mach || globalThis.Mach?.Mach || MachInitError
  Resolver = mach.Resolver || globalThis.Mach?.Resolver || MachInitError
  Transformer =
    mach.Transformer || globalThis.Mach?.Transformer || MachInitError
  Dependency = mach.Dependency || globalThis.Mach?.Dependency || MachInitError
  MutableAsset =
    mach.MutableAsset || globalThis.Mach?.MutableAsset || MachInitError
}
