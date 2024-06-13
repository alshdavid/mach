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

export type RpcCallbackWorker =
  // Ping
  | RpcCallbackBase<0, null, undefined>

export type MachNapiOptions = {
  nodeWorkers?: number
  threads?: number
}

export type MachNapiBuildOptions = {
  entries?: string[]
  projectRoot?: string
  outFolder?: string
  clean?: boolean
  optimize?: boolean
  bundleSplitting?: boolean
}

export type MachNapi = {
  nodeWorkerCount: number
}

export function machNapiNew(options: MachNapiOptions, callback: any): MachNapi
export function machNapiBuild(mach: MachNapi, options: MachNapiBuildOptions, callback: Callback<[error: any, data: any]>): any
export function workerCallback(callback: any): any
