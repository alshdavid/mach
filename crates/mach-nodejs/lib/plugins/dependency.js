import * as types from '../types/index.js'

/**
 * @class
 * @implements {types.Dependency}
 */
export class Dependency {
  #internal

  constructor(/** @type {any} */ internal) {
    this.#internal = internal
  }

  get id() {
    return this.#internal.id
  }

  get specifier() {
    return this.#internal.specifier
  }

  get specifierType() {
    return this.#internal.specifier_type
  }

  get isEntry() {
    return this.#internal.is_entry
  }

  get priority() {
    return this.#internal.priority
  }

  get sourcePath() {
    return this.#internal.source_path
  }

  get sourceAsset() {
    return this.#internal.source_asset
  }

  get resolveFrom() {
    return this.#internal.resolve_from
  }

  get importedSymbols() {
    return this.#internal.imported_symbols
  }

  get bundleBehavior() {
    return this.#internal.bundle_behavior
  }

  get needsStableName() {
    throw new Error('Not implemented')
    return this.#internal.needs_stable_name
  }

  get isOptional() {
    throw new Error('Not implemented')
    return this.#internal.is_optional
  }

  get loc() {
    throw new Error('Not implemented')
    return this.#internal.loc
  }

  get env() {
    throw new Error('Not implemented')
    return this.#internal.env
  }

  get packageConditions() {
    throw new Error('Not implemented')
    return this.#internal.package_conditions
  }

  get meta() {
    throw new Error('Not implemented')
    return this.#internal.meta
  }

  get target() {
    throw new Error('Not implemented')
    return this.#internal.target
  }

  get sourceAssetId() {
    throw new Error('Not implemented')
    return this.#internal.source_asset_id
  }

  get sourceAssetType() {
    throw new Error('Not implemented')
    return this.#internal.source_asset_type
  }

  get range() {
    throw new Error('Not implemented')
    return this.#internal.range
  }

  get pipeline() {
    throw new Error('Not implemented')
    return this.#internal.pipeline
  }

  get symbols() {
    throw new Error('Not implemented')
    return this.#internal.symbols
  }
}
