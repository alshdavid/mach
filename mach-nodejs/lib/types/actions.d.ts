export type Action = (
  PingAction |
  ResolverRegisterAction |
  ResolverRunAction |
  TransformerRegisterAction |
  TransformerRunAction
)

export type PingAction = {
  Ping: {}
}

export type ResolverRegisterAction = {
  ResolverRegister: {
    specifier: string
  }
}

export type ResolverRunAction = {
  ResolverRun: {
    specifier: string
    dependency: any
  }
}

export type TransformerRegisterAction = {
  TransformerRegister: {
    specifier: string
  }
}

export type TransformerRunAction = {
  TransformerRun: {
    specifier: string
    mutable_asset: any
  }
}
