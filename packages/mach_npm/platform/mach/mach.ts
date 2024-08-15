import native, { MachNapi, MachNapiOptions } from '../native/index.cjs'

export type BuildReport = {
  readonly assets: Record<string, string>
}

export type MachOptions = MachNapiOptions

export type MachBuildOptions = {
  readonly clean?: boolean
  readonly optimize?: boolean
  readonly bundleSplitting?: boolean
}

export class Mach {
  #inner: MachNapi

  constructor(options: MachOptions = {}) {
    this.#inner = native.machNapiNew(options)
  }

  static build(options: MachBuildOptions & MachOptions): Promise<BuildReport> {
    return new Mach(options).build(options)
  }

  async build(options: MachBuildOptions = {}): Promise<BuildReport> {
    native.machNapiBuild(this.#inner, options, () => {})
    return {} as any
  }
}
