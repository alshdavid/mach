import { Readable } from 'stream'
import * as types from '../types/index.js'

/**
 * @class
 * @implements {types.MutableAsset}
 */
export class MutableAsset {
  #internal
  #dependencies

  constructor(
    /** @type {Omit<types.TransformerTransformAction['TransformerTransform'], 'specifier'>} */ internal,
    /** @type {Array<any>} */ dependencies,
  ) {
    this.#internal = internal
    this.#dependencies = dependencies
  }

  /** @type {string} */
  get type() {
    return this.#internal.kind
  }

  set type(/** @type {string} */ value) {
    this.#internal.kind = value
  }

  /** @type {types.BundleBehavior | null | undefined} */
  get bundleBehavior() {
    throw new Error('Method not implemented.')
    // return this.#internal.bundle_behavior
  }

  /** @type {boolean} */
  get isBundleSplittable() {
    throw new Error('Method not implemented.')
    // return this.#internal.is_bundle_splittable
  }

  /** @type {boolean} */
  get sideEffects() {
    throw new Error('Method not implemented.')
    // return this.#internal.side_effects
  }

  /** @type {string | null | undefined} */
  get uniqueKey() {
    throw new Error('Method not implemented.')
    // return this.#internal.unique_key
  }

  /** @type {types.MutableAssetSymbols} */
  get symbols() {
    throw new Error('Method not implemented.')
    // return this.#internal.symbols
  }

  /** @type {unknown} */
  get setAST() {
    throw new Error('Method not implemented.')
    // return this.#internal.set_ast
  }

  /** @type {unknown} */
  get isASTDirty() {
    throw new Error('Method not implemented.')
    // return this.#internal.is_ast_dirty
  }

  /** @type {string} */
  get id() {
    throw new Error('Method not implemented.')
    // return this.#internal.id
  }

  /** @type {FileSystem} */
  get fs() {
    throw new Error('Method not implemented.')
    // return this.#internal.fs
  }

  /** @type {string} */
  get filePath() {
    return this.#internal.file_path
  }

  /** @type {URLSearchParams} */
  get query() {
    throw new Error('Method not implemented.')
    // return this.#internal.query
  }

  /** @type {types.Environment} */
  get env() {
    throw new Error('Method not implemented.')
    // return this.#internal.env
  }

  /** @type {boolean} */
  get isSource() {
    throw new Error('Method not implemented.')
    // return this.#internal.is_source
  }

  /** @type {types.JSONObject} */
  get meta() {
    throw new Error('Method not implemented.')
    // return this.#internal.meta
  }

  /** @type {unknown} */
  get astGenerator() {
    throw new Error('Method not implemented.')
    // return this.#internal.ast_generator
  }

  /** @type {string | null | undefined} */
  get pipeline() {
    throw new Error('Method not implemented.')
    // return this.#internal.pipeline
  }

  /** @type {unknown} */
  get getAST() {
    throw new Error('Method not implemented.')
    // return this.#internal.get_ast
  }

  /** @type {unknown} */
  get invalidateOnFileCreate() {
    throw new Error('Method not implemented.')
    // return this.#internal.invalidate_on_file_create
  }

  /** @return {Promise<unknown | null | undefined>} */
  getMap() {
    throw new Error('Method not implemented.')
  }

  /** @return {string} */
  addDependency(/** @type {types.DependencyOptions} */ arg0) {
    throw new Error('Method not implemented.')
  }

  /** @return {string} */
  addURLDependency(
    /** @type {string} */ url,
    /** @type {Partial<types.DependencyOptions>} */ opts,
  ) {
    throw new Error('Method not implemented.')
  }

  invalidateOnFileChange(/** @type {string} */ arg0) {
    throw new Error('Method not implemented.')
  }

  invalidateOnEnvChange(/** @type {string} */ arg0) {
    throw new Error('Method not implemented.')
  }

  invalidateOnStartup() {
    throw new Error('Method not implemented.')
  }

  invalidateOnBuild() {
    throw new Error('Method not implemented.')
  }

  setCode(/** @type {string} */ arg0) {
    this.#internal.content = Array.from(Buffer.from(arg0, 'utf-8'))
  }

  setBuffer(/** @type {Buffer} */ arg0) {
    throw new Error('Method not implemented.')
  }

  setStream(/** @type {Readable} */ arg0) {
    throw new Error('Method not implemented.')
  }

  setMap(/** @type {any} */ arg0) {
    throw new Error('Method not implemented.')
  }

  setEnvironment(/** @type {types.EnvironmentOptions} */ opts) {
    throw new Error('Method not implemented.')
  }

  /** @return {Promise<string>} */
  async getCode() {
    return (await this.getBuffer()).toString()
  }

  /** @return {Promise<Buffer>} */
  async getBuffer() {
    return Buffer.from(this.#internal.content)
  }

  /** @return {Readable} */
  getStream() {
    return Readable.from(this.#internal.content)
  }

  /** @return {Promise<Buffer | null | undefined>} */
  getMapBuffer() {
    throw new Error('Method not implemented.')
  }

  /** @return {ReadonlyArray<types.Dependency>} */
  getDependencies() {
    throw new Error('Method not implemented.')
  }
}
