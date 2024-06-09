export declare const ROOT: string;

export type NapiCallback<T extends Array<any>> = (error: any | undefined, ...args: T) => any | Promise<any>
export type RpcCallback = NapiCallback<[any]>
export type BuildCallback = NapiCallback<[any]>

export type MachNapiOptions = {
  nodeWorkers?: number
  threads?: number
  rpc?: RpcCallback
}

export type MachNapiBuildOptions = {
  entries?: string[]
  projectRoot?: string
  outFolder?: string
  clean?: boolean
  optimize?: boolean
  bundleSplitting?: boolean
}

export declare class MachNapi {
  constructor(options: MachNapiOptions)
  build(options: MachNapiBuildOptions, callback: BuildCallback): any
}