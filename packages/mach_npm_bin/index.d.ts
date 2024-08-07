// CLI runner
export function exec(): any

// Mach
export type MachNapi = {
  nodeWorkerCount: number
}

// Mach.new()
export type MachNapiOptions = {
  nodeWorkers?: number
  threads?: number
}

export function machNapiNew(options: MachNapiOptions, callback: any): MachNapi

// Mach.build()
export type MachNapiBuildOptions = {
  entries?: string[]
  projectRoot?: string
  outFolder?: string
  clean?: boolean
  optimize?: boolean
  bundleSplitting?: boolean
}

export function machNapiBuild(
  mach: MachNapi,
  options: MachNapiBuildOptions,
  callback: any,
): any

// Worker.new()
export function machWorkerNew(): any


