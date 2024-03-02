import { DependencyOptions, IMutableAsset, Transformer } from '../public/transformer'

export type RequestRunTransformer = {
  plugin_key: string,
  config: Record<string, string>,
  file_path: string,
  kind: string,
  code: string,
}

export async function run_transformer(transformer: Transformer, { plugin_key, config, file_path, kind, code }: RequestRunTransformer) {
  let updated = false
  const dependencies: DependencyOptions[]  = []
  const asset = new class MutableAsset implements IMutableAsset {
    file_path = file_path;
    get kind() {
      return kind
    }

    set kind(value) {
      updated = true
      kind = value
    }

    async get_code() {
      return code
    }
    async set_code(value: string) {
      updated = true
      code = value
    }
    add_dependency(options: DependencyOptions) {
      updated = true
      dependencies.push(options)
    }
  }()
  const result = await transformer.init.transform({ config, asset })
  if (!updated) {
    return {
      updated: false,
      dependencies: [],
      code: ''
    }
  }
  return {
    updated: true,
    dependencies,
    code,
  }
}