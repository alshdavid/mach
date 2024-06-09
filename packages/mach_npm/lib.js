export let Mach
export let Resolver
export let Transformer
export let Dependency
export let MutableAsset

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

class MachInitError extends Error {
  constructor() {
    super('Mach is not initialized')
    throw this
  }
}

export function _apply_exports(mach) {
  Mach = (mach.Mach || globalThis.Mach?.Mach || MachInitError)
}
