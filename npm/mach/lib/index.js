class MachInitError extends Error {
  constructor() {
    super('Mach is not initialized')
    throw this
  }
}

export const Resolver = globalThis.Mach?.Resolver || new MachInitError()
export const Transformer = globalThis.Mach?.Transformer || new MachInitError()
