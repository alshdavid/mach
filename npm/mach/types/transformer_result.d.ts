import type { DependencyOptions } from "./dependency_options.d.ts";
import type { EnvironmentOptions } from "./environment_options.d.ts";
import type { Environment } from "./environment.d.ts";
import type { BundleBehavior } from "./bundle_behavior.d.ts";
import type { Meta } from "./meta.d.ts";
import type { SourceLocation } from "./source_location.d.ts";

/**
 * Transformers can return multiple result objects to create new assets.
 * For example, a file may contain multiple parts of different types,
 * which should be processed by their respective transformation pipelines.
 *
 * @section transformer
 */
export type TransformerResult = {
  /** The asset's type. */
  readonly type: string;

  /** The content of the asset. Either content or an AST is required. */
  readonly content?: Blob | null | undefined;

  /** The asset's AST. Either content or an AST is required. */
  // readonly ast?: AST | null | undefined;

  /** The source map for the asset. */
  // readonly map?: SourceMap | null | undefined;
  readonly map?: any | null | undefined;

  /** The dependencies of the asset. */
  readonly dependencies?: ReadonlyArray<DependencyOptions>;

  /** The environment of the asset. The options are merged with the input asset's environment. */
  readonly env?: EnvironmentOptions | Environment;

  /**
   * Controls which bundle the asset is placed into.
   *   - inline: The asset will be placed into a new inline bundle. Inline bundles are not written
   *       to a separate file, but embedded into the parent bundle.
   *   - isolated: The asset will be isolated from its parents in a separate bundle. Shared assets
   *       will be duplicated.
   */
  readonly bundleBehavior?: BundleBehavior | null | undefined;

  /**
   * If the asset is used as a bundle entry, this controls whether that bundle can be split
   * into multiple, or whether all of the dependencies must be placed in a single bundle.
   */
  readonly isBundleSplittable?: boolean;

  /** Plugin-specific metadata for the asset. */
  readonly meta?: Meta;

  /** The pipeline defined in .parcelrc that the asset should be processed with. */
  readonly pipeline?: string | null | undefined;

  /**
   * Whether this asset can be omitted if none of its exports are being used.
   * This is initially set by the resolver, but can be overridden by transformers.
   */
  readonly sideEffects?: boolean;

  /** The symbols that the asset exports. */
  readonly symbols?: ReadonlyMap<Symbol, {
    local: Symbol;
    loc: SourceLocation | null | undefined;
  }>;

  /**
   * When a transformer returns multiple assets, it can give them unique keys to identify them.
   * This can be used to find assets during packaging, or to create dependencies between multiple
   * assets returned by a transformer by using the unique key as the dependency specifier.
   */
  readonly uniqueKey?: string | null | undefined;
};
