import { MachNapi, machNapiNew } from '../_/napi.cjs'

export type BuildReport = {
  entries: Record<string, string>
}

export type MachOptions = {
  threads?: number
  nodeWorkers?: number
}

export type MachBuildOptions = {
  entries: string[]
  projectRoot?: string
  outFolder?: string
  clean?: boolean
  optimize?: boolean
  bundleSplitting?: boolean
}

export class Mach {
  constructor(options: MachOptions = {}) {}

  static build(options: MachBuildOptions & MachOptions): Promise<BuildReport> {
    const mach = new Mach(options)
    return mach.build(options)
  }

  async build(options: MachBuildOptions): Promise<BuildReport> {
    return {} as any
  }
}
