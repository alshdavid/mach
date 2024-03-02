export type DependencyOptions = {
  specifier: string,
  specifier_type: string,
  priority: string,
  resolve_from: string,
  imported_symbols: Array<string>,
}

export interface IMutableAsset {
  file_path: string
  get_code(): Promise<string>
  set_code(value: string): Promise<void>
  add_dependency(options: DependencyOptions): void
}

export type TransformOptions = {
  config: Record<string, string>,
  asset: IMutableAsset,
}

export type TransformerInit = {
  transform(options: TransformOptions): (IMutableAsset | null | undefined) | Promise<IMutableAsset | null | undefined>
}

export class Transformer {
  init: TransformerInit

  constructor(init: TransformerInit) {
    this.init = init
  }
}
