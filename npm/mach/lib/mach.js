import * as types from '../types/index.js'

/**
 * @class
 * @implements {types.Mach}
 */
export class Mach {
  build(options) {
    throw new Error('Method not implemented.');
  }
  dev(options) {
    throw new Error('Method not implemented.');
  }
  watch(options) {
    throw new Error('Method not implemented.');
  }
  serve(options) {
    throw new Error('Method not implemented.');
  }
  subscribe(type, callback);
  subscribe(type, callback);
  subscribe(type, callback);
  subscribe(type: unknown, callback: unknown): types.DisposeFunc {
    throw new Error('Method not implemented.');
  }
}