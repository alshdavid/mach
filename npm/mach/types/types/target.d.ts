import type { Environment } from "./environment.d.ts";
import type { FilePath } from "./file_path.d.ts";
import type { SourceLocation } from "./source_location.d.ts";

/**
 * A parsed version of PackageTargetDescriptor
 */
export interface Target {
  /** The output filename of the entry */
  readonly distEntry: FilePath | null | undefined;

  /** The output folder */
  readonly distDir: FilePath;
  readonly env: Environment;
  readonly name: string;
  readonly publicUrl: string;

  /** The location that created this Target, e.g. `package.json#main`*/
  readonly loc: SourceLocation | null | undefined;
}