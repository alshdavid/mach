class MachInitError extends Error {
  constructor() {
    super('Mach is not initialized')
    throw this
  }
}

export const Resolver = globalThis.Mach?.Resolver || MachInitError
export const Transformer = globalThis.Mach?.Transformer || MachInitError
export const Dependency = globalThis.Mach?.Dependency || MachInitError
export const MutableAsset = globalThis.Mach?.MutableAsset || MachInitError

let exports = undefined

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
  exports = await import(`@alshdavid/mach-${OS}-${ARCH}`)
} catch (error) {
  try {
    exports = await import(`@alshdavid/mach-os-arch`)
  } catch (err) {
    throw error    
  }
}

export const Mach = exports.Mach
