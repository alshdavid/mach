export type SourceLocation = {
  readonly filePath: string

  /** inclusive */
  readonly start: {
    readonly line: number
    readonly column: number
  }

  /** exclusive */
  readonly end: {
    readonly line: number
    readonly column: number
  }
}
