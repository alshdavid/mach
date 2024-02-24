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
  constructor(init: ResolverInit)
}

export type DependencyOptions = {
  specifier: string,
  specifier_type: string,
  priority: string,
  resolve_from: string,
  imported_symbols: Array<string>,
}

export class MutableAsset {
  file_path: string
  get_code(): Promise<string>
  set_code(value: string): Promise<void>
  add_dependency(options: DependencyOptions): void
}

export type TransformOptions = {
  asset: MutableAsset
}

export type TransformerInit = {
  transform(options: TransformOptions): (MutableAsset | null | undefined) | Promise<MutableAsset | null | undefined>
}

export class Transformer {
  init: TransformerInit
  constructor(init: TransformerInit)
}
