class MachInitError extends Error {
  constructor() {
    super('Mach is not initialized')
    throw this
  }
}

export const Resolver = globalThis.Mach?.Resolver || MachInitError
export const Transformer = globalThis.Mach?.Transformer || MachInitError
