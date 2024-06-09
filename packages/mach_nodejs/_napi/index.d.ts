export declare const ROOT: string;

export type RpcCallback = () => any | Promise<any>

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
  build(options: MachNapiBuildOptions): any
}