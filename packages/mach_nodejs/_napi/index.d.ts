export declare const ROOT: string;

export type NapiCallback<T extends Array<any>> = (...args: T) => any | Promise<any>

export type RpcCallbackDoneFunc<T> = (value: { Ok: T } | { Err: any }) => any | Promise<any>
export type RpcCallback = (
  NapiCallback<[error: any | null, id: 0, data: null, done: RpcCallbackDoneFunc<null>]>
)

export type BuildCallback = NapiCallback<[error: any, data: any]>

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