import type { BundleBehavior } from './bundle_behavior.d.ts'
import type { DependencyPriority } from './dependency_priority.d.ts'
import type { DependencySpecifier } from './dependency_specifier.d.ts'
import type { EnvironmentOptions } from './environment_options.d.ts'
import type { FilePath } from './file_path.d.ts'
import type { Meta } from './meta.d.ts'
import type { SemverRange } from './semver_range.d.ts'
import type { SourceLocation } from './source_location.d.ts'
import type { SpecifierType } from './specifier_type.d.ts'

export type DependencyOptions = {
  /** The specifier used to resolve the dependency. */
  readonly specifier: DependencySpecifier

  /**
   * How the specifier should be interpreted.
   *   - esm: An ES module specifier. It is parsed as a URL, but bare specifiers are treated as node_modules.
   *   - commonjs: A CommonJS specifier. It is not parsed as a URL.
   *   - url: A URL that works as in a browser. Bare specifiers are treated as relative URLs.
   *   - custom: A custom specifier. Must be handled by a custom resolver plugin.
   */
  readonly specifierType: SpecifierType

  /**
   * When the dependency should be loaded.
   *   - sync: The dependency should be resolvable synchronously. The resolved asset will be placed
   *       in the same bundle as the parent, or another bundle that's already on the page.
   *   - parallel: The dependency should be placed in a separate bundle that's loaded in parallel
   *       with the current bundle.
   *   - lazy: The dependency should be placed in a separate bundle that's loaded later.
   * @default 'sync'
   */
  readonly priority?: DependencyPriority

  /**
   * Controls the behavior of the bundle the resolved asset is placed into. Use in combination with `priority`
   * to determine when the bundle is loaded.
   *   - inline: The resolved asset will be placed into a new inline bundle. Inline bundles are not written
   *       to a separate file, but embedded into the parent bundle.
   *   - isolated: The resolved asset will be isolated from its parents in a separate bundle.
   *       Shared assets will be duplicated.
   */
  readonly bundleBehavior?: BundleBehavior

  /**
   * When the dependency is a bundle entry (priority is "parallel" or "lazy"), this controls the naming
   * of that bundle. `needsStableName` indicates that the name should be stable over time, even when the
   * content of the bundle changes. This is useful for entries that a user would manually enter the URL
   * for, as well as for things like service workers or RSS feeds, where the URL must remain consistent
   * over time.
   */
  readonly needsStableName?: boolean

  /** Whether the dependency is optional. If the dependency cannot be resolved, this will not fail the build. */
  readonly isOptional?: boolean

  /** The location within the source file where the dependency was found. */
  readonly loc?: SourceLocation

  /** The environment of the dependency. */
  readonly env?: EnvironmentOptions

  /**
   * A list of custom conditions to use when resolving package.json "exports" and "imports".
   * This is combined with the conditions from the environment. However, it overrides the
   * default "import" and "require" conditions inferred from the specifierType. To include those
   * in addition to custom conditions, explicitly add them to this list.
   */
  readonly packageConditions?: Array<string>

  /** Plugin-specific metadata for the dependency. */
  readonly meta?: Meta

  /** The pipeline defined in .parcelrc that the dependency should be processed with. */
  readonly pipeline?: string

  /**
   * The file path where the dependency should be resolved from.
   * By default, this is the path of the source file where the dependency was specified.
   */
  readonly resolveFrom?: FilePath

  /** The semver version range expected for the dependency. */
  readonly range?: SemverRange

  /** The symbols within the resolved module that the source file depends on. */
  readonly symbols?: ReadonlyMap<
    Symbol,
    {
      local: Symbol
      loc: SourceLocation | null | undefined
      isWeak: boolean
      meta?: Meta
    }
  >
}
