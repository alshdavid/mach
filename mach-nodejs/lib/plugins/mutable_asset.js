import { Readable } from 'stream';
import * as types from '../types/index.js'

/**
 * @class
 * @implements {types.MutableAsset}
 */
export class MutableAsset {
  #internal

  constructor(
    /** @type {any} */ internal
  ) {
    this.#internal = internal
  }

  /** @type {string} */
  get type() {
    return this.#internal.type
  }

  /** @type {types.BundleBehavior | null | undefined} */
  get bundleBehavior() {
    return this.#internal.bundle_behavior
  }

  /** @type {boolean} */
  get isBundleSplittable() {
    return this.#internal.is_bundle_splittable
  }

  /** @type {boolean} */
  get sideEffects() {
    return this.#internal.side_effects
  }

  /** @type {string | null | undefined} */
  get uniqueKey() {
    return this.#internal.unique_key
  }

  /** @type {types.MutableAssetSymbols} */
  get symbols() {
    return this.#internal.symbols
  }

  /** @type {unknown} */
  get setAST() {
    return this.#internal.set_ast
  }

  /** @type {unknown} */
  get isASTDirty() {
    return this.#internal.is_ast_dirty
  }

  /** @type {string} */
  get id() {
    return this.#internal.id
  }

  /** @type {FileSystem} */
  get fs() {
    return this.#internal.fs
  }

  /** @type {string} */
  get filePath() {
    return this.#internal.file_path
  }

  /** @type {URLSearchParams} */
  get query() {
    return this.#internal.query
  }

  /** @type {types.Environment} */
  get env() {
    return this.#internal.env
  }

  /** @type {boolean} */
  get isSource() {
    return this.#internal.is_source
  }

  /** @type {types.JSONObject} */
  get meta() {
    return this.#internal.meta
  }

  /** @type {unknown} */
  get astGenerator() {
    return this.#internal.ast_generator
  }

  /** @type {string | null | undefined} */
  get pipeline() {
    return this.#internal.pipeline
  }

  /** @type {unknown} */
  get getAST() {
    return this.#internal.get_ast
  }

  /** @type {unknown} */
  get invalidateOnFileCreate() {
    return this.#internal.invalidate_on_file_create
  }

  /** @return {Promise<unknown | null | undefined>} */
  getMap() {
    throw new Error('Method not implemented.');
  }

  /** @return {string} */
  addDependency(/** @type {types.DependencyOptions} */ arg0) {
    throw new Error('Method not implemented.');
  }

  /** @return {string} */
  addURLDependency(
    /** @type {string} */url,
    /** @type {Partial<types.DependencyOptions>} */ opts,
  ) {
    throw new Error('Method not implemented.');
  }

  invalidateOnFileChange(
    /** @type {string} */ arg0,
  ) {
    throw new Error('Method not implemented.');
  }

  invalidateOnEnvChange(
    /** @type {string} */ arg0,
  ) {
    throw new Error('Method not implemented.');
  }

  invalidateOnStartup() {
    throw new Error('Method not implemented.');
  }

  invalidateOnBuild() {
    throw new Error('Method not implemented.');
  }

  setCode(
    /** @type {string} */ arg0,
  ) {
    throw new Error('Method not implemented.');
  }

  setBuffer(
    /** @type {Buffer} */ arg0,
  ) {
    throw new Error('Method not implemented.');
  }

  setStream(
    /** @type {Readable} */ arg0
  ) {
    throw new Error('Method not implemented.');
  }

  setMap(
    /** @type {any} */ arg0,
  ) {
    throw new Error('Method not implemented.');
  }

  setEnvironment(
    /** @type {types.EnvironmentOptions} */ opts,
  ) {
    throw new Error('Method not implemented.');
  }

  /** @return {Promise<string>} */
  getCode() {
    throw new Error('Method not implemented.');
  }

  /** @return {Promise<Buffer>} */
  getBuffer() {
    throw new Error('Method not implemented.');
  }


  /** @return {Readable} */
  getStream() {
    throw new Error('Method not implemented.');
  }

  /** @return {Promise<Buffer | null | undefined>} */
  getMapBuffer() {
    throw new Error('Method not implemented.');
  }

  /** @return {ReadonlyArray<types.Dependency>} */
  getDependencies() {
    throw new Error('Method not implemented.');
  }
}