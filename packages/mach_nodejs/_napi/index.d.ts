export declare const ROOT: string;

export type NapiCallback<T extends Array<any>> = (...args: T) => any | Promise<any>

export type RpcCallbackDoneFunc<T> = (value: { Ok: T } | { Err: any }) => any | Promise<any>
export type RpcCallbackBase<I extends number, D, R> = [err: any | null, id: I, data: D, RpcCallbackDoneFunc<R>]
export type RpcCallbackData = (
  // Ping
  RpcCallbackBase<0, null, null> |
  // Start worker
  RpcCallbackBase<1, null, null>
)

export type RpcWorkerCallbackData = (
  // Ping
  RpcCallbackBase<0, null, null>
)

export type BuildCallback = NapiCallback<[error: any, data: any]>

export type MachNapiOptions = {
  nodeWorkers?: number
  threads?: number
  rpc?: any
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

export declare class MachWorkerNapi {
  constructor(options: any)
}
