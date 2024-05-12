export type Action = (
  PingAction |
  ResolverRegisterAction |
  ResolverRunAction
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
