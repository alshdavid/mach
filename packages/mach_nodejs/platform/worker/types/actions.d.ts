export type Action =
  | RequestContext<0, { Ping: PingAction }>
  | RequestContext<1, { ResolverRegister: ResolverRegisterAction }>
  | RequestContext<2, { ResolverLoadConfig: ResolverLoadConfigAction }>
  | RequestContext<3, { ResolverResolve: ResolverResolveAction }>
  | RequestContext<4, { TransformerRegister: TransformerRegisterAction }>
  | RequestContext<5, { TransformerLoadConfig: TransformerLoadConfigAction }>
  | RequestContext<6, { TransformerTransform: TransformerTransformAction }>

export type RequestContext<T extends number, U> = [T, U]

export type PingAction = {}

export type ResolverRegisterAction = {
  specifier: string
}

export type ResolverLoadConfigAction = {
  specifier: string
}

export type ResolverResolveAction = {
  specifier: string
  dependency: any
}

export type TransformerRegisterAction = {
  specifier: string
}

export type TransformerLoadConfigAction = {
  specifier: string
}

export type TransformerTransformAction = {
  specifier: string
  file_path: string
  kind: string
  content: Array<any>
}
