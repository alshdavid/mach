import type { MutableAsset } from './mutable_asset.d.ts'
import type { TransformerResult } from './transformer_result.d.ts'

export type TransformerInitOpts<ConfigType> = {
  loadConfig?(options: {
    config: any // Config;
    options: any // PluginOptions;
    logger: any // PluginLogger;
    tracer: any // PluginTracer;
  }): void

  transform(options: {
    asset: MutableAsset
    config: ConfigType
    resolve: any // ResolveFn
    options: any // PluginOptions
    logger: any // PluginLogger
    tracer: any // PluginTracer
  }):
    | Promise<Array<TransformerResult | MutableAsset>>
    | Array<TransformerResult | MutableAsset>

  /** @deprecated Not supported by Mach */
  canReuseAST?: unknown
  /** @deprecated Not supported by Mach */
  parse?: unknown
  /** @deprecated Not supported by Mach */
  postProcess?: unknown
  /** @deprecated Not supported by Mach */
  generate?: unknown
}

export interface ITransformer<ConfigType>
  extends Omit<Transformer<ConfigType>, 'constructor'> {}
export declare class Transformer<ConfigType> {
  constructor(opts: TransformerInitOpts<ConfigType>)
  triggerLoadConfig: TransformerInitOpts<ConfigType>['loadConfig']
  triggerTransform: TransformerInitOpts<ConfigType>['transform']
}
