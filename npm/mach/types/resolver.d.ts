import type { Dependency } from './dependency.d.ts'
import type { FilePath } from './file_path.d.ts'
import type { ResolveResult } from './resolve_result.d.ts'

export type ResolverInitOpts<ConfigType> = {
  loadConfig?(options: {
    config: any //Config
    options: any //PluginOptions
    logger: any //PluginLogger
  }): ConfigType | Promise<ConfigType>

  resolve: (options: {
    dependency: Dependency
    options: any // PluginOptions
    logger: any // PluginLogger
    specifier: FilePath
    pipeline: string | null | undefined
    config: ConfigType
  }) => undefined | null | ResolveResult | Promise<ResolveResult>
}

export interface IResolver<ConfigType> extends Omit<Resolver<ConfigType>, 'constructor'> {}

export declare class Resolver<ConfigType> {
  constructor(opts: ResolverInitOpts<ConfigType>)
  triggerLoadConfig: ResolverInitOpts<ConfigType>['loadConfig']
  triggerResolve: ResolverInitOpts<ConfigType>['resolve']
}
