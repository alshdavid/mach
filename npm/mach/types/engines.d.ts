import type { SemverRange } from './semver_range.d.ts'

export type Engines = {
  readonly browsers?: string | Array<string>
  readonly electron?: SemverRange
  readonly node?: SemverRange
  readonly parcel?: SemverRange
}
