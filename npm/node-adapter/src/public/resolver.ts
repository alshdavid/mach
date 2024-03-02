export type Dependency = {
  id: string,
  specifier: string,
  specifier_type: string,
  is_entry: boolean,
  source_asset_id: string,
  source_path: string,
  resolve_from: string,
  imported_symbols: Array<string>,
}

export type ResolveOptions = {
  dependency: Dependency
}

export type ResolveResult = {
  file_path: string
}

export type ResolverInit = {
  resolve(options: ResolveOptions): (ResolveResult | undefined) | Promise<(ResolveResult | undefined)>
}

export class Resolver {
  init: ResolverInit

  constructor(init: ResolverInit) {
    this.init = init
  }
}
