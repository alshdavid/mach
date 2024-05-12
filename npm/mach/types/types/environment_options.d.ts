import type { Engines } from "./engines.d.ts"
import type { EnvironmentContext } from "./environment_context.d.ts"
import type { OutputFormat } from "./output_format.d.ts"
import type { PackageName } from "./package_name.d.ts"
import type { SourceLocation } from "./source_location.d.ts"
import type { SourceType } from "./source_type.js"
import type { TargetSourceMapOptions } from "./target_source_map_options.d.ts"

export type EnvironmentOptions = {
  readonly context?: EnvironmentContext
  readonly engines?: Engines
  readonly includeNodeModules?:
    | boolean
    | Array<PackageName>
    | Record<PackageName, boolean>
  readonly outputFormat?: OutputFormat
  readonly sourceType?: SourceType
  readonly isLibrary?: boolean
  readonly shouldOptimize?: boolean
  readonly shouldScopeHoist?: boolean
  readonly sourceMap?: TargetSourceMapOptions | null | undefined
  readonly loc?: SourceLocation | null | undefined
}
