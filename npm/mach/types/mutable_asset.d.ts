import type { Readable } from 'node:stream'
import type { BaseAsset } from './base_asset.d.ts'
import type { BundleBehavior } from './bundle_behavior.d.ts'
import type { DependencyOptions } from './dependency_options.d.ts'
import type { FilePath } from './file_path.d.ts'
import type { MutableAssetSymbols } from './mutable_asset_symbols.d.ts'
import type { EnvironmentOptions } from './environment_options.d.ts'

export interface MutableAsset extends BaseAsset {
  /**
   * The asset's type. This initially corresponds to the source file extension,
   * but it may be changed during transformation.
   */
  type: string

  /**
   * Controls which bundle the asset is placed into.
   *   - inline: The asset will be placed into a new inline bundle. Inline bundles are not written
   *       to a separate file, but embedded into the parent bundle.
   *   - isolated: The asset will be isolated from its parents in a separate bundle. Shared assets
   *       will be duplicated.
   */
  bundleBehavior: BundleBehavior | null | undefined

  /**
   * If the asset is used as a bundle entry, this controls whether that bundle can be split
   * into multiple, or whether all of the dependencies must be placed in a single bundle.
   * @default true
   */
  isBundleSplittable: boolean

  /**
   * Whether this asset can be omitted if none of its exports are being used.
   * This is initially set by the resolver, but can be overridden by transformers.
   */
  sideEffects: boolean

  /**
   * When a transformer returns multiple assets, it can give them unique keys to identify them.
   * This can be used to find assets during packaging, or to create dependencies between multiple
   * assets returned by a transformer by using the unique key as the dependency specifier.
   */
  uniqueKey: string | null | undefined

  /** The symbols that the asset exports. */
  readonly symbols: MutableAssetSymbols

  /** Adds a dependency to the asset. */
  addDependency(arg0: DependencyOptions): string

  /**
   * Adds a url dependency to the asset.
   * This is a shortcut for addDependency that sets the specifierType to 'url' and priority to 'lazy'.
   */
  addURLDependency(url: string, opts: Partial<DependencyOptions>): string

  /** Invalidates the transformation when the given file is modified or deleted. */
  invalidateOnFileChange(arg0: FilePath): void

  /** Invalidates the transformation when matched files are created. */
  // invalidateOnFileCreate(arg0: FileCreateInvalidation): void;
  invalidateOnFileCreate: unknown

  /** Invalidates the transformation when the given environment variable changes. */
  invalidateOnEnvChange(arg0: string): void

  /** Invalidates the transformation only when Parcel restarts. */
  invalidateOnStartup(): void

  /** Invalidates the transformation on every build. */
  invalidateOnBuild(): void

  /** Sets the asset contents as a string. */
  setCode(arg0: string): void

  /** Sets the asset contents as a buffer. */
  setBuffer(arg0: Buffer): void

  /** Sets the asset contents as a stream. */
  setStream(arg0: Readable): void

  /** Sets the asset's AST. */
  // setAST(arg0: AST): void;
  setAST: unknown

  /** Returns whether the AST has been modified. */
  // isASTDirty(): boolean;
  isASTDirty: unknown

  /** Sets the asset's source map. */
  // setMap(arg0: SourceMap | null | undefined): void;
  setMap(arg0: any | null | undefined): void

  setEnvironment(opts: EnvironmentOptions): void
}
