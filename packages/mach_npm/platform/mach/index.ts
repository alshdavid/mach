import native from '../native/index.cjs'
import { Worker } from 'node:worker_threads'

export { native }

export function foo() {
  new Worker('/home/dalsh/Development/alshdavid/mach/packages/mach_npm/_/worker.js')
}

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
