import type { DependencyPriority } from './dependency_priority.d.ts'
import type { FilePath } from './file_path.d.ts'
import type { JSONObject } from './json_object.d.ts'

export type ResolveResult = {
  /** An absolute path to the resolved file. */
  readonly filePath?: FilePath

  /** An optional named pipeline to use to compile the resolved file. */
  readonly pipeline?: string | null | undefined

  /** Query parameters to be used by transformers when compiling the resolved file. */
  readonly query?: URLSearchParams

  /** Whether the resolved file should be excluded from the build. */
  readonly isExcluded?: boolean

  /** Overrides the priority set on the dependency. */
  readonly priority?: DependencyPriority

  /** Corresponds to BaseAsset's <code>sideEffects</code>. */
  readonly sideEffects?: boolean

  /** The code of the resolved asset. If provided, this is used rather than reading the file from disk. */
  readonly code?: string

  /** Whether this dependency can be deferred by Parcel itself (true by default). */
  readonly canDefer?: boolean

  /** A resolver might return diagnostics to also run subsequent resolvers while still providing a reason why it failed. */
  // readonly diagnostics?: Diagnostic | Array<Diagnostic>

  /** Is spread (shallowly merged) onto the request's dependency.meta */
  readonly meta?: JSONObject

  /** A list of file paths or patterns that should invalidate the resolution if created. */
  // readonly invalidateOnFileCreate?: Array<FileCreateInvalidation>

  /** A list of files that should invalidate the resolution if modified or deleted. */
  readonly invalidateOnFileChange?: Array<FilePath>

  /** Invalidates the resolution when the given environment variable changes.*/
  readonly invalidateOnEnvChange?: Array<string>
}
