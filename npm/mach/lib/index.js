class MachInitError extends Error {
  constructor() {
    super('Mach is not initialized')
    throw this
  }
}

export * from './mach.js'
export const Resolver = globalThis.Mach?.Resolver || MachInitError
export const Transformer = globalThis.Mach?.Transformer || MachInitError
export const Dependency = globalThis.Mach?.Dependency || MachInitError
export const MutableAsset = globalThis.Mach?.MutableAsset || MachInitError
