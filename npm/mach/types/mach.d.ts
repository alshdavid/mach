export type MachOptions = {}

export type DevOptions = {}
export type ServeOptions = {}
export type WatchOptions = {}

export type DisposeFunc = () => void

export type BuildOptions = {
  /** @description Filepaths to the files to compile */
  entries: string[]
  /** @description The root directory for the build */
  projectRoot?: string
  /** @description Target output folder */
  outFolder?: string
  /** @description Delete output folder before emitting files */
  clean?: boolean
  /** @description Enable optimizations like minification, tree shaking, etc */
  optimize?: boolean
  /** @description Enable bundle splitting */
  bundleSplitting?: boolean
  /** @description How many threads to use for compilation */
  threads?: number
  /** @description How many Node.js workers to spawn for plugins */
  nodeWorkers?: number
}

export type BuildProgress = {}

export type BuildReport = {
  bundleManifest: Record<string, string>
  output: Record<string, string>
}

export declare class Mach {
  constructor(options?: MachOptions)
  build(options: BuildOptions): Promise<BuildReport>
  /** @todo */
  dev(options: DevOptions): DisposeFunc
  /** @todo */
  watch(options: WatchOptions): DisposeFunc
  /** @todo */
  serve(options: ServeOptions): DisposeFunc
  /** @todo */
  subscribe(
    type: 'build_start',
    callback: (value: BuildOptions) => any | Promise<any>,
  ): DisposeFunc
  /** @todo */
  subscribe(
    type: 'build_progress',
    callback: (value: BuildProgress) => any | Promise<any>,
  ): DisposeFunc
  /** @todo */
  subscribe(
    type: 'build_end',
    callback: (value: BuildReport) => any | Promise<any>,
  ): DisposeFunc
  static build(options: MachOptions & BuildOptions): Promise<BuildReport>
}

export interface IMach extends Mach {}
