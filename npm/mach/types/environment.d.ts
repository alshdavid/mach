import type { Engines } from "./engines.d.ts"
import type { EnvironmentContext } from "./environment_context.d.ts"
import type { EnvironmentFeature } from "./environment_feature.d.ts"
import type { OutputFormat } from "./output_format.d.ts"
import type { PackageName } from "./package_name.d.ts"
import type { SourceLocation } from "./source_location.d.ts"
import type { SourceType } from "./source_type.d.ts"
import type { TargetSourceMapOptions } from "./target_source_map_options.d.ts"
import type { VersionMap } from "./version_map.d.ts"

export interface Environment {
  readonly id: string
  readonly context: EnvironmentContext
  readonly engines: Engines

  /** Whether to include all/none packages \
   *  (<code>true / false</code>), an array of package names to include, or an object \
   *  (of a package is not specified, it's included).
   */
  readonly includeNodeModules:
    | boolean
    | Array<PackageName>
    | Record<PackageName, boolean>
  readonly outputFormat: OutputFormat
  readonly sourceType: SourceType

  /** Whether this is a library build (e.g. less loaders) */
  readonly isLibrary: boolean

  /** Whether the output should be minified. */
  readonly shouldOptimize: boolean

  /** Whether scope hoisting is enabled. */
  readonly shouldScopeHoist: boolean
  readonly sourceMap: TargetSourceMapOptions | null | undefined
  readonly loc: SourceLocation | null | undefined

  /** Whether <code>context</code> specifies a browser context. */
  isBrowser(): boolean

  /** Whether <code>context</code> specifies a node context. */
  isNode(): boolean

  /** Whether <code>context</code> specifies an electron context. */
  isElectron(): boolean

  /** Whether <code>context</code> specifies a worker context. */
  isWorker(): boolean

  /** Whether <code>context</code> specifies a worklet context. */
  isWorklet(): boolean

  /** Whether <code>context</code> specifies an isolated context (can't access other loaded ancestor bundles). */
  isIsolated(): boolean
  matchesEngines(minVersions: VersionMap, defaultValue?: boolean): boolean
  supports(feature: EnvironmentFeature, defaultValue?: boolean): boolean
}
