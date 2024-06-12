export declare const ROOT: string

export type Callback<T extends Array<any>, R = (any | Promise<any>)> = (
  ...args: T
) => R

export type RpcCallbackBase<I extends number, D, R> = [
  err: any | null,
  id: I,
  data: D,
  done: (value: { Ok: R } | { Err: any }) => any | Promise<any>,
]
export type RpcCallbackMain =
  // Ping
  | RpcCallbackBase<0, null, undefined>
  // Start worker
  | RpcCallbackBase<1, null, undefined>

export type RpcCallbackWorker =
  // Ping
  | RpcCallbackBase<0, null, undefined>

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

export type MachNapi = {}

export function machNapiNew(options: MachNapiOptions): MachNapi
export function machNapiBuild(mach: MachNapi, options: MachNapiBuildOptions, callback: Callback<[error: any, data: any]>): any
export function workerCallback(callback: any): any
