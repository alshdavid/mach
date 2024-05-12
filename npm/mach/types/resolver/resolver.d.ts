import type { Dependency, FilePath } from '../types/index.d.ts'
import type { ResolveResult } from './resolve_result.d.ts'

export type ResolverInitOpts<ConfigType> = {
  loadConfig?(options: ResolverResolveOptions): ConfigType | Promise<ConfigType>
  resolve: (options: ResolverLoadConfigOptions<ConfigType>) => ResolveResult | Promise<ResolveResult>
}

export interface IResolver<ConfigType> extends Omit<Resolver<ConfigType>, 'new'> {}

export declare class Resolver<ConfigType> {
  constructor(opts: ResolverInitOpts<ConfigType>)
  triggerLoadConfig?(options: ResolverResolveOptions): ConfigType | Promise<ConfigType>
  triggerResolve(options: ResolverLoadConfigOptions<ConfigType>): ResolveResult | Promise<ResolveResult>
}

export type ResolverResolveOptions = {
  config: any //Config
  options: any //PluginOptions
  logger: any //PluginLogger
}

export type ResolverLoadConfigOptions<ConfigType> = {
  dependency: Dependency
  options: any // PluginOptions
  logger: any // PluginLogger
  specifier: FilePath
  pipeline: string | null | undefined
  config: ConfigType
}
