import type { AssetSymbols } from "./asset_symbols.d.ts"
import type { Meta } from "./meta.d.ts"
import type { SourceLocation } from "./source_location.d.ts"

export interface MutableAssetSymbols extends AssetSymbols {
  /**
   * Initilizes the map, sets isCleared to false.
   */
  ensure(): void;
  set(exportSymbol: Symbol, local: Symbol, loc: SourceLocation | null | undefined, meta?: Meta | null | undefined): void;
  delete(exportSymbol: Symbol): void;
}
