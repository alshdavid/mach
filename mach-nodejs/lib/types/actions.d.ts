export type Action =
  | PingAction
  | ResolverRegisterAction
  | ResolverLoadConfigAction
  | ResolverResolveAction
  | TransformerRegisterAction
  | TransformerLoadConfigAction
  | TransformerTransformAction

export type PingAction = {
  Ping: {}
}

export type ResolverRegisterAction = {
  ResolverRegister: {
    specifier: string
  }
}

export type ResolverLoadConfigAction = {
  ResolverLoadConfig: {
    specifier: string
  }
}

export type ResolverResolveAction = {
  ResolverResolve: {
    specifier: string
    dependency: any
  }
}

export type TransformerRegisterAction = {
  TransformerRegister: {
    specifier: string
  }
}

export type TransformerLoadConfigAction = {
  TransformerLoadConfig: {
    specifier: string
  }
}

export type TransformerTransformAction = {
  TransformerTransform: {
    specifier: string
    file_path: string
    kind: string
    content: Array<any>
  }
}
