import type { Meta } from './meta.d.ts'
import type { SourceLocation } from './source_location.d.ts'

export interface MutableDependencySymbols
  extends Iterable<
    [
      Symbol,
      {
        local: Symbol
        loc: SourceLocation | null | undefined
        isWeak: boolean
        meta?: Meta | null | undefined
      },
    ]
  > {
  /**
   * Initilizes the map, sets isCleared to false.
   */
  ensure(): void

  /**
   * The symbols taht are imports are unknown, rather than just empty.
   * This is the default state.
   */
  readonly isCleared: boolean
  get(exportSymbol: Symbol):
    | {
        local: Symbol
        loc: SourceLocation | null | undefined
        isWeak: boolean
        meta?: Meta | null | undefined
      }
    | null
    | undefined
  hasExportSymbol(exportSymbol: Symbol): boolean
  hasLocalSymbol(local: Symbol): boolean
  exportSymbols(): Iterable<Symbol>
  set(
    exportSymbol: Symbol,
    local: Symbol,
    loc: SourceLocation | null | undefined,
    isWeak: boolean | null | undefined,
  ): void
  delete(exportSymbol: Symbol): void
}
