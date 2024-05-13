import type { Meta } from "./meta.d.ts"
import type { SourceLocation } from "./source_location.d.ts"

/**
 * A map of export names to the corresponding asset's local variable names.
 */
export interface AssetSymbols extends Iterable<[Symbol, {
  local: Symbol;
  loc: SourceLocation | null | undefined;
  meta?: Meta | null | undefined;
}]> {
  /**
   * The exports of the asset are unknown, rather than just empty.
   * This is the default state.
   */
  readonly isCleared: boolean;

  get(exportSymbol: Symbol): {
    local: Symbol;
    loc: SourceLocation | null | undefined;
    meta?: Meta | null | undefined;
  } | null | undefined;

  hasExportSymbol(exportSymbol: Symbol): boolean;
  
  hasLocalSymbol(local: Symbol): boolean;
  
  exportSymbols(): Iterable<Symbol>;
}
