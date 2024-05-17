export type Action =
  | PingAction
  | ResolverRegisterAction
  | ResolverLoadConfigAction
  | ResolverResolveAction
  | TransformerRegisterAction
  | TransformerLoadConfigAction
  | TransformerTransformAction

export type PingAction = [0, {
  Ping: {}
}]

export type ResolverRegisterAction = [1, {
  ResolverRegister: {
    specifier: string
  }
}]

export type ResolverLoadConfigAction = [2, {
  ResolverLoadConfig: {
    specifier: string
  }
}]

export type ResolverResolveAction = [3, {
  ResolverResolve: {
    specifier: string
    dependency: any
  }
}]

export type TransformerRegisterAction = [4, {
  TransformerRegister: {
    specifier: string
  }
}]

export type TransformerLoadConfigAction = [5, {
  TransformerLoadConfig: {
    specifier: string
  }
}]

export type TransformerTransformAction = [6, {
  TransformerTransform: {
    specifier: string
    file_path: string
    kind: string
    content: Array<any>
  }
}]
