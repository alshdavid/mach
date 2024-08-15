// CLI runner
export function exec(args: string[]): any

// Mach
export type MachNapi = {
  readonly nodeWorkerCount: number
}

// Mach.new()
export type MachNapiOptions = {
  readonly threads?: number
  readonly nodeWorkers?: number
  readonly entries?: string | Array<string>
  readonly config?: string
  readonly env?: Record<string, string | undefined>
  readonly projectRoot?: string
  readonly outFolder?: string
}

export function machNapiNew(options: MachNapiOptions): MachNapi

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
