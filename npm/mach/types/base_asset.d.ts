import type { Readable } from 'node:stream'
import type { AssetSymbols } from "./asset_symbols.d.ts"
import type { BundleBehavior } from "./bundle_behavior.d.ts"
import type { Environment } from "./environment.d.ts"
import type { FilePath } from "./file_path.d.ts"
import type { Meta } from "./meta.d.ts"
import type { Dependency } from './dependency.d.ts'

export interface BaseAsset {
  /** The id of the asset. */
  readonly id: string;

  /** The file system where the source is located. */
  readonly fs: FileSystem;

  /** The file path of the asset. */
  readonly filePath: FilePath;

  /**
   * The asset's type. This initially corresponds to the source file extension,
   * but it may be changed during transformation.
   */
  readonly type: string;

  /** The transformer options for the asset from the dependency query string. */
  readonly query: URLSearchParams;

  /** The environment of the asset. */
  readonly env: Environment;

  /**
   * Whether this asset is part of the project, and not an external dependency (e.g. in node_modules).
   * This indicates that transformation using the project's configuration should be applied.
   */
  readonly isSource: boolean;

  /** Plugin-specific metadata for the asset. */
  readonly meta: Meta;

  /**
   * Controls which bundle the asset is placed into.
   *   - inline: The asset will be placed into a new inline bundle. Inline bundles are not written
   *       to a separate file, but embedded into the parent bundle.
   *   - isolated: The asset will be isolated from its parents in a separate bundle. Shared assets
   *       will be duplicated.
   */
  readonly bundleBehavior: BundleBehavior | null | undefined;

  /**
   * If the asset is used as a bundle entry, this controls whether that bundle can be split
   * into multiple, or whether all of the dependencies must be placed in a single bundle.
   */
  readonly isBundleSplittable: boolean;

  /**
   * Whether this asset can be omitted if none of its exports are being used.
   * This is initially set by the resolver, but can be overridden by transformers.
   */
  readonly sideEffects: boolean;

  /**
   * When a transformer returns multiple assets, it can give them unique keys to identify them.
   * This can be used to find assets during packaging, or to create dependencies between multiple
   * assets returned by a transformer by using the unique key as the dependency specifier.
   */
  readonly uniqueKey: string | null | undefined;

  /** @todo */
  readonly astGenerator: unknown;

  /** The pipeline defined in .parcelrc that the asset should be processed with. */
  readonly pipeline: string | null | undefined;

  /** The symbols that the asset exports. */
  readonly symbols: AssetSymbols;

  /** Returns the current AST. */
  getAST: unknown;

  /** Returns the asset contents as a string. */
  getCode(): Promise<string>;

  /** Returns the asset contents as a buffer. */
  getBuffer(): Promise<Buffer>;

  /** Returns the asset contents as a stream. */
  getStream(): Readable;

  /** Returns the source map for the asset, if available. */
  /** @todo */
  getMap: unknown
  // getMap(): Promise<SourceMap | null | undefined>;

  /** Returns a buffer representation of the source map, if available. */
  getMapBuffer(): Promise<Buffer | null | undefined>;

  /** Returns a list of dependencies for the asset. */
  getDependencies(): ReadonlyArray<Dependency>;
}
