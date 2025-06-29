// Vendored https://www.npmjs.com/package/tar

var __create = Object.create;
var __defProp = Object.defineProperty;
var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
var __getOwnPropNames = Object.getOwnPropertyNames;
var __getProtoOf = Object.getPrototypeOf;
var __hasOwnProp = Object.prototype.hasOwnProperty;
var __export = (target, all) => {
  for (var name2 in all)
    __defProp(target, name2, { get: all[name2], enumerable: true });
};
var __copyProps = (to, from, except, desc2) => {
  if ((from && typeof from === "object") || typeof from === "function") {
    for (let key of __getOwnPropNames(from))
      if (!__hasOwnProp.call(to, key) && key !== except)
        __defProp(to, key, {
          get: () => from[key],
          enumerable:
            !(desc2 = __getOwnPropDesc(from, key)) || desc2.enumerable,
        });
  }
  return to;
};
var __toESM = (mod, isNodeMode, target) => (
  (target = mod != null ? __create(__getProtoOf(mod)) : {}),
  __copyProps(
    // If the importer is in node compatibility mode or this is not an ESM
    // file that has been converted to a CommonJS file using a Babel-
    // compatible transform (i.e. "__esModule" has not been set), then set
    // "default" to the CommonJS "module.exports" for node compatibility.
    isNodeMode || !mod || !mod.__esModule
      ? __defProp(target, "default", { value: mod, enumerable: true })
      : target,
    mod
  )
);
var __toCommonJS = (mod) =>
  __copyProps(__defProp({}, "__esModule", { value: true }), mod);

// tar.mjs
var tar_exports = {};
__export(tar_exports, {
  Header: () => Header,
  Pack: () => Pack,
  PackJob: () => PackJob,
  PackSync: () => PackSync,
  Parser: () => Parser,
  Pax: () => Pax,
  ReadEntry: () => ReadEntry,
  Unpack: () => Unpack,
  UnpackSync: () => UnpackSync,
  WriteEntry: () => WriteEntry,
  WriteEntrySync: () => WriteEntrySync,
  WriteEntryTar: () => WriteEntryTar,
  c: () => create,
  create: () => create,
  extract: () => extract,
  filesFilter: () => filesFilter,
  list: () => list,
  r: () => replace,
  replace: () => replace,
  t: () => list,
  types: () => types_exports,
  u: () => update,
  update: () => update,
  x: () => extract,
});
module.exports = __toCommonJS(tar_exports);

// node_modules/@isaacs/fs-minipass/dist/esm/index.js
var import_events = __toESM(require("events"), 1);
var import_fs = __toESM(require("fs"), 1);

// node_modules/minipass/dist/esm/index.js
var import_node_events = require("node:events");
var import_node_stream = __toESM(require("node:stream"), 1);
var import_node_string_decoder = require("node:string_decoder");
var proc =
  typeof process === "object" && process
    ? process
    : {
        stdout: null,
        stderr: null,
      };
var isStream = (s) =>
  !!s &&
  typeof s === "object" &&
  (s instanceof Minipass ||
    s instanceof import_node_stream.default ||
    isReadable(s) ||
    isWritable(s));
var isReadable = (s) =>
  !!s &&
  typeof s === "object" &&
  s instanceof import_node_events.EventEmitter &&
  typeof s.pipe === "function" && // node core Writable streams have a pipe() method, but it throws
  s.pipe !== import_node_stream.default.Writable.prototype.pipe;
var isWritable = (s) =>
  !!s &&
  typeof s === "object" &&
  s instanceof import_node_events.EventEmitter &&
  typeof s.write === "function" &&
  typeof s.end === "function";
var EOF = Symbol("EOF");
var MAYBE_EMIT_END = Symbol("maybeEmitEnd");
var EMITTED_END = Symbol("emittedEnd");
var EMITTING_END = Symbol("emittingEnd");
var EMITTED_ERROR = Symbol("emittedError");
var CLOSED = Symbol("closed");
var READ = Symbol("read");
var FLUSH = Symbol("flush");
var FLUSHCHUNK = Symbol("flushChunk");
var ENCODING = Symbol("encoding");
var DECODER = Symbol("decoder");
var FLOWING = Symbol("flowing");
var PAUSED = Symbol("paused");
var RESUME = Symbol("resume");
var BUFFER = Symbol("buffer");
var PIPES = Symbol("pipes");
var BUFFERLENGTH = Symbol("bufferLength");
var BUFFERPUSH = Symbol("bufferPush");
var BUFFERSHIFT = Symbol("bufferShift");
var OBJECTMODE = Symbol("objectMode");
var DESTROYED = Symbol("destroyed");
var ERROR = Symbol("error");
var EMITDATA = Symbol("emitData");
var EMITEND = Symbol("emitEnd");
var EMITEND2 = Symbol("emitEnd2");
var ASYNC = Symbol("async");
var ABORT = Symbol("abort");
var ABORTED = Symbol("aborted");
var SIGNAL = Symbol("signal");
var DATALISTENERS = Symbol("dataListeners");
var DISCARDED = Symbol("discarded");
var defer = (fn) => Promise.resolve().then(fn);
var nodefer = (fn) => fn();
var isEndish = (ev) => ev === "end" || ev === "finish" || ev === "prefinish";
var isArrayBufferLike = (b) =>
  b instanceof ArrayBuffer ||
  (!!b &&
    typeof b === "object" &&
    b.constructor &&
    b.constructor.name === "ArrayBuffer" &&
    b.byteLength >= 0);
var isArrayBufferView = (b) => !Buffer.isBuffer(b) && ArrayBuffer.isView(b);
var Pipe = class {
  src;
  dest;
  opts;
  ondrain;
  constructor(src, dest, opts) {
    this.src = src;
    this.dest = dest;
    this.opts = opts;
    this.ondrain = () => src[RESUME]();
    this.dest.on("drain", this.ondrain);
  }
  unpipe() {
    this.dest.removeListener("drain", this.ondrain);
  }
  // only here for the prototype
  /* c8 ignore start */
  proxyErrors(_er) {}
  /* c8 ignore stop */
  end() {
    this.unpipe();
    if (this.opts.end) this.dest.end();
  }
};
var PipeProxyErrors = class extends Pipe {
  unpipe() {
    this.src.removeListener("error", this.proxyErrors);
    super.unpipe();
  }
  constructor(src, dest, opts) {
    super(src, dest, opts);
    this.proxyErrors = (er) => dest.emit("error", er);
    src.on("error", this.proxyErrors);
  }
};
var isObjectModeOptions = (o) => !!o.objectMode;
var isEncodingOptions = (o) =>
  !o.objectMode && !!o.encoding && o.encoding !== "buffer";
var Minipass = class extends import_node_events.EventEmitter {
  [FLOWING] = false;
  [PAUSED] = false;
  [PIPES] = [];
  [BUFFER] = [];
  [OBJECTMODE];
  [ENCODING];
  [ASYNC];
  [DECODER];
  [EOF] = false;
  [EMITTED_END] = false;
  [EMITTING_END] = false;
  [CLOSED] = false;
  [EMITTED_ERROR] = null;
  [BUFFERLENGTH] = 0;
  [DESTROYED] = false;
  [SIGNAL];
  [ABORTED] = false;
  [DATALISTENERS] = 0;
  [DISCARDED] = false;
  /**
   * true if the stream can be written
   */
  writable = true;
  /**
   * true if the stream can be read
   */
  readable = true;
  /**
   * If `RType` is Buffer, then options do not need to be provided.
   * Otherwise, an options object must be provided to specify either
   * {@link Minipass.SharedOptions.objectMode} or
   * {@link Minipass.SharedOptions.encoding}, as appropriate.
   */
  constructor(...args) {
    const options = args[0] || {};
    super();
    if (options.objectMode && typeof options.encoding === "string") {
      throw new TypeError("Encoding and objectMode may not be used together");
    }
    if (isObjectModeOptions(options)) {
      this[OBJECTMODE] = true;
      this[ENCODING] = null;
    } else if (isEncodingOptions(options)) {
      this[ENCODING] = options.encoding;
      this[OBJECTMODE] = false;
    } else {
      this[OBJECTMODE] = false;
      this[ENCODING] = null;
    }
    this[ASYNC] = !!options.async;
    this[DECODER] = this[ENCODING]
      ? new import_node_string_decoder.StringDecoder(this[ENCODING])
      : null;
    if (options && options.debugExposeBuffer === true) {
      Object.defineProperty(this, "buffer", { get: () => this[BUFFER] });
    }
    if (options && options.debugExposePipes === true) {
      Object.defineProperty(this, "pipes", { get: () => this[PIPES] });
    }
    const { signal } = options;
    if (signal) {
      this[SIGNAL] = signal;
      if (signal.aborted) {
        this[ABORT]();
      } else {
        signal.addEventListener("abort", () => this[ABORT]());
      }
    }
  }
  /**
   * The amount of data stored in the buffer waiting to be read.
   *
   * For Buffer strings, this will be the total byte length.
   * For string encoding streams, this will be the string character length,
   * according to JavaScript's `string.length` logic.
   * For objectMode streams, this is a count of the items waiting to be
   * emitted.
   */
  get bufferLength() {
    return this[BUFFERLENGTH];
  }
  /**
   * The `BufferEncoding` currently in use, or `null`
   */
  get encoding() {
    return this[ENCODING];
  }
  /**
   * @deprecated - This is a read only property
   */
  set encoding(_enc) {
    throw new Error("Encoding must be set at instantiation time");
  }
  /**
   * @deprecated - Encoding may only be set at instantiation time
   */
  setEncoding(_enc) {
    throw new Error("Encoding must be set at instantiation time");
  }
  /**
   * True if this is an objectMode stream
   */
  get objectMode() {
    return this[OBJECTMODE];
  }
  /**
   * @deprecated - This is a read-only property
   */
  set objectMode(_om) {
    throw new Error("objectMode must be set at instantiation time");
  }
  /**
   * true if this is an async stream
   */
  get ["async"]() {
    return this[ASYNC];
  }
  /**
   * Set to true to make this stream async.
   *
   * Once set, it cannot be unset, as this would potentially cause incorrect
   * behavior.  Ie, a sync stream can be made async, but an async stream
   * cannot be safely made sync.
   */
  set ["async"](a) {
    this[ASYNC] = this[ASYNC] || !!a;
  }
  // drop everything and get out of the flow completely
  [ABORT]() {
    this[ABORTED] = true;
    this.emit("abort", this[SIGNAL]?.reason);
    this.destroy(this[SIGNAL]?.reason);
  }
  /**
   * True if the stream has been aborted.
   */
  get aborted() {
    return this[ABORTED];
  }
  /**
   * No-op setter. Stream aborted status is set via the AbortSignal provided
   * in the constructor options.
   */
  set aborted(_) {}
  write(chunk, encoding, cb) {
    if (this[ABORTED]) return false;
    if (this[EOF]) throw new Error("write after end");
    if (this[DESTROYED]) {
      this.emit(
        "error",
        Object.assign(
          new Error("Cannot call write after a stream was destroyed"),
          { code: "ERR_STREAM_DESTROYED" }
        )
      );
      return true;
    }
    if (typeof encoding === "function") {
      cb = encoding;
      encoding = "utf8";
    }
    if (!encoding) encoding = "utf8";
    const fn = this[ASYNC] ? defer : nodefer;
    if (!this[OBJECTMODE] && !Buffer.isBuffer(chunk)) {
      if (isArrayBufferView(chunk)) {
        chunk = Buffer.from(chunk.buffer, chunk.byteOffset, chunk.byteLength);
      } else if (isArrayBufferLike(chunk)) {
        chunk = Buffer.from(chunk);
      } else if (typeof chunk !== "string") {
        throw new Error("Non-contiguous data written to non-objectMode stream");
      }
    }
    if (this[OBJECTMODE]) {
      if (this[FLOWING] && this[BUFFERLENGTH] !== 0) this[FLUSH](true);
      if (this[FLOWING]) this.emit("data", chunk);
      else this[BUFFERPUSH](chunk);
      if (this[BUFFERLENGTH] !== 0) this.emit("readable");
      if (cb) fn(cb);
      return this[FLOWING];
    }
    if (!chunk.length) {
      if (this[BUFFERLENGTH] !== 0) this.emit("readable");
      if (cb) fn(cb);
      return this[FLOWING];
    }
    if (
      typeof chunk === "string" && // unless it is a string already ready for us to use
      !(encoding === this[ENCODING] && !this[DECODER]?.lastNeed)
    ) {
      chunk = Buffer.from(chunk, encoding);
    }
    if (Buffer.isBuffer(chunk) && this[ENCODING]) {
      chunk = this[DECODER].write(chunk);
    }
    if (this[FLOWING] && this[BUFFERLENGTH] !== 0) this[FLUSH](true);
    if (this[FLOWING]) this.emit("data", chunk);
    else this[BUFFERPUSH](chunk);
    if (this[BUFFERLENGTH] !== 0) this.emit("readable");
    if (cb) fn(cb);
    return this[FLOWING];
  }
  /**
   * Low-level explicit read method.
   *
   * In objectMode, the argument is ignored, and one item is returned if
   * available.
   *
   * `n` is the number of bytes (or in the case of encoding streams,
   * characters) to consume. If `n` is not provided, then the entire buffer
   * is returned, or `null` is returned if no data is available.
   *
   * If `n` is greater that the amount of data in the internal buffer,
   * then `null` is returned.
   */
  read(n) {
    if (this[DESTROYED]) return null;
    this[DISCARDED] = false;
    if (this[BUFFERLENGTH] === 0 || n === 0 || (n && n > this[BUFFERLENGTH])) {
      this[MAYBE_EMIT_END]();
      return null;
    }
    if (this[OBJECTMODE]) n = null;
    if (this[BUFFER].length > 1 && !this[OBJECTMODE]) {
      this[BUFFER] = [
        this[ENCODING]
          ? this[BUFFER].join("")
          : Buffer.concat(this[BUFFER], this[BUFFERLENGTH]),
      ];
    }
    const ret = this[READ](n || null, this[BUFFER][0]);
    this[MAYBE_EMIT_END]();
    return ret;
  }
  [READ](n, chunk) {
    if (this[OBJECTMODE]) this[BUFFERSHIFT]();
    else {
      const c = chunk;
      if (n === c.length || n === null) this[BUFFERSHIFT]();
      else if (typeof c === "string") {
        this[BUFFER][0] = c.slice(n);
        chunk = c.slice(0, n);
        this[BUFFERLENGTH] -= n;
      } else {
        this[BUFFER][0] = c.subarray(n);
        chunk = c.subarray(0, n);
        this[BUFFERLENGTH] -= n;
      }
    }
    this.emit("data", chunk);
    if (!this[BUFFER].length && !this[EOF]) this.emit("drain");
    return chunk;
  }
  end(chunk, encoding, cb) {
    if (typeof chunk === "function") {
      cb = chunk;
      chunk = void 0;
    }
    if (typeof encoding === "function") {
      cb = encoding;
      encoding = "utf8";
    }
    if (chunk !== void 0) this.write(chunk, encoding);
    if (cb) this.once("end", cb);
    this[EOF] = true;
    this.writable = false;
    if (this[FLOWING] || !this[PAUSED]) this[MAYBE_EMIT_END]();
    return this;
  }
  // don't let the internal resume be overwritten
  [RESUME]() {
    if (this[DESTROYED]) return;
    if (!this[DATALISTENERS] && !this[PIPES].length) {
      this[DISCARDED] = true;
    }
    this[PAUSED] = false;
    this[FLOWING] = true;
    this.emit("resume");
    if (this[BUFFER].length) this[FLUSH]();
    else if (this[EOF]) this[MAYBE_EMIT_END]();
    else this.emit("drain");
  }
  /**
   * Resume the stream if it is currently in a paused state
   *
   * If called when there are no pipe destinations or `data` event listeners,
   * this will place the stream in a "discarded" state, where all data will
   * be thrown away. The discarded state is removed if a pipe destination or
   * data handler is added, if pause() is called, or if any synchronous or
   * asynchronous iteration is started.
   */
  resume() {
    return this[RESUME]();
  }
  /**
   * Pause the stream
   */
  pause() {
    this[FLOWING] = false;
    this[PAUSED] = true;
    this[DISCARDED] = false;
  }
  /**
   * true if the stream has been forcibly destroyed
   */
  get destroyed() {
    return this[DESTROYED];
  }
  /**
   * true if the stream is currently in a flowing state, meaning that
   * any writes will be immediately emitted.
   */
  get flowing() {
    return this[FLOWING];
  }
  /**
   * true if the stream is currently in a paused state
   */
  get paused() {
    return this[PAUSED];
  }
  [BUFFERPUSH](chunk) {
    if (this[OBJECTMODE]) this[BUFFERLENGTH] += 1;
    else this[BUFFERLENGTH] += chunk.length;
    this[BUFFER].push(chunk);
  }
  [BUFFERSHIFT]() {
    if (this[OBJECTMODE]) this[BUFFERLENGTH] -= 1;
    else this[BUFFERLENGTH] -= this[BUFFER][0].length;
    return this[BUFFER].shift();
  }
  [FLUSH](noDrain = false) {
    do {} while (this[FLUSHCHUNK](this[BUFFERSHIFT]()) && this[BUFFER].length);
    if (!noDrain && !this[BUFFER].length && !this[EOF]) this.emit("drain");
  }
  [FLUSHCHUNK](chunk) {
    this.emit("data", chunk);
    return this[FLOWING];
  }
  /**
   * Pipe all data emitted by this stream into the destination provided.
   *
   * Triggers the flow of data.
   */
  pipe(dest, opts) {
    if (this[DESTROYED]) return dest;
    this[DISCARDED] = false;
    const ended = this[EMITTED_END];
    opts = opts || {};
    if (dest === proc.stdout || dest === proc.stderr) opts.end = false;
    else opts.end = opts.end !== false;
    opts.proxyErrors = !!opts.proxyErrors;
    if (ended) {
      if (opts.end) dest.end();
    } else {
      this[PIPES].push(
        !opts.proxyErrors
          ? new Pipe(this, dest, opts)
          : new PipeProxyErrors(this, dest, opts)
      );
      if (this[ASYNC]) defer(() => this[RESUME]());
      else this[RESUME]();
    }
    return dest;
  }
  /**
   * Fully unhook a piped destination stream.
   *
   * If the destination stream was the only consumer of this stream (ie,
   * there are no other piped destinations or `'data'` event listeners)
   * then the flow of data will stop until there is another consumer or
   * {@link Minipass#resume} is explicitly called.
   */
  unpipe(dest) {
    const p = this[PIPES].find((p2) => p2.dest === dest);
    if (p) {
      if (this[PIPES].length === 1) {
        if (this[FLOWING] && this[DATALISTENERS] === 0) {
          this[FLOWING] = false;
        }
        this[PIPES] = [];
      } else this[PIPES].splice(this[PIPES].indexOf(p), 1);
      p.unpipe();
    }
  }
  /**
   * Alias for {@link Minipass#on}
   */
  addListener(ev, handler) {
    return this.on(ev, handler);
  }
  /**
   * Mostly identical to `EventEmitter.on`, with the following
   * behavior differences to prevent data loss and unnecessary hangs:
   *
   * - Adding a 'data' event handler will trigger the flow of data
   *
   * - Adding a 'readable' event handler when there is data waiting to be read
   *   will cause 'readable' to be emitted immediately.
   *
   * - Adding an 'endish' event handler ('end', 'finish', etc.) which has
   *   already passed will cause the event to be emitted immediately and all
   *   handlers removed.
   *
   * - Adding an 'error' event handler after an error has been emitted will
   *   cause the event to be re-emitted immediately with the error previously
   *   raised.
   */
  on(ev, handler) {
    const ret = super.on(ev, handler);
    if (ev === "data") {
      this[DISCARDED] = false;
      this[DATALISTENERS]++;
      if (!this[PIPES].length && !this[FLOWING]) {
        this[RESUME]();
      }
    } else if (ev === "readable" && this[BUFFERLENGTH] !== 0) {
      super.emit("readable");
    } else if (isEndish(ev) && this[EMITTED_END]) {
      super.emit(ev);
      this.removeAllListeners(ev);
    } else if (ev === "error" && this[EMITTED_ERROR]) {
      const h = handler;
      if (this[ASYNC]) defer(() => h.call(this, this[EMITTED_ERROR]));
      else h.call(this, this[EMITTED_ERROR]);
    }
    return ret;
  }
  /**
   * Alias for {@link Minipass#off}
   */
  removeListener(ev, handler) {
    return this.off(ev, handler);
  }
  /**
   * Mostly identical to `EventEmitter.off`
   *
   * If a 'data' event handler is removed, and it was the last consumer
   * (ie, there are no pipe destinations or other 'data' event listeners),
   * then the flow of data will stop until there is another consumer or
   * {@link Minipass#resume} is explicitly called.
   */
  off(ev, handler) {
    const ret = super.off(ev, handler);
    if (ev === "data") {
      this[DATALISTENERS] = this.listeners("data").length;
      if (
        this[DATALISTENERS] === 0 &&
        !this[DISCARDED] &&
        !this[PIPES].length
      ) {
        this[FLOWING] = false;
      }
    }
    return ret;
  }
  /**
   * Mostly identical to `EventEmitter.removeAllListeners`
   *
   * If all 'data' event handlers are removed, and they were the last consumer
   * (ie, there are no pipe destinations), then the flow of data will stop
   * until there is another consumer or {@link Minipass#resume} is explicitly
   * called.
   */
  removeAllListeners(ev) {
    const ret = super.removeAllListeners(ev);
    if (ev === "data" || ev === void 0) {
      this[DATALISTENERS] = 0;
      if (!this[DISCARDED] && !this[PIPES].length) {
        this[FLOWING] = false;
      }
    }
    return ret;
  }
  /**
   * true if the 'end' event has been emitted
   */
  get emittedEnd() {
    return this[EMITTED_END];
  }
  [MAYBE_EMIT_END]() {
    if (
      !this[EMITTING_END] &&
      !this[EMITTED_END] &&
      !this[DESTROYED] &&
      this[BUFFER].length === 0 &&
      this[EOF]
    ) {
      this[EMITTING_END] = true;
      this.emit("end");
      this.emit("prefinish");
      this.emit("finish");
      if (this[CLOSED]) this.emit("close");
      this[EMITTING_END] = false;
    }
  }
  /**
   * Mostly identical to `EventEmitter.emit`, with the following
   * behavior differences to prevent data loss and unnecessary hangs:
   *
   * If the stream has been destroyed, and the event is something other
   * than 'close' or 'error', then `false` is returned and no handlers
   * are called.
   *
   * If the event is 'end', and has already been emitted, then the event
   * is ignored. If the stream is in a paused or non-flowing state, then
   * the event will be deferred until data flow resumes. If the stream is
   * async, then handlers will be called on the next tick rather than
   * immediately.
   *
   * If the event is 'close', and 'end' has not yet been emitted, then
   * the event will be deferred until after 'end' is emitted.
   *
   * If the event is 'error', and an AbortSignal was provided for the stream,
   * and there are no listeners, then the event is ignored, matching the
   * behavior of node core streams in the presense of an AbortSignal.
   *
   * If the event is 'finish' or 'prefinish', then all listeners will be
   * removed after emitting the event, to prevent double-firing.
   */
  emit(ev, ...args) {
    const data = args[0];
    if (
      ev !== "error" &&
      ev !== "close" &&
      ev !== DESTROYED &&
      this[DESTROYED]
    ) {
      return false;
    } else if (ev === "data") {
      return !this[OBJECTMODE] && !data
        ? false
        : this[ASYNC]
        ? (defer(() => this[EMITDATA](data)), true)
        : this[EMITDATA](data);
    } else if (ev === "end") {
      return this[EMITEND]();
    } else if (ev === "close") {
      this[CLOSED] = true;
      if (!this[EMITTED_END] && !this[DESTROYED]) return false;
      const ret2 = super.emit("close");
      this.removeAllListeners("close");
      return ret2;
    } else if (ev === "error") {
      this[EMITTED_ERROR] = data;
      super.emit(ERROR, data);
      const ret2 =
        !this[SIGNAL] || this.listeners("error").length
          ? super.emit("error", data)
          : false;
      this[MAYBE_EMIT_END]();
      return ret2;
    } else if (ev === "resume") {
      const ret2 = super.emit("resume");
      this[MAYBE_EMIT_END]();
      return ret2;
    } else if (ev === "finish" || ev === "prefinish") {
      const ret2 = super.emit(ev);
      this.removeAllListeners(ev);
      return ret2;
    }
    const ret = super.emit(ev, ...args);
    this[MAYBE_EMIT_END]();
    return ret;
  }
  [EMITDATA](data) {
    for (const p of this[PIPES]) {
      if (p.dest.write(data) === false) this.pause();
    }
    const ret = this[DISCARDED] ? false : super.emit("data", data);
    this[MAYBE_EMIT_END]();
    return ret;
  }
  [EMITEND]() {
    if (this[EMITTED_END]) return false;
    this[EMITTED_END] = true;
    this.readable = false;
    return this[ASYNC]
      ? (defer(() => this[EMITEND2]()), true)
      : this[EMITEND2]();
  }
  [EMITEND2]() {
    if (this[DECODER]) {
      const data = this[DECODER].end();
      if (data) {
        for (const p of this[PIPES]) {
          p.dest.write(data);
        }
        if (!this[DISCARDED]) super.emit("data", data);
      }
    }
    for (const p of this[PIPES]) {
      p.end();
    }
    const ret = super.emit("end");
    this.removeAllListeners("end");
    return ret;
  }
  /**
   * Return a Promise that resolves to an array of all emitted data once
   * the stream ends.
   */
  async collect() {
    const buf = Object.assign([], {
      dataLength: 0,
    });
    if (!this[OBJECTMODE]) buf.dataLength = 0;
    const p = this.promise();
    this.on("data", (c) => {
      buf.push(c);
      if (!this[OBJECTMODE]) buf.dataLength += c.length;
    });
    await p;
    return buf;
  }
  /**
   * Return a Promise that resolves to the concatenation of all emitted data
   * once the stream ends.
   *
   * Not allowed on objectMode streams.
   */
  async concat() {
    if (this[OBJECTMODE]) {
      throw new Error("cannot concat in objectMode");
    }
    const buf = await this.collect();
    return this[ENCODING] ? buf.join("") : Buffer.concat(buf, buf.dataLength);
  }
  /**
   * Return a void Promise that resolves once the stream ends.
   */
  async promise() {
    return new Promise((resolve2, reject) => {
      this.on(DESTROYED, () => reject(new Error("stream destroyed")));
      this.on("error", (er) => reject(er));
      this.on("end", () => resolve2());
    });
  }
  /**
   * Asynchronous `for await of` iteration.
   *
   * This will continue emitting all chunks until the stream terminates.
   */
  [Symbol.asyncIterator]() {
    this[DISCARDED] = false;
    let stopped = false;
    const stop = async () => {
      this.pause();
      stopped = true;
      return { value: void 0, done: true };
    };
    const next = () => {
      if (stopped) return stop();
      const res = this.read();
      if (res !== null) return Promise.resolve({ done: false, value: res });
      if (this[EOF]) return stop();
      let resolve2;
      let reject;
      const onerr = (er) => {
        this.off("data", ondata);
        this.off("end", onend);
        this.off(DESTROYED, ondestroy);
        stop();
        reject(er);
      };
      const ondata = (value) => {
        this.off("error", onerr);
        this.off("end", onend);
        this.off(DESTROYED, ondestroy);
        this.pause();
        resolve2({ value, done: !!this[EOF] });
      };
      const onend = () => {
        this.off("error", onerr);
        this.off("data", ondata);
        this.off(DESTROYED, ondestroy);
        stop();
        resolve2({ done: true, value: void 0 });
      };
      const ondestroy = () => onerr(new Error("stream destroyed"));
      return new Promise((res2, rej) => {
        reject = rej;
        resolve2 = res2;
        this.once(DESTROYED, ondestroy);
        this.once("error", onerr);
        this.once("end", onend);
        this.once("data", ondata);
      });
    };
    return {
      next,
      throw: stop,
      return: stop,
      [Symbol.asyncIterator]() {
        return this;
      },
    };
  }
  /**
   * Synchronous `for of` iteration.
   *
   * The iteration will terminate when the internal buffer runs out, even
   * if the stream has not yet terminated.
   */
  [Symbol.iterator]() {
    this[DISCARDED] = false;
    let stopped = false;
    const stop = () => {
      this.pause();
      this.off(ERROR, stop);
      this.off(DESTROYED, stop);
      this.off("end", stop);
      stopped = true;
      return { done: true, value: void 0 };
    };
    const next = () => {
      if (stopped) return stop();
      const value = this.read();
      return value === null ? stop() : { done: false, value };
    };
    this.once("end", stop);
    this.once(ERROR, stop);
    this.once(DESTROYED, stop);
    return {
      next,
      throw: stop,
      return: stop,
      [Symbol.iterator]() {
        return this;
      },
    };
  }
  /**
   * Destroy a stream, preventing it from being used for any further purpose.
   *
   * If the stream has a `close()` method, then it will be called on
   * destruction.
   *
   * After destruction, any attempt to write data, read data, or emit most
   * events will be ignored.
   *
   * If an error argument is provided, then it will be emitted in an
   * 'error' event.
   */
  destroy(er) {
    if (this[DESTROYED]) {
      if (er) this.emit("error", er);
      else this.emit(DESTROYED);
      return this;
    }
    this[DESTROYED] = true;
    this[DISCARDED] = true;
    this[BUFFER].length = 0;
    this[BUFFERLENGTH] = 0;
    const wc = this;
    if (typeof wc.close === "function" && !this[CLOSED]) wc.close();
    if (er) this.emit("error", er);
    else this.emit(DESTROYED);
    return this;
  }
  /**
   * Alias for {@link isStream}
   *
   * Former export location, maintained for backwards compatibility.
   *
   * @deprecated
   */
  static get isStream() {
    return isStream;
  }
};

// node_modules/@isaacs/fs-minipass/dist/esm/index.js
var writev = import_fs.default.writev;
var _autoClose = Symbol("_autoClose");
var _close = Symbol("_close");
var _ended = Symbol("_ended");
var _fd = Symbol("_fd");
var _finished = Symbol("_finished");
var _flags = Symbol("_flags");
var _flush = Symbol("_flush");
var _handleChunk = Symbol("_handleChunk");
var _makeBuf = Symbol("_makeBuf");
var _mode = Symbol("_mode");
var _needDrain = Symbol("_needDrain");
var _onerror = Symbol("_onerror");
var _onopen = Symbol("_onopen");
var _onread = Symbol("_onread");
var _onwrite = Symbol("_onwrite");
var _open = Symbol("_open");
var _path = Symbol("_path");
var _pos = Symbol("_pos");
var _queue = Symbol("_queue");
var _read = Symbol("_read");
var _readSize = Symbol("_readSize");
var _reading = Symbol("_reading");
var _remain = Symbol("_remain");
var _size = Symbol("_size");
var _write = Symbol("_write");
var _writing = Symbol("_writing");
var _defaultFlag = Symbol("_defaultFlag");
var _errored = Symbol("_errored");
var ReadStream = class extends Minipass {
  [_errored] = false;
  [_fd];
  [_path];
  [_readSize];
  [_reading] = false;
  [_size];
  [_remain];
  [_autoClose];
  constructor(path8, opt) {
    opt = opt || {};
    super(opt);
    this.readable = true;
    this.writable = false;
    if (typeof path8 !== "string") {
      throw new TypeError("path must be a string");
    }
    this[_errored] = false;
    this[_fd] = typeof opt.fd === "number" ? opt.fd : void 0;
    this[_path] = path8;
    this[_readSize] = opt.readSize || 16 * 1024 * 1024;
    this[_reading] = false;
    this[_size] = typeof opt.size === "number" ? opt.size : Infinity;
    this[_remain] = this[_size];
    this[_autoClose] =
      typeof opt.autoClose === "boolean" ? opt.autoClose : true;
    if (typeof this[_fd] === "number") {
      this[_read]();
    } else {
      this[_open]();
    }
  }
  get fd() {
    return this[_fd];
  }
  get path() {
    return this[_path];
  }
  //@ts-ignore
  write() {
    throw new TypeError("this is a readable stream");
  }
  //@ts-ignore
  end() {
    throw new TypeError("this is a readable stream");
  }
  [_open]() {
    import_fs.default.open(this[_path], "r", (er, fd) => this[_onopen](er, fd));
  }
  [_onopen](er, fd) {
    if (er) {
      this[_onerror](er);
    } else {
      this[_fd] = fd;
      this.emit("open", fd);
      this[_read]();
    }
  }
  [_makeBuf]() {
    return Buffer.allocUnsafe(Math.min(this[_readSize], this[_remain]));
  }
  [_read]() {
    if (!this[_reading]) {
      this[_reading] = true;
      const buf = this[_makeBuf]();
      if (buf.length === 0) {
        return process.nextTick(() => this[_onread](null, 0, buf));
      }
      import_fs.default.read(this[_fd], buf, 0, buf.length, null, (er, br, b) =>
        this[_onread](er, br, b)
      );
    }
  }
  [_onread](er, br, buf) {
    this[_reading] = false;
    if (er) {
      this[_onerror](er);
    } else if (this[_handleChunk](br, buf)) {
      this[_read]();
    }
  }
  [_close]() {
    if (this[_autoClose] && typeof this[_fd] === "number") {
      const fd = this[_fd];
      this[_fd] = void 0;
      import_fs.default.close(fd, (er) =>
        er ? this.emit("error", er) : this.emit("close")
      );
    }
  }
  [_onerror](er) {
    this[_reading] = true;
    this[_close]();
    this.emit("error", er);
  }
  [_handleChunk](br, buf) {
    let ret = false;
    this[_remain] -= br;
    if (br > 0) {
      ret = super.write(br < buf.length ? buf.subarray(0, br) : buf);
    }
    if (br === 0 || this[_remain] <= 0) {
      ret = false;
      this[_close]();
      super.end();
    }
    return ret;
  }
  emit(ev, ...args) {
    switch (ev) {
      case "prefinish":
      case "finish":
        return false;
      case "drain":
        if (typeof this[_fd] === "number") {
          this[_read]();
        }
        return false;
      case "error":
        if (this[_errored]) {
          return false;
        }
        this[_errored] = true;
        return super.emit(ev, ...args);
      default:
        return super.emit(ev, ...args);
    }
  }
};
var ReadStreamSync = class extends ReadStream {
  [_open]() {
    let threw = true;
    try {
      this[_onopen](null, import_fs.default.openSync(this[_path], "r"));
      threw = false;
    } finally {
      if (threw) {
        this[_close]();
      }
    }
  }
  [_read]() {
    let threw = true;
    try {
      if (!this[_reading]) {
        this[_reading] = true;
        do {
          const buf = this[_makeBuf]();
          const br =
            buf.length === 0
              ? 0
              : import_fs.default.readSync(this[_fd], buf, 0, buf.length, null);
          if (!this[_handleChunk](br, buf)) {
            break;
          }
        } while (true);
        this[_reading] = false;
      }
      threw = false;
    } finally {
      if (threw) {
        this[_close]();
      }
    }
  }
  [_close]() {
    if (this[_autoClose] && typeof this[_fd] === "number") {
      const fd = this[_fd];
      this[_fd] = void 0;
      import_fs.default.closeSync(fd);
      this.emit("close");
    }
  }
};
var WriteStream = class extends import_events.default {
  readable = false;
  writable = true;
  [_errored] = false;
  [_writing] = false;
  [_ended] = false;
  [_queue] = [];
  [_needDrain] = false;
  [_path];
  [_mode];
  [_autoClose];
  [_fd];
  [_defaultFlag];
  [_flags];
  [_finished] = false;
  [_pos];
  constructor(path8, opt) {
    opt = opt || {};
    super(opt);
    this[_path] = path8;
    this[_fd] = typeof opt.fd === "number" ? opt.fd : void 0;
    this[_mode] = opt.mode === void 0 ? 438 : opt.mode;
    this[_pos] = typeof opt.start === "number" ? opt.start : void 0;
    this[_autoClose] =
      typeof opt.autoClose === "boolean" ? opt.autoClose : true;
    const defaultFlag = this[_pos] !== void 0 ? "r+" : "w";
    this[_defaultFlag] = opt.flags === void 0;
    this[_flags] = opt.flags === void 0 ? defaultFlag : opt.flags;
    if (this[_fd] === void 0) {
      this[_open]();
    }
  }
  emit(ev, ...args) {
    if (ev === "error") {
      if (this[_errored]) {
        return false;
      }
      this[_errored] = true;
    }
    return super.emit(ev, ...args);
  }
  get fd() {
    return this[_fd];
  }
  get path() {
    return this[_path];
  }
  [_onerror](er) {
    this[_close]();
    this[_writing] = true;
    this.emit("error", er);
  }
  [_open]() {
    import_fs.default.open(this[_path], this[_flags], this[_mode], (er, fd) =>
      this[_onopen](er, fd)
    );
  }
  [_onopen](er, fd) {
    if (
      this[_defaultFlag] &&
      this[_flags] === "r+" &&
      er &&
      er.code === "ENOENT"
    ) {
      this[_flags] = "w";
      this[_open]();
    } else if (er) {
      this[_onerror](er);
    } else {
      this[_fd] = fd;
      this.emit("open", fd);
      if (!this[_writing]) {
        this[_flush]();
      }
    }
  }
  end(buf, enc) {
    if (buf) {
      this.write(buf, enc);
    }
    this[_ended] = true;
    if (
      !this[_writing] &&
      !this[_queue].length &&
      typeof this[_fd] === "number"
    ) {
      this[_onwrite](null, 0);
    }
    return this;
  }
  write(buf, enc) {
    if (typeof buf === "string") {
      buf = Buffer.from(buf, enc);
    }
    if (this[_ended]) {
      this.emit("error", new Error("write() after end()"));
      return false;
    }
    if (this[_fd] === void 0 || this[_writing] || this[_queue].length) {
      this[_queue].push(buf);
      this[_needDrain] = true;
      return false;
    }
    this[_writing] = true;
    this[_write](buf);
    return true;
  }
  [_write](buf) {
    import_fs.default.write(
      this[_fd],
      buf,
      0,
      buf.length,
      this[_pos],
      (er, bw) => this[_onwrite](er, bw)
    );
  }
  [_onwrite](er, bw) {
    if (er) {
      this[_onerror](er);
    } else {
      if (this[_pos] !== void 0 && typeof bw === "number") {
        this[_pos] += bw;
      }
      if (this[_queue].length) {
        this[_flush]();
      } else {
        this[_writing] = false;
        if (this[_ended] && !this[_finished]) {
          this[_finished] = true;
          this[_close]();
          this.emit("finish");
        } else if (this[_needDrain]) {
          this[_needDrain] = false;
          this.emit("drain");
        }
      }
    }
  }
  [_flush]() {
    if (this[_queue].length === 0) {
      if (this[_ended]) {
        this[_onwrite](null, 0);
      }
    } else if (this[_queue].length === 1) {
      this[_write](this[_queue].pop());
    } else {
      const iovec = this[_queue];
      this[_queue] = [];
      writev(this[_fd], iovec, this[_pos], (er, bw) => this[_onwrite](er, bw));
    }
  }
  [_close]() {
    if (this[_autoClose] && typeof this[_fd] === "number") {
      const fd = this[_fd];
      this[_fd] = void 0;
      import_fs.default.close(fd, (er) =>
        er ? this.emit("error", er) : this.emit("close")
      );
    }
  }
};
var WriteStreamSync = class extends WriteStream {
  [_open]() {
    let fd;
    if (this[_defaultFlag] && this[_flags] === "r+") {
      try {
        fd = import_fs.default.openSync(this[_path], this[_flags], this[_mode]);
      } catch (er) {
        if (er?.code === "ENOENT") {
          this[_flags] = "w";
          return this[_open]();
        } else {
          throw er;
        }
      }
    } else {
      fd = import_fs.default.openSync(this[_path], this[_flags], this[_mode]);
    }
    this[_onopen](null, fd);
  }
  [_close]() {
    if (this[_autoClose] && typeof this[_fd] === "number") {
      const fd = this[_fd];
      this[_fd] = void 0;
      import_fs.default.closeSync(fd);
      this.emit("close");
    }
  }
  [_write](buf) {
    let threw = true;
    try {
      this[_onwrite](
        null,
        import_fs.default.writeSync(this[_fd], buf, 0, buf.length, this[_pos])
      );
      threw = false;
    } finally {
      if (threw) {
        try {
          this[_close]();
        } catch {}
      }
    }
  }
};

// node_modules/tar/dist/esm/create.js
var import_node_path4 = __toESM(require("node:path"), 1);

// node_modules/tar/dist/esm/list.js
var import_node_fs = __toESM(require("node:fs"), 1);
var import_path = require("path");

// node_modules/tar/dist/esm/options.js
var argmap = /* @__PURE__ */ new Map([
  ["C", "cwd"],
  ["f", "file"],
  ["z", "gzip"],
  ["P", "preservePaths"],
  ["U", "unlink"],
  ["strip-components", "strip"],
  ["stripComponents", "strip"],
  ["keep-newer", "newer"],
  ["keepNewer", "newer"],
  ["keep-newer-files", "newer"],
  ["keepNewerFiles", "newer"],
  ["k", "keep"],
  ["keep-existing", "keep"],
  ["keepExisting", "keep"],
  ["m", "noMtime"],
  ["no-mtime", "noMtime"],
  ["p", "preserveOwner"],
  ["L", "follow"],
  ["h", "follow"],
  ["onentry", "onReadEntry"],
]);
var isSyncFile = (o) => !!o.sync && !!o.file;
var isAsyncFile = (o) => !o.sync && !!o.file;
var isSyncNoFile = (o) => !!o.sync && !o.file;
var isAsyncNoFile = (o) => !o.sync && !o.file;
var isFile = (o) => !!o.file;
var dealiasKey = (k) => {
  const d = argmap.get(k);
  if (d) return d;
  return k;
};
var dealias = (opt = {}) => {
  if (!opt) return {};
  const result = {};
  for (const [key, v] of Object.entries(opt)) {
    const k = dealiasKey(key);
    result[k] = v;
  }
  if (result.chmod === void 0 && result.noChmod === false) {
    result.chmod = true;
  }
  delete result.noChmod;
  return result;
};

// node_modules/tar/dist/esm/make-command.js
var makeCommand = (syncFile, asyncFile, syncNoFile, asyncNoFile, validate) => {
  return Object.assign(
    (opt_ = [], entries, cb) => {
      if (Array.isArray(opt_)) {
        entries = opt_;
        opt_ = {};
      }
      if (typeof entries === "function") {
        cb = entries;
        entries = void 0;
      }
      if (!entries) {
        entries = [];
      } else {
        entries = Array.from(entries);
      }
      const opt = dealias(opt_);
      validate?.(opt, entries);
      if (isSyncFile(opt)) {
        if (typeof cb === "function") {
          throw new TypeError("callback not supported for sync tar functions");
        }
        return syncFile(opt, entries);
      } else if (isAsyncFile(opt)) {
        const p = asyncFile(opt, entries);
        const c = cb ? cb : void 0;
        return c ? p.then(() => c(), c) : p;
      } else if (isSyncNoFile(opt)) {
        if (typeof cb === "function") {
          throw new TypeError("callback not supported for sync tar functions");
        }
        return syncNoFile(opt, entries);
      } else if (isAsyncNoFile(opt)) {
        if (typeof cb === "function") {
          throw new TypeError("callback only supported with file option");
        }
        return asyncNoFile(opt, entries);
      } else {
        throw new Error("impossible options??");
      }
    },
    {
      syncFile,
      asyncFile,
      syncNoFile,
      asyncNoFile,
      validate,
    }
  );
};

// node_modules/tar/dist/esm/parse.js
var import_events2 = require("events");

// node_modules/minizlib/dist/esm/index.js
var import_assert = __toESM(require("assert"), 1);
var import_buffer = require("buffer");
var realZlib2 = __toESM(require("zlib"), 1);

// node_modules/minizlib/dist/esm/constants.js
var import_zlib = __toESM(require("zlib"), 1);
var realZlibConstants = import_zlib.default.constants || { ZLIB_VERNUM: 4736 };
var constants = Object.freeze(
  Object.assign(
    /* @__PURE__ */ Object.create(null),
    {
      Z_NO_FLUSH: 0,
      Z_PARTIAL_FLUSH: 1,
      Z_SYNC_FLUSH: 2,
      Z_FULL_FLUSH: 3,
      Z_FINISH: 4,
      Z_BLOCK: 5,
      Z_OK: 0,
      Z_STREAM_END: 1,
      Z_NEED_DICT: 2,
      Z_ERRNO: -1,
      Z_STREAM_ERROR: -2,
      Z_DATA_ERROR: -3,
      Z_MEM_ERROR: -4,
      Z_BUF_ERROR: -5,
      Z_VERSION_ERROR: -6,
      Z_NO_COMPRESSION: 0,
      Z_BEST_SPEED: 1,
      Z_BEST_COMPRESSION: 9,
      Z_DEFAULT_COMPRESSION: -1,
      Z_FILTERED: 1,
      Z_HUFFMAN_ONLY: 2,
      Z_RLE: 3,
      Z_FIXED: 4,
      Z_DEFAULT_STRATEGY: 0,
      DEFLATE: 1,
      INFLATE: 2,
      GZIP: 3,
      GUNZIP: 4,
      DEFLATERAW: 5,
      INFLATERAW: 6,
      UNZIP: 7,
      BROTLI_DECODE: 8,
      BROTLI_ENCODE: 9,
      Z_MIN_WINDOWBITS: 8,
      Z_MAX_WINDOWBITS: 15,
      Z_DEFAULT_WINDOWBITS: 15,
      Z_MIN_CHUNK: 64,
      Z_MAX_CHUNK: Infinity,
      Z_DEFAULT_CHUNK: 16384,
      Z_MIN_MEMLEVEL: 1,
      Z_MAX_MEMLEVEL: 9,
      Z_DEFAULT_MEMLEVEL: 8,
      Z_MIN_LEVEL: -1,
      Z_MAX_LEVEL: 9,
      Z_DEFAULT_LEVEL: -1,
      BROTLI_OPERATION_PROCESS: 0,
      BROTLI_OPERATION_FLUSH: 1,
      BROTLI_OPERATION_FINISH: 2,
      BROTLI_OPERATION_EMIT_METADATA: 3,
      BROTLI_MODE_GENERIC: 0,
      BROTLI_MODE_TEXT: 1,
      BROTLI_MODE_FONT: 2,
      BROTLI_DEFAULT_MODE: 0,
      BROTLI_MIN_QUALITY: 0,
      BROTLI_MAX_QUALITY: 11,
      BROTLI_DEFAULT_QUALITY: 11,
      BROTLI_MIN_WINDOW_BITS: 10,
      BROTLI_MAX_WINDOW_BITS: 24,
      BROTLI_LARGE_MAX_WINDOW_BITS: 30,
      BROTLI_DEFAULT_WINDOW: 22,
      BROTLI_MIN_INPUT_BLOCK_BITS: 16,
      BROTLI_MAX_INPUT_BLOCK_BITS: 24,
      BROTLI_PARAM_MODE: 0,
      BROTLI_PARAM_QUALITY: 1,
      BROTLI_PARAM_LGWIN: 2,
      BROTLI_PARAM_LGBLOCK: 3,
      BROTLI_PARAM_DISABLE_LITERAL_CONTEXT_MODELING: 4,
      BROTLI_PARAM_SIZE_HINT: 5,
      BROTLI_PARAM_LARGE_WINDOW: 6,
      BROTLI_PARAM_NPOSTFIX: 7,
      BROTLI_PARAM_NDIRECT: 8,
      BROTLI_DECODER_RESULT_ERROR: 0,
      BROTLI_DECODER_RESULT_SUCCESS: 1,
      BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT: 2,
      BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT: 3,
      BROTLI_DECODER_PARAM_DISABLE_RING_BUFFER_REALLOCATION: 0,
      BROTLI_DECODER_PARAM_LARGE_WINDOW: 1,
      BROTLI_DECODER_NO_ERROR: 0,
      BROTLI_DECODER_SUCCESS: 1,
      BROTLI_DECODER_NEEDS_MORE_INPUT: 2,
      BROTLI_DECODER_NEEDS_MORE_OUTPUT: 3,
      BROTLI_DECODER_ERROR_FORMAT_EXUBERANT_NIBBLE: -1,
      BROTLI_DECODER_ERROR_FORMAT_RESERVED: -2,
      BROTLI_DECODER_ERROR_FORMAT_EXUBERANT_META_NIBBLE: -3,
      BROTLI_DECODER_ERROR_FORMAT_SIMPLE_HUFFMAN_ALPHABET: -4,
      BROTLI_DECODER_ERROR_FORMAT_SIMPLE_HUFFMAN_SAME: -5,
      BROTLI_DECODER_ERROR_FORMAT_CL_SPACE: -6,
      BROTLI_DECODER_ERROR_FORMAT_HUFFMAN_SPACE: -7,
      BROTLI_DECODER_ERROR_FORMAT_CONTEXT_MAP_REPEAT: -8,
      BROTLI_DECODER_ERROR_FORMAT_BLOCK_LENGTH_1: -9,
      BROTLI_DECODER_ERROR_FORMAT_BLOCK_LENGTH_2: -10,
      BROTLI_DECODER_ERROR_FORMAT_TRANSFORM: -11,
      BROTLI_DECODER_ERROR_FORMAT_DICTIONARY: -12,
      BROTLI_DECODER_ERROR_FORMAT_WINDOW_BITS: -13,
      BROTLI_DECODER_ERROR_FORMAT_PADDING_1: -14,
      BROTLI_DECODER_ERROR_FORMAT_PADDING_2: -15,
      BROTLI_DECODER_ERROR_FORMAT_DISTANCE: -16,
      BROTLI_DECODER_ERROR_DICTIONARY_NOT_SET: -19,
      BROTLI_DECODER_ERROR_INVALID_ARGUMENTS: -20,
      BROTLI_DECODER_ERROR_ALLOC_CONTEXT_MODES: -21,
      BROTLI_DECODER_ERROR_ALLOC_TREE_GROUPS: -22,
      BROTLI_DECODER_ERROR_ALLOC_CONTEXT_MAP: -25,
      BROTLI_DECODER_ERROR_ALLOC_RING_BUFFER_1: -26,
      BROTLI_DECODER_ERROR_ALLOC_RING_BUFFER_2: -27,
      BROTLI_DECODER_ERROR_ALLOC_BLOCK_TYPE_TREES: -30,
      BROTLI_DECODER_ERROR_UNREACHABLE: -31,
    },
    realZlibConstants
  )
);

// node_modules/minizlib/dist/esm/index.js
var OriginalBufferConcat = import_buffer.Buffer.concat;
var desc = Object.getOwnPropertyDescriptor(import_buffer.Buffer, "concat");
var noop = (args) => args;
var passthroughBufferConcat =
  desc?.writable === true || desc?.set !== void 0
    ? (makeNoOp) => {
        import_buffer.Buffer.concat = makeNoOp ? noop : OriginalBufferConcat;
      }
    : (_) => {};
var _superWrite = Symbol("_superWrite");
var ZlibError = class extends Error {
  code;
  errno;
  constructor(err) {
    super("zlib: " + err.message);
    this.code = err.code;
    this.errno = err.errno;
    if (!this.code) this.code = "ZLIB_ERROR";
    this.message = "zlib: " + err.message;
    Error.captureStackTrace(this, this.constructor);
  }
  get name() {
    return "ZlibError";
  }
};
var _flushFlag = Symbol("flushFlag");
var ZlibBase = class extends Minipass {
  #sawError = false;
  #ended = false;
  #flushFlag;
  #finishFlushFlag;
  #fullFlushFlag;
  #handle;
  #onError;
  get sawError() {
    return this.#sawError;
  }
  get handle() {
    return this.#handle;
  }
  /* c8 ignore start */
  get flushFlag() {
    return this.#flushFlag;
  }
  /* c8 ignore stop */
  constructor(opts, mode) {
    if (!opts || typeof opts !== "object")
      throw new TypeError("invalid options for ZlibBase constructor");
    super(opts);
    this.#flushFlag = opts.flush ?? 0;
    this.#finishFlushFlag = opts.finishFlush ?? 0;
    this.#fullFlushFlag = opts.fullFlushFlag ?? 0;
    try {
      this.#handle = new realZlib2[mode](opts);
    } catch (er) {
      throw new ZlibError(er);
    }
    this.#onError = (err) => {
      if (this.#sawError) return;
      this.#sawError = true;
      this.close();
      this.emit("error", err);
    };
    this.#handle?.on("error", (er) => this.#onError(new ZlibError(er)));
    this.once("end", () => this.close);
  }
  close() {
    if (this.#handle) {
      this.#handle.close();
      this.#handle = void 0;
      this.emit("close");
    }
  }
  reset() {
    if (!this.#sawError) {
      (0, import_assert.default)(this.#handle, "zlib binding closed");
      return this.#handle.reset?.();
    }
  }
  flush(flushFlag) {
    if (this.ended) return;
    if (typeof flushFlag !== "number") flushFlag = this.#fullFlushFlag;
    this.write(
      Object.assign(import_buffer.Buffer.alloc(0), { [_flushFlag]: flushFlag })
    );
  }
  end(chunk, encoding, cb) {
    if (typeof chunk === "function") {
      cb = chunk;
      encoding = void 0;
      chunk = void 0;
    }
    if (typeof encoding === "function") {
      cb = encoding;
      encoding = void 0;
    }
    if (chunk) {
      if (encoding) this.write(chunk, encoding);
      else this.write(chunk);
    }
    this.flush(this.#finishFlushFlag);
    this.#ended = true;
    return super.end(cb);
  }
  get ended() {
    return this.#ended;
  }
  // overridden in the gzip classes to do portable writes
  [_superWrite](data) {
    return super.write(data);
  }
  write(chunk, encoding, cb) {
    if (typeof encoding === "function") (cb = encoding), (encoding = "utf8");
    if (typeof chunk === "string")
      chunk = import_buffer.Buffer.from(chunk, encoding);
    if (this.#sawError) return;
    (0, import_assert.default)(this.#handle, "zlib binding closed");
    const nativeHandle = this.#handle._handle;
    const originalNativeClose = nativeHandle.close;
    nativeHandle.close = () => {};
    const originalClose = this.#handle.close;
    this.#handle.close = () => {};
    passthroughBufferConcat(true);
    let result = void 0;
    try {
      const flushFlag =
        typeof chunk[_flushFlag] === "number"
          ? chunk[_flushFlag]
          : this.#flushFlag;
      result = this.#handle._processChunk(chunk, flushFlag);
      passthroughBufferConcat(false);
    } catch (err) {
      passthroughBufferConcat(false);
      this.#onError(new ZlibError(err));
    } finally {
      if (this.#handle) {
        this.#handle._handle = nativeHandle;
        nativeHandle.close = originalNativeClose;
        this.#handle.close = originalClose;
        this.#handle.removeAllListeners("error");
      }
    }
    if (this.#handle)
      this.#handle.on("error", (er) => this.#onError(new ZlibError(er)));
    let writeReturn;
    if (result) {
      if (Array.isArray(result) && result.length > 0) {
        const r = result[0];
        writeReturn = this[_superWrite](import_buffer.Buffer.from(r));
        for (let i = 1; i < result.length; i++) {
          writeReturn = this[_superWrite](result[i]);
        }
      } else {
        writeReturn = this[_superWrite](import_buffer.Buffer.from(result));
      }
    }
    if (cb) cb();
    return writeReturn;
  }
};
var Zlib = class extends ZlibBase {
  #level;
  #strategy;
  constructor(opts, mode) {
    opts = opts || {};
    opts.flush = opts.flush || constants.Z_NO_FLUSH;
    opts.finishFlush = opts.finishFlush || constants.Z_FINISH;
    opts.fullFlushFlag = constants.Z_FULL_FLUSH;
    super(opts, mode);
    this.#level = opts.level;
    this.#strategy = opts.strategy;
  }
  params(level, strategy) {
    if (this.sawError) return;
    if (!this.handle)
      throw new Error("cannot switch params when binding is closed");
    if (!this.handle.params)
      throw new Error("not supported in this implementation");
    if (this.#level !== level || this.#strategy !== strategy) {
      this.flush(constants.Z_SYNC_FLUSH);
      (0, import_assert.default)(this.handle, "zlib binding closed");
      const origFlush = this.handle.flush;
      this.handle.flush = (flushFlag, cb) => {
        if (typeof flushFlag === "function") {
          cb = flushFlag;
          flushFlag = this.flushFlag;
        }
        this.flush(flushFlag);
        cb?.();
      };
      try {
        this.handle.params(level, strategy);
      } finally {
        this.handle.flush = origFlush;
      }
      if (this.handle) {
        this.#level = level;
        this.#strategy = strategy;
      }
    }
  }
};
var Gzip = class extends Zlib {
  #portable;
  constructor(opts) {
    super(opts, "Gzip");
    this.#portable = opts && !!opts.portable;
  }
  [_superWrite](data) {
    if (!this.#portable) return super[_superWrite](data);
    this.#portable = false;
    data[9] = 255;
    return super[_superWrite](data);
  }
};
var Unzip = class extends Zlib {
  constructor(opts) {
    super(opts, "Unzip");
  }
};
var Brotli = class extends ZlibBase {
  constructor(opts, mode) {
    opts = opts || {};
    opts.flush = opts.flush || constants.BROTLI_OPERATION_PROCESS;
    opts.finishFlush = opts.finishFlush || constants.BROTLI_OPERATION_FINISH;
    opts.fullFlushFlag = constants.BROTLI_OPERATION_FLUSH;
    super(opts, mode);
  }
};
var BrotliCompress = class extends Brotli {
  constructor(opts) {
    super(opts, "BrotliCompress");
  }
};
var BrotliDecompress = class extends Brotli {
  constructor(opts) {
    super(opts, "BrotliDecompress");
  }
};

// node_modules/yallist/dist/esm/index.js
var Yallist = class _Yallist {
  tail;
  head;
  length = 0;
  static create(list2 = []) {
    return new _Yallist(list2);
  }
  constructor(list2 = []) {
    for (const item of list2) {
      this.push(item);
    }
  }
  *[Symbol.iterator]() {
    for (let walker = this.head; walker; walker = walker.next) {
      yield walker.value;
    }
  }
  removeNode(node) {
    if (node.list !== this) {
      throw new Error("removing node which does not belong to this list");
    }
    const next = node.next;
    const prev = node.prev;
    if (next) {
      next.prev = prev;
    }
    if (prev) {
      prev.next = next;
    }
    if (node === this.head) {
      this.head = next;
    }
    if (node === this.tail) {
      this.tail = prev;
    }
    this.length--;
    node.next = void 0;
    node.prev = void 0;
    node.list = void 0;
    return next;
  }
  unshiftNode(node) {
    if (node === this.head) {
      return;
    }
    if (node.list) {
      node.list.removeNode(node);
    }
    const head = this.head;
    node.list = this;
    node.next = head;
    if (head) {
      head.prev = node;
    }
    this.head = node;
    if (!this.tail) {
      this.tail = node;
    }
    this.length++;
  }
  pushNode(node) {
    if (node === this.tail) {
      return;
    }
    if (node.list) {
      node.list.removeNode(node);
    }
    const tail = this.tail;
    node.list = this;
    node.prev = tail;
    if (tail) {
      tail.next = node;
    }
    this.tail = node;
    if (!this.head) {
      this.head = node;
    }
    this.length++;
  }
  push(...args) {
    for (let i = 0, l = args.length; i < l; i++) {
      push(this, args[i]);
    }
    return this.length;
  }
  unshift(...args) {
    for (var i = 0, l = args.length; i < l; i++) {
      unshift(this, args[i]);
    }
    return this.length;
  }
  pop() {
    if (!this.tail) {
      return void 0;
    }
    const res = this.tail.value;
    const t = this.tail;
    this.tail = this.tail.prev;
    if (this.tail) {
      this.tail.next = void 0;
    } else {
      this.head = void 0;
    }
    t.list = void 0;
    this.length--;
    return res;
  }
  shift() {
    if (!this.head) {
      return void 0;
    }
    const res = this.head.value;
    const h = this.head;
    this.head = this.head.next;
    if (this.head) {
      this.head.prev = void 0;
    } else {
      this.tail = void 0;
    }
    h.list = void 0;
    this.length--;
    return res;
  }
  forEach(fn, thisp) {
    thisp = thisp || this;
    for (let walker = this.head, i = 0; !!walker; i++) {
      fn.call(thisp, walker.value, i, this);
      walker = walker.next;
    }
  }
  forEachReverse(fn, thisp) {
    thisp = thisp || this;
    for (let walker = this.tail, i = this.length - 1; !!walker; i--) {
      fn.call(thisp, walker.value, i, this);
      walker = walker.prev;
    }
  }
  get(n) {
    let i = 0;
    let walker = this.head;
    for (; !!walker && i < n; i++) {
      walker = walker.next;
    }
    if (i === n && !!walker) {
      return walker.value;
    }
  }
  getReverse(n) {
    let i = 0;
    let walker = this.tail;
    for (; !!walker && i < n; i++) {
      walker = walker.prev;
    }
    if (i === n && !!walker) {
      return walker.value;
    }
  }
  map(fn, thisp) {
    thisp = thisp || this;
    const res = new _Yallist();
    for (let walker = this.head; !!walker; ) {
      res.push(fn.call(thisp, walker.value, this));
      walker = walker.next;
    }
    return res;
  }
  mapReverse(fn, thisp) {
    thisp = thisp || this;
    var res = new _Yallist();
    for (let walker = this.tail; !!walker; ) {
      res.push(fn.call(thisp, walker.value, this));
      walker = walker.prev;
    }
    return res;
  }
  reduce(fn, initial) {
    let acc;
    let walker = this.head;
    if (arguments.length > 1) {
      acc = initial;
    } else if (this.head) {
      walker = this.head.next;
      acc = this.head.value;
    } else {
      throw new TypeError("Reduce of empty list with no initial value");
    }
    for (var i = 0; !!walker; i++) {
      acc = fn(acc, walker.value, i);
      walker = walker.next;
    }
    return acc;
  }
  reduceReverse(fn, initial) {
    let acc;
    let walker = this.tail;
    if (arguments.length > 1) {
      acc = initial;
    } else if (this.tail) {
      walker = this.tail.prev;
      acc = this.tail.value;
    } else {
      throw new TypeError("Reduce of empty list with no initial value");
    }
    for (let i = this.length - 1; !!walker; i--) {
      acc = fn(acc, walker.value, i);
      walker = walker.prev;
    }
    return acc;
  }
  toArray() {
    const arr = new Array(this.length);
    for (let i = 0, walker = this.head; !!walker; i++) {
      arr[i] = walker.value;
      walker = walker.next;
    }
    return arr;
  }
  toArrayReverse() {
    const arr = new Array(this.length);
    for (let i = 0, walker = this.tail; !!walker; i++) {
      arr[i] = walker.value;
      walker = walker.prev;
    }
    return arr;
  }
  slice(from = 0, to = this.length) {
    if (to < 0) {
      to += this.length;
    }
    if (from < 0) {
      from += this.length;
    }
    const ret = new _Yallist();
    if (to < from || to < 0) {
      return ret;
    }
    if (from < 0) {
      from = 0;
    }
    if (to > this.length) {
      to = this.length;
    }
    let walker = this.head;
    let i = 0;
    for (i = 0; !!walker && i < from; i++) {
      walker = walker.next;
    }
    for (; !!walker && i < to; i++, walker = walker.next) {
      ret.push(walker.value);
    }
    return ret;
  }
  sliceReverse(from = 0, to = this.length) {
    if (to < 0) {
      to += this.length;
    }
    if (from < 0) {
      from += this.length;
    }
    const ret = new _Yallist();
    if (to < from || to < 0) {
      return ret;
    }
    if (from < 0) {
      from = 0;
    }
    if (to > this.length) {
      to = this.length;
    }
    let i = this.length;
    let walker = this.tail;
    for (; !!walker && i > to; i--) {
      walker = walker.prev;
    }
    for (; !!walker && i > from; i--, walker = walker.prev) {
      ret.push(walker.value);
    }
    return ret;
  }
  splice(start, deleteCount = 0, ...nodes) {
    if (start > this.length) {
      start = this.length - 1;
    }
    if (start < 0) {
      start = this.length + start;
    }
    let walker = this.head;
    for (let i = 0; !!walker && i < start; i++) {
      walker = walker.next;
    }
    const ret = [];
    for (let i = 0; !!walker && i < deleteCount; i++) {
      ret.push(walker.value);
      walker = this.removeNode(walker);
    }
    if (!walker) {
      walker = this.tail;
    } else if (walker !== this.tail) {
      walker = walker.prev;
    }
    for (const v of nodes) {
      walker = insertAfter(this, walker, v);
    }
    return ret;
  }
  reverse() {
    const head = this.head;
    const tail = this.tail;
    for (let walker = head; !!walker; walker = walker.prev) {
      const p = walker.prev;
      walker.prev = walker.next;
      walker.next = p;
    }
    this.head = tail;
    this.tail = head;
    return this;
  }
};
function insertAfter(self, node, value) {
  const prev = node;
  const next = node ? node.next : self.head;
  const inserted = new Node(value, prev, next, self);
  if (inserted.next === void 0) {
    self.tail = inserted;
  }
  if (inserted.prev === void 0) {
    self.head = inserted;
  }
  self.length++;
  return inserted;
}
function push(self, item) {
  self.tail = new Node(item, self.tail, void 0, self);
  if (!self.head) {
    self.head = self.tail;
  }
  self.length++;
}
function unshift(self, item) {
  self.head = new Node(item, void 0, self.head, self);
  if (!self.tail) {
    self.tail = self.head;
  }
  self.length++;
}
var Node = class {
  list;
  next;
  prev;
  value;
  constructor(value, prev, next, list2) {
    this.list = list2;
    this.value = value;
    if (prev) {
      prev.next = this;
      this.prev = prev;
    } else {
      this.prev = void 0;
    }
    if (next) {
      next.prev = this;
      this.next = next;
    } else {
      this.next = void 0;
    }
  }
};

// node_modules/tar/dist/esm/header.js
var import_node_path = require("node:path");

// node_modules/tar/dist/esm/large-numbers.js
var encode = (num, buf) => {
  if (!Number.isSafeInteger(num)) {
    throw Error(
      "cannot encode number outside of javascript safe integer range"
    );
  } else if (num < 0) {
    encodeNegative(num, buf);
  } else {
    encodePositive(num, buf);
  }
  return buf;
};
var encodePositive = (num, buf) => {
  buf[0] = 128;
  for (var i = buf.length; i > 1; i--) {
    buf[i - 1] = num & 255;
    num = Math.floor(num / 256);
  }
};
var encodeNegative = (num, buf) => {
  buf[0] = 255;
  var flipped = false;
  num = num * -1;
  for (var i = buf.length; i > 1; i--) {
    var byte = num & 255;
    num = Math.floor(num / 256);
    if (flipped) {
      buf[i - 1] = onesComp(byte);
    } else if (byte === 0) {
      buf[i - 1] = 0;
    } else {
      flipped = true;
      buf[i - 1] = twosComp(byte);
    }
  }
};
var parse = (buf) => {
  const pre = buf[0];
  const value =
    pre === 128
      ? pos(buf.subarray(1, buf.length))
      : pre === 255
      ? twos(buf)
      : null;
  if (value === null) {
    throw Error("invalid base256 encoding");
  }
  if (!Number.isSafeInteger(value)) {
    throw Error("parsed number outside of javascript safe integer range");
  }
  return value;
};
var twos = (buf) => {
  var len = buf.length;
  var sum = 0;
  var flipped = false;
  for (var i = len - 1; i > -1; i--) {
    var byte = Number(buf[i]);
    var f;
    if (flipped) {
      f = onesComp(byte);
    } else if (byte === 0) {
      f = byte;
    } else {
      flipped = true;
      f = twosComp(byte);
    }
    if (f !== 0) {
      sum -= f * Math.pow(256, len - i - 1);
    }
  }
  return sum;
};
var pos = (buf) => {
  var len = buf.length;
  var sum = 0;
  for (var i = len - 1; i > -1; i--) {
    var byte = Number(buf[i]);
    if (byte !== 0) {
      sum += byte * Math.pow(256, len - i - 1);
    }
  }
  return sum;
};
var onesComp = (byte) => (255 ^ byte) & 255;
var twosComp = (byte) => ((255 ^ byte) + 1) & 255;

// node_modules/tar/dist/esm/types.js
var types_exports = {};
__export(types_exports, {
  code: () => code,
  isCode: () => isCode,
  isName: () => isName,
  name: () => name,
});
var isCode = (c) => name.has(c);
var isName = (c) => code.has(c);
var name = /* @__PURE__ */ new Map([
  ["0", "File"],
  // same as File
  ["", "OldFile"],
  ["1", "Link"],
  ["2", "SymbolicLink"],
  // Devices and FIFOs aren't fully supported
  // they are parsed, but skipped when unpacking
  ["3", "CharacterDevice"],
  ["4", "BlockDevice"],
  ["5", "Directory"],
  ["6", "FIFO"],
  // same as File
  ["7", "ContiguousFile"],
  // pax headers
  ["g", "GlobalExtendedHeader"],
  ["x", "ExtendedHeader"],
  // vendor-specific stuff
  // skip
  ["A", "SolarisACL"],
  // like 5, but with data, which should be skipped
  ["D", "GNUDumpDir"],
  // metadata only, skip
  ["I", "Inode"],
  // data = link path of next file
  ["K", "NextFileHasLongLinkpath"],
  // data = path of next file
  ["L", "NextFileHasLongPath"],
  // skip
  ["M", "ContinuationFile"],
  // like L
  ["N", "OldGnuLongPath"],
  // skip
  ["S", "SparseFile"],
  // skip
  ["V", "TapeVolumeHeader"],
  // like x
  ["X", "OldExtendedHeader"],
]);
var code = new Map(Array.from(name).map((kv) => [kv[1], kv[0]]));

// node_modules/tar/dist/esm/header.js
var Header = class {
  cksumValid = false;
  needPax = false;
  nullBlock = false;
  block;
  path;
  mode;
  uid;
  gid;
  size;
  cksum;
  #type = "Unsupported";
  linkpath;
  uname;
  gname;
  devmaj = 0;
  devmin = 0;
  atime;
  ctime;
  mtime;
  charset;
  comment;
  constructor(data, off = 0, ex, gex) {
    if (Buffer.isBuffer(data)) {
      this.decode(data, off || 0, ex, gex);
    } else if (data) {
      this.#slurp(data);
    }
  }
  decode(buf, off, ex, gex) {
    if (!off) {
      off = 0;
    }
    if (!buf || !(buf.length >= off + 512)) {
      throw new Error("need 512 bytes for header");
    }
    this.path = decString(buf, off, 100);
    this.mode = decNumber(buf, off + 100, 8);
    this.uid = decNumber(buf, off + 108, 8);
    this.gid = decNumber(buf, off + 116, 8);
    this.size = decNumber(buf, off + 124, 12);
    this.mtime = decDate(buf, off + 136, 12);
    this.cksum = decNumber(buf, off + 148, 12);
    if (gex) this.#slurp(gex, true);
    if (ex) this.#slurp(ex);
    const t = decString(buf, off + 156, 1);
    if (isCode(t)) {
      this.#type = t || "0";
    }
    if (this.#type === "0" && this.path.slice(-1) === "/") {
      this.#type = "5";
    }
    if (this.#type === "5") {
      this.size = 0;
    }
    this.linkpath = decString(buf, off + 157, 100);
    if (buf.subarray(off + 257, off + 265).toString() === "ustar\x0000") {
      this.uname = decString(buf, off + 265, 32);
      this.gname = decString(buf, off + 297, 32);
      this.devmaj = decNumber(buf, off + 329, 8) ?? 0;
      this.devmin = decNumber(buf, off + 337, 8) ?? 0;
      if (buf[off + 475] !== 0) {
        const prefix = decString(buf, off + 345, 155);
        this.path = prefix + "/" + this.path;
      } else {
        const prefix = decString(buf, off + 345, 130);
        if (prefix) {
          this.path = prefix + "/" + this.path;
        }
        this.atime = decDate(buf, off + 476, 12);
        this.ctime = decDate(buf, off + 488, 12);
      }
    }
    let sum = 8 * 32;
    for (let i = off; i < off + 148; i++) {
      sum += buf[i];
    }
    for (let i = off + 156; i < off + 512; i++) {
      sum += buf[i];
    }
    this.cksumValid = sum === this.cksum;
    if (this.cksum === void 0 && sum === 8 * 32) {
      this.nullBlock = true;
    }
  }
  #slurp(ex, gex = false) {
    Object.assign(
      this,
      Object.fromEntries(
        Object.entries(ex).filter(([k, v]) => {
          return !(
            v === null ||
            v === void 0 ||
            (k === "path" && gex) ||
            (k === "linkpath" && gex) ||
            k === "global"
          );
        })
      )
    );
  }
  encode(buf, off = 0) {
    if (!buf) {
      buf = this.block = Buffer.alloc(512);
    }
    if (this.#type === "Unsupported") {
      this.#type = "0";
    }
    if (!(buf.length >= off + 512)) {
      throw new Error("need 512 bytes for header");
    }
    const prefixSize = this.ctime || this.atime ? 130 : 155;
    const split = splitPrefix(this.path || "", prefixSize);
    const path8 = split[0];
    const prefix = split[1];
    this.needPax = !!split[2];
    this.needPax = encString(buf, off, 100, path8) || this.needPax;
    this.needPax = encNumber(buf, off + 100, 8, this.mode) || this.needPax;
    this.needPax = encNumber(buf, off + 108, 8, this.uid) || this.needPax;
    this.needPax = encNumber(buf, off + 116, 8, this.gid) || this.needPax;
    this.needPax = encNumber(buf, off + 124, 12, this.size) || this.needPax;
    this.needPax = encDate(buf, off + 136, 12, this.mtime) || this.needPax;
    buf[off + 156] = this.#type.charCodeAt(0);
    this.needPax =
      encString(buf, off + 157, 100, this.linkpath) || this.needPax;
    buf.write("ustar\x0000", off + 257, 8);
    this.needPax = encString(buf, off + 265, 32, this.uname) || this.needPax;
    this.needPax = encString(buf, off + 297, 32, this.gname) || this.needPax;
    this.needPax = encNumber(buf, off + 329, 8, this.devmaj) || this.needPax;
    this.needPax = encNumber(buf, off + 337, 8, this.devmin) || this.needPax;
    this.needPax =
      encString(buf, off + 345, prefixSize, prefix) || this.needPax;
    if (buf[off + 475] !== 0) {
      this.needPax = encString(buf, off + 345, 155, prefix) || this.needPax;
    } else {
      this.needPax = encString(buf, off + 345, 130, prefix) || this.needPax;
      this.needPax = encDate(buf, off + 476, 12, this.atime) || this.needPax;
      this.needPax = encDate(buf, off + 488, 12, this.ctime) || this.needPax;
    }
    let sum = 8 * 32;
    for (let i = off; i < off + 148; i++) {
      sum += buf[i];
    }
    for (let i = off + 156; i < off + 512; i++) {
      sum += buf[i];
    }
    this.cksum = sum;
    encNumber(buf, off + 148, 8, this.cksum);
    this.cksumValid = true;
    return this.needPax;
  }
  get type() {
    return this.#type === "Unsupported" ? this.#type : name.get(this.#type);
  }
  get typeKey() {
    return this.#type;
  }
  set type(type) {
    const c = String(code.get(type));
    if (isCode(c) || c === "Unsupported") {
      this.#type = c;
    } else if (isCode(type)) {
      this.#type = type;
    } else {
      throw new TypeError("invalid entry type: " + type);
    }
  }
};
var splitPrefix = (p, prefixSize) => {
  const pathSize = 100;
  let pp = p;
  let prefix = "";
  let ret = void 0;
  const root = import_node_path.posix.parse(p).root || ".";
  if (Buffer.byteLength(pp) < pathSize) {
    ret = [pp, prefix, false];
  } else {
    prefix = import_node_path.posix.dirname(pp);
    pp = import_node_path.posix.basename(pp);
    do {
      if (
        Buffer.byteLength(pp) <= pathSize &&
        Buffer.byteLength(prefix) <= prefixSize
      ) {
        ret = [pp, prefix, false];
      } else if (
        Buffer.byteLength(pp) > pathSize &&
        Buffer.byteLength(prefix) <= prefixSize
      ) {
        ret = [pp.slice(0, pathSize - 1), prefix, true];
      } else {
        pp = import_node_path.posix.join(
          import_node_path.posix.basename(prefix),
          pp
        );
        prefix = import_node_path.posix.dirname(prefix);
      }
    } while (prefix !== root && ret === void 0);
    if (!ret) {
      ret = [p.slice(0, pathSize - 1), "", true];
    }
  }
  return ret;
};
var decString = (buf, off, size) =>
  buf
    .subarray(off, off + size)
    .toString("utf8")
    .replace(/\0.*/, "");
var decDate = (buf, off, size) => numToDate(decNumber(buf, off, size));
var numToDate = (num) => (num === void 0 ? void 0 : new Date(num * 1e3));
var decNumber = (buf, off, size) =>
  Number(buf[off]) & 128
    ? parse(buf.subarray(off, off + size))
    : decSmallNumber(buf, off, size);
var nanUndef = (value) => (isNaN(value) ? void 0 : value);
var decSmallNumber = (buf, off, size) =>
  nanUndef(
    parseInt(
      buf
        .subarray(off, off + size)
        .toString("utf8")
        .replace(/\0.*$/, "")
        .trim(),
      8
    )
  );
var MAXNUM = {
  12: 8589934591,
  8: 2097151,
};
var encNumber = (buf, off, size, num) =>
  num === void 0
    ? false
    : num > MAXNUM[size] || num < 0
    ? (encode(num, buf.subarray(off, off + size)), true)
    : (encSmallNumber(buf, off, size, num), false);
var encSmallNumber = (buf, off, size, num) =>
  buf.write(octalString(num, size), off, size, "ascii");
var octalString = (num, size) => padOctal(Math.floor(num).toString(8), size);
var padOctal = (str, size) =>
  (str.length === size - 1
    ? str
    : new Array(size - str.length - 1).join("0") + str + " ") + "\0";
var encDate = (buf, off, size, date) =>
  date === void 0 ? false : encNumber(buf, off, size, date.getTime() / 1e3);
var NULLS = new Array(156).join("\0");
var encString = (buf, off, size, str) =>
  str === void 0
    ? false
    : (buf.write(str + NULLS, off, size, "utf8"),
      str.length !== Buffer.byteLength(str) || str.length > size);

// node_modules/tar/dist/esm/pax.js
var import_node_path2 = require("node:path");
var Pax = class _Pax {
  atime;
  mtime;
  ctime;
  charset;
  comment;
  gid;
  uid;
  gname;
  uname;
  linkpath;
  dev;
  ino;
  nlink;
  path;
  size;
  mode;
  global;
  constructor(obj, global = false) {
    this.atime = obj.atime;
    this.charset = obj.charset;
    this.comment = obj.comment;
    this.ctime = obj.ctime;
    this.dev = obj.dev;
    this.gid = obj.gid;
    this.global = global;
    this.gname = obj.gname;
    this.ino = obj.ino;
    this.linkpath = obj.linkpath;
    this.mtime = obj.mtime;
    this.nlink = obj.nlink;
    this.path = obj.path;
    this.size = obj.size;
    this.uid = obj.uid;
    this.uname = obj.uname;
  }
  encode() {
    const body = this.encodeBody();
    if (body === "") {
      return Buffer.allocUnsafe(0);
    }
    const bodyLen = Buffer.byteLength(body);
    const bufLen = 512 * Math.ceil(1 + bodyLen / 512);
    const buf = Buffer.allocUnsafe(bufLen);
    for (let i = 0; i < 512; i++) {
      buf[i] = 0;
    }
    new Header({
      // XXX split the path
      // then the path should be PaxHeader + basename, but less than 99,
      // prepend with the dirname
      /* c8 ignore start */
      path: (
        "PaxHeader/" + (0, import_node_path2.basename)(this.path ?? "")
      ).slice(0, 99),
      /* c8 ignore stop */
      mode: this.mode || 420,
      uid: this.uid,
      gid: this.gid,
      size: bodyLen,
      mtime: this.mtime,
      type: this.global ? "GlobalExtendedHeader" : "ExtendedHeader",
      linkpath: "",
      uname: this.uname || "",
      gname: this.gname || "",
      devmaj: 0,
      devmin: 0,
      atime: this.atime,
      ctime: this.ctime,
    }).encode(buf);
    buf.write(body, 512, bodyLen, "utf8");
    for (let i = bodyLen + 512; i < buf.length; i++) {
      buf[i] = 0;
    }
    return buf;
  }
  encodeBody() {
    return (
      this.encodeField("path") +
      this.encodeField("ctime") +
      this.encodeField("atime") +
      this.encodeField("dev") +
      this.encodeField("ino") +
      this.encodeField("nlink") +
      this.encodeField("charset") +
      this.encodeField("comment") +
      this.encodeField("gid") +
      this.encodeField("gname") +
      this.encodeField("linkpath") +
      this.encodeField("mtime") +
      this.encodeField("size") +
      this.encodeField("uid") +
      this.encodeField("uname")
    );
  }
  encodeField(field) {
    if (this[field] === void 0) {
      return "";
    }
    const r = this[field];
    const v = r instanceof Date ? r.getTime() / 1e3 : r;
    const s =
      " " +
      (field === "dev" || field === "ino" || field === "nlink"
        ? "SCHILY."
        : "") +
      field +
      "=" +
      v +
      "\n";
    const byteLen = Buffer.byteLength(s);
    let digits = Math.floor(Math.log(byteLen) / Math.log(10)) + 1;
    if (byteLen + digits >= Math.pow(10, digits)) {
      digits += 1;
    }
    const len = digits + byteLen;
    return len + s;
  }
  static parse(str, ex, g = false) {
    return new _Pax(merge(parseKV(str), ex), g);
  }
};
var merge = (a, b) => (b ? Object.assign({}, b, a) : a);
var parseKV = (str) =>
  str
    .replace(/\n$/, "")
    .split("\n")
    .reduce(parseKVLine, /* @__PURE__ */ Object.create(null));
var parseKVLine = (set, line) => {
  const n = parseInt(line, 10);
  if (n !== Buffer.byteLength(line) + 1) {
    return set;
  }
  line = line.slice((n + " ").length);
  const kv = line.split("=");
  const r = kv.shift();
  if (!r) {
    return set;
  }
  const k = r.replace(/^SCHILY\.(dev|ino|nlink)/, "$1");
  const v = kv.join("=");
  set[k] = /^([A-Z]+\.)?([mac]|birth|creation)time$/.test(k)
    ? new Date(Number(v) * 1e3)
    : /^[0-9]+$/.test(v)
    ? +v
    : v;
  return set;
};

// node_modules/tar/dist/esm/normalize-windows-path.js
var platform = process.env.TESTING_TAR_FAKE_PLATFORM || process.platform;
var normalizeWindowsPath =
  platform !== "win32" ? (p) => p : (p) => p && p.replace(/\\/g, "/");

// node_modules/tar/dist/esm/read-entry.js
var ReadEntry = class extends Minipass {
  extended;
  globalExtended;
  header;
  startBlockSize;
  blockRemain;
  remain;
  type;
  meta = false;
  ignore = false;
  path;
  mode;
  uid;
  gid;
  uname;
  gname;
  size = 0;
  mtime;
  atime;
  ctime;
  linkpath;
  dev;
  ino;
  nlink;
  invalid = false;
  absolute;
  unsupported = false;
  constructor(header, ex, gex) {
    super({});
    this.pause();
    this.extended = ex;
    this.globalExtended = gex;
    this.header = header;
    this.remain = header.size ?? 0;
    this.startBlockSize = 512 * Math.ceil(this.remain / 512);
    this.blockRemain = this.startBlockSize;
    this.type = header.type;
    switch (this.type) {
      case "File":
      case "OldFile":
      case "Link":
      case "SymbolicLink":
      case "CharacterDevice":
      case "BlockDevice":
      case "Directory":
      case "FIFO":
      case "ContiguousFile":
      case "GNUDumpDir":
        break;
      case "NextFileHasLongLinkpath":
      case "NextFileHasLongPath":
      case "OldGnuLongPath":
      case "GlobalExtendedHeader":
      case "ExtendedHeader":
      case "OldExtendedHeader":
        this.meta = true;
        break;
      // NOTE: gnutar and bsdtar treat unrecognized types as 'File'
      // it may be worth doing the same, but with a warning.
      default:
        this.ignore = true;
    }
    if (!header.path) {
      throw new Error("no path provided for tar.ReadEntry");
    }
    this.path = normalizeWindowsPath(header.path);
    this.mode = header.mode;
    if (this.mode) {
      this.mode = this.mode & 4095;
    }
    this.uid = header.uid;
    this.gid = header.gid;
    this.uname = header.uname;
    this.gname = header.gname;
    this.size = this.remain;
    this.mtime = header.mtime;
    this.atime = header.atime;
    this.ctime = header.ctime;
    this.linkpath = header.linkpath
      ? normalizeWindowsPath(header.linkpath)
      : void 0;
    this.uname = header.uname;
    this.gname = header.gname;
    if (ex) {
      this.#slurp(ex);
    }
    if (gex) {
      this.#slurp(gex, true);
    }
  }
  write(data) {
    const writeLen = data.length;
    if (writeLen > this.blockRemain) {
      throw new Error("writing more to entry than is appropriate");
    }
    const r = this.remain;
    const br = this.blockRemain;
    this.remain = Math.max(0, r - writeLen);
    this.blockRemain = Math.max(0, br - writeLen);
    if (this.ignore) {
      return true;
    }
    if (r >= writeLen) {
      return super.write(data);
    }
    return super.write(data.subarray(0, r));
  }
  #slurp(ex, gex = false) {
    if (ex.path) ex.path = normalizeWindowsPath(ex.path);
    if (ex.linkpath) ex.linkpath = normalizeWindowsPath(ex.linkpath);
    Object.assign(
      this,
      Object.fromEntries(
        Object.entries(ex).filter(([k, v]) => {
          return !(v === null || v === void 0 || (k === "path" && gex));
        })
      )
    );
  }
};

// node_modules/tar/dist/esm/warn-method.js
var warnMethod = (self, code2, message, data = {}) => {
  if (self.file) {
    data.file = self.file;
  }
  if (self.cwd) {
    data.cwd = self.cwd;
  }
  data.code = (message instanceof Error && message.code) || code2;
  data.tarCode = code2;
  if (!self.strict && data.recoverable !== false) {
    if (message instanceof Error) {
      data = Object.assign(message, data);
      message = message.message;
    }
    self.emit("warn", code2, message, data);
  } else if (message instanceof Error) {
    self.emit("error", Object.assign(message, data));
  } else {
    self.emit("error", Object.assign(new Error(`${code2}: ${message}`), data));
  }
};

// node_modules/tar/dist/esm/parse.js
var maxMetaEntrySize = 1024 * 1024;
var gzipHeader = Buffer.from([31, 139]);
var STATE = Symbol("state");
var WRITEENTRY = Symbol("writeEntry");
var READENTRY = Symbol("readEntry");
var NEXTENTRY = Symbol("nextEntry");
var PROCESSENTRY = Symbol("processEntry");
var EX = Symbol("extendedHeader");
var GEX = Symbol("globalExtendedHeader");
var META = Symbol("meta");
var EMITMETA = Symbol("emitMeta");
var BUFFER2 = Symbol("buffer");
var QUEUE = Symbol("queue");
var ENDED = Symbol("ended");
var EMITTEDEND = Symbol("emittedEnd");
var EMIT = Symbol("emit");
var UNZIP = Symbol("unzip");
var CONSUMECHUNK = Symbol("consumeChunk");
var CONSUMECHUNKSUB = Symbol("consumeChunkSub");
var CONSUMEBODY = Symbol("consumeBody");
var CONSUMEMETA = Symbol("consumeMeta");
var CONSUMEHEADER = Symbol("consumeHeader");
var CONSUMING = Symbol("consuming");
var BUFFERCONCAT = Symbol("bufferConcat");
var MAYBEEND = Symbol("maybeEnd");
var WRITING = Symbol("writing");
var ABORTED2 = Symbol("aborted");
var DONE = Symbol("onDone");
var SAW_VALID_ENTRY = Symbol("sawValidEntry");
var SAW_NULL_BLOCK = Symbol("sawNullBlock");
var SAW_EOF = Symbol("sawEOF");
var CLOSESTREAM = Symbol("closeStream");
var noop2 = () => true;
var Parser = class extends import_events2.EventEmitter {
  file;
  strict;
  maxMetaEntrySize;
  filter;
  brotli;
  writable = true;
  readable = false;
  [QUEUE] = new Yallist();
  [BUFFER2];
  [READENTRY];
  [WRITEENTRY];
  [STATE] = "begin";
  [META] = "";
  [EX];
  [GEX];
  [ENDED] = false;
  [UNZIP];
  [ABORTED2] = false;
  [SAW_VALID_ENTRY];
  [SAW_NULL_BLOCK] = false;
  [SAW_EOF] = false;
  [WRITING] = false;
  [CONSUMING] = false;
  [EMITTEDEND] = false;
  constructor(opt = {}) {
    super();
    this.file = opt.file || "";
    this.on(DONE, () => {
      if (this[STATE] === "begin" || this[SAW_VALID_ENTRY] === false) {
        this.warn("TAR_BAD_ARCHIVE", "Unrecognized archive format");
      }
    });
    if (opt.ondone) {
      this.on(DONE, opt.ondone);
    } else {
      this.on(DONE, () => {
        this.emit("prefinish");
        this.emit("finish");
        this.emit("end");
      });
    }
    this.strict = !!opt.strict;
    this.maxMetaEntrySize = opt.maxMetaEntrySize || maxMetaEntrySize;
    this.filter = typeof opt.filter === "function" ? opt.filter : noop2;
    const isTBR =
      opt.file && (opt.file.endsWith(".tar.br") || opt.file.endsWith(".tbr"));
    this.brotli =
      !opt.gzip && opt.brotli !== void 0 ? opt.brotli : isTBR ? void 0 : false;
    this.on("end", () => this[CLOSESTREAM]());
    if (typeof opt.onwarn === "function") {
      this.on("warn", opt.onwarn);
    }
    if (typeof opt.onReadEntry === "function") {
      this.on("entry", opt.onReadEntry);
    }
  }
  warn(code2, message, data = {}) {
    warnMethod(this, code2, message, data);
  }
  [CONSUMEHEADER](chunk, position) {
    if (this[SAW_VALID_ENTRY] === void 0) {
      this[SAW_VALID_ENTRY] = false;
    }
    let header;
    try {
      header = new Header(chunk, position, this[EX], this[GEX]);
    } catch (er) {
      return this.warn("TAR_ENTRY_INVALID", er);
    }
    if (header.nullBlock) {
      if (this[SAW_NULL_BLOCK]) {
        this[SAW_EOF] = true;
        if (this[STATE] === "begin") {
          this[STATE] = "header";
        }
        this[EMIT]("eof");
      } else {
        this[SAW_NULL_BLOCK] = true;
        this[EMIT]("nullBlock");
      }
    } else {
      this[SAW_NULL_BLOCK] = false;
      if (!header.cksumValid) {
        this.warn("TAR_ENTRY_INVALID", "checksum failure", { header });
      } else if (!header.path) {
        this.warn("TAR_ENTRY_INVALID", "path is required", { header });
      } else {
        const type = header.type;
        if (/^(Symbolic)?Link$/.test(type) && !header.linkpath) {
          this.warn("TAR_ENTRY_INVALID", "linkpath required", {
            header,
          });
        } else if (
          !/^(Symbolic)?Link$/.test(type) &&
          !/^(Global)?ExtendedHeader$/.test(type) &&
          header.linkpath
        ) {
          this.warn("TAR_ENTRY_INVALID", "linkpath forbidden", {
            header,
          });
        } else {
          const entry = (this[WRITEENTRY] = new ReadEntry(
            header,
            this[EX],
            this[GEX]
          ));
          if (!this[SAW_VALID_ENTRY]) {
            if (entry.remain) {
              const onend = () => {
                if (!entry.invalid) {
                  this[SAW_VALID_ENTRY] = true;
                }
              };
              entry.on("end", onend);
            } else {
              this[SAW_VALID_ENTRY] = true;
            }
          }
          if (entry.meta) {
            if (entry.size > this.maxMetaEntrySize) {
              entry.ignore = true;
              this[EMIT]("ignoredEntry", entry);
              this[STATE] = "ignore";
              entry.resume();
            } else if (entry.size > 0) {
              this[META] = "";
              entry.on("data", (c) => (this[META] += c));
              this[STATE] = "meta";
            }
          } else {
            this[EX] = void 0;
            entry.ignore = entry.ignore || !this.filter(entry.path, entry);
            if (entry.ignore) {
              this[EMIT]("ignoredEntry", entry);
              this[STATE] = entry.remain ? "ignore" : "header";
              entry.resume();
            } else {
              if (entry.remain) {
                this[STATE] = "body";
              } else {
                this[STATE] = "header";
                entry.end();
              }
              if (!this[READENTRY]) {
                this[QUEUE].push(entry);
                this[NEXTENTRY]();
              } else {
                this[QUEUE].push(entry);
              }
            }
          }
        }
      }
    }
  }
  [CLOSESTREAM]() {
    queueMicrotask(() => this.emit("close"));
  }
  [PROCESSENTRY](entry) {
    let go = true;
    if (!entry) {
      this[READENTRY] = void 0;
      go = false;
    } else if (Array.isArray(entry)) {
      const [ev, ...args] = entry;
      this.emit(ev, ...args);
    } else {
      this[READENTRY] = entry;
      this.emit("entry", entry);
      if (!entry.emittedEnd) {
        entry.on("end", () => this[NEXTENTRY]());
        go = false;
      }
    }
    return go;
  }
  [NEXTENTRY]() {
    do {} while (this[PROCESSENTRY](this[QUEUE].shift()));
    if (!this[QUEUE].length) {
      const re = this[READENTRY];
      const drainNow = !re || re.flowing || re.size === re.remain;
      if (drainNow) {
        if (!this[WRITING]) {
          this.emit("drain");
        }
      } else {
        re.once("drain", () => this.emit("drain"));
      }
    }
  }
  [CONSUMEBODY](chunk, position) {
    const entry = this[WRITEENTRY];
    if (!entry) {
      throw new Error("attempt to consume body without entry??");
    }
    const br = entry.blockRemain ?? 0;
    const c =
      br >= chunk.length && position === 0
        ? chunk
        : chunk.subarray(position, position + br);
    entry.write(c);
    if (!entry.blockRemain) {
      this[STATE] = "header";
      this[WRITEENTRY] = void 0;
      entry.end();
    }
    return c.length;
  }
  [CONSUMEMETA](chunk, position) {
    const entry = this[WRITEENTRY];
    const ret = this[CONSUMEBODY](chunk, position);
    if (!this[WRITEENTRY] && entry) {
      this[EMITMETA](entry);
    }
    return ret;
  }
  [EMIT](ev, data, extra) {
    if (!this[QUEUE].length && !this[READENTRY]) {
      this.emit(ev, data, extra);
    } else {
      this[QUEUE].push([ev, data, extra]);
    }
  }
  [EMITMETA](entry) {
    this[EMIT]("meta", this[META]);
    switch (entry.type) {
      case "ExtendedHeader":
      case "OldExtendedHeader":
        this[EX] = Pax.parse(this[META], this[EX], false);
        break;
      case "GlobalExtendedHeader":
        this[GEX] = Pax.parse(this[META], this[GEX], true);
        break;
      case "NextFileHasLongPath":
      case "OldGnuLongPath": {
        const ex = this[EX] ?? /* @__PURE__ */ Object.create(null);
        this[EX] = ex;
        ex.path = this[META].replace(/\0.*/, "");
        break;
      }
      case "NextFileHasLongLinkpath": {
        const ex = this[EX] || /* @__PURE__ */ Object.create(null);
        this[EX] = ex;
        ex.linkpath = this[META].replace(/\0.*/, "");
        break;
      }
      /* c8 ignore start */
      default:
        throw new Error("unknown meta: " + entry.type);
    }
  }
  abort(error) {
    this[ABORTED2] = true;
    this.emit("abort", error);
    this.warn("TAR_ABORT", error, { recoverable: false });
  }
  write(chunk, encoding, cb) {
    if (typeof encoding === "function") {
      cb = encoding;
      encoding = void 0;
    }
    if (typeof chunk === "string") {
      chunk = Buffer.from(
        chunk,
        /* c8 ignore next */
        typeof encoding === "string" ? encoding : "utf8"
      );
    }
    if (this[ABORTED2]) {
      cb?.();
      return false;
    }
    const needSniff =
      this[UNZIP] === void 0 ||
      (this.brotli === void 0 && this[UNZIP] === false);
    if (needSniff && chunk) {
      if (this[BUFFER2]) {
        chunk = Buffer.concat([this[BUFFER2], chunk]);
        this[BUFFER2] = void 0;
      }
      if (chunk.length < gzipHeader.length) {
        this[BUFFER2] = chunk;
        cb?.();
        return true;
      }
      for (let i = 0; this[UNZIP] === void 0 && i < gzipHeader.length; i++) {
        if (chunk[i] !== gzipHeader[i]) {
          this[UNZIP] = false;
        }
      }
      const maybeBrotli = this.brotli === void 0;
      if (this[UNZIP] === false && maybeBrotli) {
        if (chunk.length < 512) {
          if (this[ENDED]) {
            this.brotli = true;
          } else {
            this[BUFFER2] = chunk;
            cb?.();
            return true;
          }
        } else {
          try {
            new Header(chunk.subarray(0, 512));
            this.brotli = false;
          } catch (_) {
            this.brotli = true;
          }
        }
      }
      if (this[UNZIP] === void 0 || (this[UNZIP] === false && this.brotli)) {
        const ended = this[ENDED];
        this[ENDED] = false;
        this[UNZIP] =
          this[UNZIP] === void 0 ? new Unzip({}) : new BrotliDecompress({});
        this[UNZIP].on("data", (chunk2) => this[CONSUMECHUNK](chunk2));
        this[UNZIP].on("error", (er) => this.abort(er));
        this[UNZIP].on("end", () => {
          this[ENDED] = true;
          this[CONSUMECHUNK]();
        });
        this[WRITING] = true;
        const ret2 = !!this[UNZIP][ended ? "end" : "write"](chunk);
        this[WRITING] = false;
        cb?.();
        return ret2;
      }
    }
    this[WRITING] = true;
    if (this[UNZIP]) {
      this[UNZIP].write(chunk);
    } else {
      this[CONSUMECHUNK](chunk);
    }
    this[WRITING] = false;
    const ret = this[QUEUE].length
      ? false
      : this[READENTRY]
      ? this[READENTRY].flowing
      : true;
    if (!ret && !this[QUEUE].length) {
      this[READENTRY]?.once("drain", () => this.emit("drain"));
    }
    cb?.();
    return ret;
  }
  [BUFFERCONCAT](c) {
    if (c && !this[ABORTED2]) {
      this[BUFFER2] = this[BUFFER2] ? Buffer.concat([this[BUFFER2], c]) : c;
    }
  }
  [MAYBEEND]() {
    if (
      this[ENDED] &&
      !this[EMITTEDEND] &&
      !this[ABORTED2] &&
      !this[CONSUMING]
    ) {
      this[EMITTEDEND] = true;
      const entry = this[WRITEENTRY];
      if (entry && entry.blockRemain) {
        const have = this[BUFFER2] ? this[BUFFER2].length : 0;
        this.warn(
          "TAR_BAD_ARCHIVE",
          `Truncated input (needed ${entry.blockRemain} more bytes, only ${have} available)`,
          { entry }
        );
        if (this[BUFFER2]) {
          entry.write(this[BUFFER2]);
        }
        entry.end();
      }
      this[EMIT](DONE);
    }
  }
  [CONSUMECHUNK](chunk) {
    if (this[CONSUMING] && chunk) {
      this[BUFFERCONCAT](chunk);
    } else if (!chunk && !this[BUFFER2]) {
      this[MAYBEEND]();
    } else if (chunk) {
      this[CONSUMING] = true;
      if (this[BUFFER2]) {
        this[BUFFERCONCAT](chunk);
        const c = this[BUFFER2];
        this[BUFFER2] = void 0;
        this[CONSUMECHUNKSUB](c);
      } else {
        this[CONSUMECHUNKSUB](chunk);
      }
      while (
        this[BUFFER2] &&
        this[BUFFER2]?.length >= 512 &&
        !this[ABORTED2] &&
        !this[SAW_EOF]
      ) {
        const c = this[BUFFER2];
        this[BUFFER2] = void 0;
        this[CONSUMECHUNKSUB](c);
      }
      this[CONSUMING] = false;
    }
    if (!this[BUFFER2] || this[ENDED]) {
      this[MAYBEEND]();
    }
  }
  [CONSUMECHUNKSUB](chunk) {
    let position = 0;
    const length = chunk.length;
    while (position + 512 <= length && !this[ABORTED2] && !this[SAW_EOF]) {
      switch (this[STATE]) {
        case "begin":
        case "header":
          this[CONSUMEHEADER](chunk, position);
          position += 512;
          break;
        case "ignore":
        case "body":
          position += this[CONSUMEBODY](chunk, position);
          break;
        case "meta":
          position += this[CONSUMEMETA](chunk, position);
          break;
        /* c8 ignore start */
        default:
          throw new Error("invalid state: " + this[STATE]);
      }
    }
    if (position < length) {
      if (this[BUFFER2]) {
        this[BUFFER2] = Buffer.concat([
          chunk.subarray(position),
          this[BUFFER2],
        ]);
      } else {
        this[BUFFER2] = chunk.subarray(position);
      }
    }
  }
  end(chunk, encoding, cb) {
    if (typeof chunk === "function") {
      cb = chunk;
      encoding = void 0;
      chunk = void 0;
    }
    if (typeof encoding === "function") {
      cb = encoding;
      encoding = void 0;
    }
    if (typeof chunk === "string") {
      chunk = Buffer.from(chunk, encoding);
    }
    if (cb) this.once("finish", cb);
    if (!this[ABORTED2]) {
      if (this[UNZIP]) {
        if (chunk) this[UNZIP].write(chunk);
        this[UNZIP].end();
      } else {
        this[ENDED] = true;
        if (this.brotli === void 0) chunk = chunk || Buffer.alloc(0);
        if (chunk) this.write(chunk);
        this[MAYBEEND]();
      }
    }
    return this;
  }
};

// node_modules/tar/dist/esm/strip-trailing-slashes.js
var stripTrailingSlashes = (str) => {
  let i = str.length - 1;
  let slashesStart = -1;
  while (i > -1 && str.charAt(i) === "/") {
    slashesStart = i;
    i--;
  }
  return slashesStart === -1 ? str : str.slice(0, slashesStart);
};

// node_modules/tar/dist/esm/list.js
var onReadEntryFunction = (opt) => {
  const onReadEntry = opt.onReadEntry;
  opt.onReadEntry = onReadEntry
    ? (e) => {
        onReadEntry(e);
        e.resume();
      }
    : (e) => e.resume();
};
var filesFilter = (opt, files) => {
  const map = new Map(files.map((f) => [stripTrailingSlashes(f), true]));
  const filter = opt.filter;
  const mapHas = (file, r = "") => {
    const root = r || (0, import_path.parse)(file).root || ".";
    let ret;
    if (file === root) ret = false;
    else {
      const m = map.get(file);
      if (m !== void 0) {
        ret = m;
      } else {
        ret = mapHas((0, import_path.dirname)(file), root);
      }
    }
    map.set(file, ret);
    return ret;
  };
  opt.filter = filter
    ? (file, entry) => filter(file, entry) && mapHas(stripTrailingSlashes(file))
    : (file) => mapHas(stripTrailingSlashes(file));
};
var listFileSync = (opt) => {
  const p = new Parser(opt);
  const file = opt.file;
  let fd;
  try {
    const stat2 = import_node_fs.default.statSync(file);
    const readSize = opt.maxReadSize || 16 * 1024 * 1024;
    if (stat2.size < readSize) {
      p.end(import_node_fs.default.readFileSync(file));
    } else {
      let pos2 = 0;
      const buf = Buffer.allocUnsafe(readSize);
      fd = import_node_fs.default.openSync(file, "r");
      while (pos2 < stat2.size) {
        const bytesRead = import_node_fs.default.readSync(
          fd,
          buf,
          0,
          readSize,
          pos2
        );
        pos2 += bytesRead;
        p.write(buf.subarray(0, bytesRead));
      }
      p.end();
    }
  } finally {
    if (typeof fd === "number") {
      try {
        import_node_fs.default.closeSync(fd);
      } catch (er) {}
    }
  }
};
var listFile = (opt, _files) => {
  const parse5 = new Parser(opt);
  const readSize = opt.maxReadSize || 16 * 1024 * 1024;
  const file = opt.file;
  const p = new Promise((resolve2, reject) => {
    parse5.on("error", reject);
    parse5.on("end", resolve2);
    import_node_fs.default.stat(file, (er, stat2) => {
      if (er) {
        reject(er);
      } else {
        const stream = new ReadStream(file, {
          readSize,
          size: stat2.size,
        });
        stream.on("error", reject);
        stream.pipe(parse5);
      }
    });
  });
  return p;
};
var list = makeCommand(
  listFileSync,
  listFile,
  (opt) => new Parser(opt),
  (opt) => new Parser(opt),
  (opt, files) => {
    if (files?.length) filesFilter(opt, files);
    if (!opt.noResume) onReadEntryFunction(opt);
  }
);

// node_modules/tar/dist/esm/pack.js
var import_fs3 = __toESM(require("fs"), 1);

// node_modules/tar/dist/esm/write-entry.js
var import_fs2 = __toESM(require("fs"), 1);
var import_path2 = __toESM(require("path"), 1);

// node_modules/tar/dist/esm/mode-fix.js
var modeFix = (mode, isDir, portable) => {
  mode &= 4095;
  if (portable) {
    mode = (mode | 384) & ~18;
  }
  if (isDir) {
    if (mode & 256) {
      mode |= 64;
    }
    if (mode & 32) {
      mode |= 8;
    }
    if (mode & 4) {
      mode |= 1;
    }
  }
  return mode;
};

// node_modules/tar/dist/esm/strip-absolute-path.js
var import_node_path3 = require("node:path");
var { isAbsolute, parse: parse3 } = import_node_path3.win32;
var stripAbsolutePath = (path8) => {
  let r = "";
  let parsed = parse3(path8);
  while (isAbsolute(path8) || parsed.root) {
    const root =
      path8.charAt(0) === "/" && path8.slice(0, 4) !== "//?/"
        ? "/"
        : parsed.root;
    path8 = path8.slice(root.length);
    r += root;
    parsed = parse3(path8);
  }
  return [r, path8];
};

// node_modules/tar/dist/esm/winchars.js
var raw = ["|", "<", ">", "?", ":"];
var win = raw.map((char) => String.fromCharCode(61440 + char.charCodeAt(0)));
var toWin = new Map(raw.map((char, i) => [char, win[i]]));
var toRaw = new Map(win.map((char, i) => [char, raw[i]]));
var encode2 = (s) => raw.reduce((s2, c) => s2.split(c).join(toWin.get(c)), s);
var decode = (s) => win.reduce((s2, c) => s2.split(c).join(toRaw.get(c)), s);

// node_modules/tar/dist/esm/write-entry.js
var prefixPath = (path8, prefix) => {
  if (!prefix) {
    return normalizeWindowsPath(path8);
  }
  path8 = normalizeWindowsPath(path8).replace(/^\.(\/|$)/, "");
  return stripTrailingSlashes(prefix) + "/" + path8;
};
var maxReadSize = 16 * 1024 * 1024;
var PROCESS = Symbol("process");
var FILE = Symbol("file");
var DIRECTORY = Symbol("directory");
var SYMLINK = Symbol("symlink");
var HARDLINK = Symbol("hardlink");
var HEADER = Symbol("header");
var READ2 = Symbol("read");
var LSTAT = Symbol("lstat");
var ONLSTAT = Symbol("onlstat");
var ONREAD = Symbol("onread");
var ONREADLINK = Symbol("onreadlink");
var OPENFILE = Symbol("openfile");
var ONOPENFILE = Symbol("onopenfile");
var CLOSE = Symbol("close");
var MODE = Symbol("mode");
var AWAITDRAIN = Symbol("awaitDrain");
var ONDRAIN = Symbol("ondrain");
var PREFIX = Symbol("prefix");
var WriteEntry = class extends Minipass {
  path;
  portable;
  myuid = (process.getuid && process.getuid()) || 0;
  // until node has builtin pwnam functions, this'll have to do
  myuser = process.env.USER || "";
  maxReadSize;
  linkCache;
  statCache;
  preservePaths;
  cwd;
  strict;
  mtime;
  noPax;
  noMtime;
  prefix;
  fd;
  blockLen = 0;
  blockRemain = 0;
  buf;
  pos = 0;
  remain = 0;
  length = 0;
  offset = 0;
  win32;
  absolute;
  header;
  type;
  linkpath;
  stat;
  onWriteEntry;
  #hadError = false;
  constructor(p, opt_ = {}) {
    const opt = dealias(opt_);
    super();
    this.path = normalizeWindowsPath(p);
    this.portable = !!opt.portable;
    this.maxReadSize = opt.maxReadSize || maxReadSize;
    this.linkCache = opt.linkCache || /* @__PURE__ */ new Map();
    this.statCache = opt.statCache || /* @__PURE__ */ new Map();
    this.preservePaths = !!opt.preservePaths;
    this.cwd = normalizeWindowsPath(opt.cwd || process.cwd());
    this.strict = !!opt.strict;
    this.noPax = !!opt.noPax;
    this.noMtime = !!opt.noMtime;
    this.mtime = opt.mtime;
    this.prefix = opt.prefix ? normalizeWindowsPath(opt.prefix) : void 0;
    this.onWriteEntry = opt.onWriteEntry;
    if (typeof opt.onwarn === "function") {
      this.on("warn", opt.onwarn);
    }
    let pathWarn = false;
    if (!this.preservePaths) {
      const [root, stripped] = stripAbsolutePath(this.path);
      if (root && typeof stripped === "string") {
        this.path = stripped;
        pathWarn = root;
      }
    }
    this.win32 = !!opt.win32 || process.platform === "win32";
    if (this.win32) {
      this.path = decode(this.path.replace(/\\/g, "/"));
      p = p.replace(/\\/g, "/");
    }
    this.absolute = normalizeWindowsPath(
      opt.absolute || import_path2.default.resolve(this.cwd, p)
    );
    if (this.path === "") {
      this.path = "./";
    }
    if (pathWarn) {
      this.warn("TAR_ENTRY_INFO", `stripping ${pathWarn} from absolute path`, {
        entry: this,
        path: pathWarn + this.path,
      });
    }
    const cs = this.statCache.get(this.absolute);
    if (cs) {
      this[ONLSTAT](cs);
    } else {
      this[LSTAT]();
    }
  }
  warn(code2, message, data = {}) {
    return warnMethod(this, code2, message, data);
  }
  emit(ev, ...data) {
    if (ev === "error") {
      this.#hadError = true;
    }
    return super.emit(ev, ...data);
  }
  [LSTAT]() {
    import_fs2.default.lstat(this.absolute, (er, stat2) => {
      if (er) {
        return this.emit("error", er);
      }
      this[ONLSTAT](stat2);
    });
  }
  [ONLSTAT](stat2) {
    this.statCache.set(this.absolute, stat2);
    this.stat = stat2;
    if (!stat2.isFile()) {
      stat2.size = 0;
    }
    this.type = getType(stat2);
    this.emit("stat", stat2);
    this[PROCESS]();
  }
  [PROCESS]() {
    switch (this.type) {
      case "File":
        return this[FILE]();
      case "Directory":
        return this[DIRECTORY]();
      case "SymbolicLink":
        return this[SYMLINK]();
      // unsupported types are ignored.
      default:
        return this.end();
    }
  }
  [MODE](mode) {
    return modeFix(mode, this.type === "Directory", this.portable);
  }
  [PREFIX](path8) {
    return prefixPath(path8, this.prefix);
  }
  [HEADER]() {
    if (!this.stat) {
      throw new Error("cannot write header before stat");
    }
    if (this.type === "Directory" && this.portable) {
      this.noMtime = true;
    }
    this.onWriteEntry?.(this);
    this.header = new Header({
      path: this[PREFIX](this.path),
      // only apply the prefix to hard links.
      linkpath:
        this.type === "Link" && this.linkpath !== void 0
          ? this[PREFIX](this.linkpath)
          : this.linkpath,
      // only the permissions and setuid/setgid/sticky bitflags
      // not the higher-order bits that specify file type
      mode: this[MODE](this.stat.mode),
      uid: this.portable ? void 0 : this.stat.uid,
      gid: this.portable ? void 0 : this.stat.gid,
      size: this.stat.size,
      mtime: this.noMtime ? void 0 : this.mtime || this.stat.mtime,
      /* c8 ignore next */
      type: this.type === "Unsupported" ? void 0 : this.type,
      uname: this.portable
        ? void 0
        : this.stat.uid === this.myuid
        ? this.myuser
        : "",
      atime: this.portable ? void 0 : this.stat.atime,
      ctime: this.portable ? void 0 : this.stat.ctime,
    });
    if (this.header.encode() && !this.noPax) {
      super.write(
        new Pax({
          atime: this.portable ? void 0 : this.header.atime,
          ctime: this.portable ? void 0 : this.header.ctime,
          gid: this.portable ? void 0 : this.header.gid,
          mtime: this.noMtime ? void 0 : this.mtime || this.header.mtime,
          path: this[PREFIX](this.path),
          linkpath:
            this.type === "Link" && this.linkpath !== void 0
              ? this[PREFIX](this.linkpath)
              : this.linkpath,
          size: this.header.size,
          uid: this.portable ? void 0 : this.header.uid,
          uname: this.portable ? void 0 : this.header.uname,
          dev: this.portable ? void 0 : this.stat.dev,
          ino: this.portable ? void 0 : this.stat.ino,
          nlink: this.portable ? void 0 : this.stat.nlink,
        }).encode()
      );
    }
    const block = this.header?.block;
    if (!block) {
      throw new Error("failed to encode header");
    }
    super.write(block);
  }
  [DIRECTORY]() {
    if (!this.stat) {
      throw new Error("cannot create directory entry without stat");
    }
    if (this.path.slice(-1) !== "/") {
      this.path += "/";
    }
    this.stat.size = 0;
    this[HEADER]();
    this.end();
  }
  [SYMLINK]() {
    import_fs2.default.readlink(this.absolute, (er, linkpath) => {
      if (er) {
        return this.emit("error", er);
      }
      this[ONREADLINK](linkpath);
    });
  }
  [ONREADLINK](linkpath) {
    this.linkpath = normalizeWindowsPath(linkpath);
    this[HEADER]();
    this.end();
  }
  [HARDLINK](linkpath) {
    if (!this.stat) {
      throw new Error("cannot create link entry without stat");
    }
    this.type = "Link";
    this.linkpath = normalizeWindowsPath(
      import_path2.default.relative(this.cwd, linkpath)
    );
    this.stat.size = 0;
    this[HEADER]();
    this.end();
  }
  [FILE]() {
    if (!this.stat) {
      throw new Error("cannot create file entry without stat");
    }
    if (this.stat.nlink > 1) {
      const linkKey = `${this.stat.dev}:${this.stat.ino}`;
      const linkpath = this.linkCache.get(linkKey);
      if (linkpath?.indexOf(this.cwd) === 0) {
        return this[HARDLINK](linkpath);
      }
      this.linkCache.set(linkKey, this.absolute);
    }
    this[HEADER]();
    if (this.stat.size === 0) {
      return this.end();
    }
    this[OPENFILE]();
  }
  [OPENFILE]() {
    import_fs2.default.open(this.absolute, "r", (er, fd) => {
      if (er) {
        return this.emit("error", er);
      }
      this[ONOPENFILE](fd);
    });
  }
  [ONOPENFILE](fd) {
    this.fd = fd;
    if (this.#hadError) {
      return this[CLOSE]();
    }
    if (!this.stat) {
      throw new Error("should stat before calling onopenfile");
    }
    this.blockLen = 512 * Math.ceil(this.stat.size / 512);
    this.blockRemain = this.blockLen;
    const bufLen = Math.min(this.blockLen, this.maxReadSize);
    this.buf = Buffer.allocUnsafe(bufLen);
    this.offset = 0;
    this.pos = 0;
    this.remain = this.stat.size;
    this.length = this.buf.length;
    this[READ2]();
  }
  [READ2]() {
    const { fd, buf, offset, length, pos: pos2 } = this;
    if (fd === void 0 || buf === void 0) {
      throw new Error("cannot read file without first opening");
    }
    import_fs2.default.read(fd, buf, offset, length, pos2, (er, bytesRead) => {
      if (er) {
        return this[CLOSE](() => this.emit("error", er));
      }
      this[ONREAD](bytesRead);
    });
  }
  /* c8 ignore start */
  [CLOSE](cb = () => {}) {
    if (this.fd !== void 0) import_fs2.default.close(this.fd, cb);
  }
  [ONREAD](bytesRead) {
    if (bytesRead <= 0 && this.remain > 0) {
      const er = Object.assign(new Error("encountered unexpected EOF"), {
        path: this.absolute,
        syscall: "read",
        code: "EOF",
      });
      return this[CLOSE](() => this.emit("error", er));
    }
    if (bytesRead > this.remain) {
      const er = Object.assign(new Error("did not encounter expected EOF"), {
        path: this.absolute,
        syscall: "read",
        code: "EOF",
      });
      return this[CLOSE](() => this.emit("error", er));
    }
    if (!this.buf) {
      throw new Error("should have created buffer prior to reading");
    }
    if (bytesRead === this.remain) {
      for (
        let i = bytesRead;
        i < this.length && bytesRead < this.blockRemain;
        i++
      ) {
        this.buf[i + this.offset] = 0;
        bytesRead++;
        this.remain++;
      }
    }
    const chunk =
      this.offset === 0 && bytesRead === this.buf.length
        ? this.buf
        : this.buf.subarray(this.offset, this.offset + bytesRead);
    const flushed = this.write(chunk);
    if (!flushed) {
      this[AWAITDRAIN](() => this[ONDRAIN]());
    } else {
      this[ONDRAIN]();
    }
  }
  [AWAITDRAIN](cb) {
    this.once("drain", cb);
  }
  write(chunk, encoding, cb) {
    if (typeof encoding === "function") {
      cb = encoding;
      encoding = void 0;
    }
    if (typeof chunk === "string") {
      chunk = Buffer.from(
        chunk,
        typeof encoding === "string" ? encoding : "utf8"
      );
    }
    if (this.blockRemain < chunk.length) {
      const er = Object.assign(new Error("writing more data than expected"), {
        path: this.absolute,
      });
      return this.emit("error", er);
    }
    this.remain -= chunk.length;
    this.blockRemain -= chunk.length;
    this.pos += chunk.length;
    this.offset += chunk.length;
    return super.write(chunk, null, cb);
  }
  [ONDRAIN]() {
    if (!this.remain) {
      if (this.blockRemain) {
        super.write(Buffer.alloc(this.blockRemain));
      }
      return this[CLOSE]((er) => (er ? this.emit("error", er) : this.end()));
    }
    if (!this.buf) {
      throw new Error("buffer lost somehow in ONDRAIN");
    }
    if (this.offset >= this.length) {
      this.buf = Buffer.allocUnsafe(
        Math.min(this.blockRemain, this.buf.length)
      );
      this.offset = 0;
    }
    this.length = this.buf.length - this.offset;
    this[READ2]();
  }
};
var WriteEntrySync = class extends WriteEntry {
  sync = true;
  [LSTAT]() {
    this[ONLSTAT](import_fs2.default.lstatSync(this.absolute));
  }
  [SYMLINK]() {
    this[ONREADLINK](import_fs2.default.readlinkSync(this.absolute));
  }
  [OPENFILE]() {
    this[ONOPENFILE](import_fs2.default.openSync(this.absolute, "r"));
  }
  [READ2]() {
    let threw = true;
    try {
      const { fd, buf, offset, length, pos: pos2 } = this;
      if (fd === void 0 || buf === void 0) {
        throw new Error("fd and buf must be set in READ method");
      }
      const bytesRead = import_fs2.default.readSync(
        fd,
        buf,
        offset,
        length,
        pos2
      );
      this[ONREAD](bytesRead);
      threw = false;
    } finally {
      if (threw) {
        try {
          this[CLOSE](() => {});
        } catch (er) {}
      }
    }
  }
  [AWAITDRAIN](cb) {
    cb();
  }
  /* c8 ignore start */
  [CLOSE](cb = () => {}) {
    if (this.fd !== void 0) import_fs2.default.closeSync(this.fd);
    cb();
  }
};
var WriteEntryTar = class extends Minipass {
  blockLen = 0;
  blockRemain = 0;
  buf = 0;
  pos = 0;
  remain = 0;
  length = 0;
  preservePaths;
  portable;
  strict;
  noPax;
  noMtime;
  readEntry;
  type;
  prefix;
  path;
  mode;
  uid;
  gid;
  uname;
  gname;
  header;
  mtime;
  atime;
  ctime;
  linkpath;
  size;
  onWriteEntry;
  warn(code2, message, data = {}) {
    return warnMethod(this, code2, message, data);
  }
  constructor(readEntry, opt_ = {}) {
    const opt = dealias(opt_);
    super();
    this.preservePaths = !!opt.preservePaths;
    this.portable = !!opt.portable;
    this.strict = !!opt.strict;
    this.noPax = !!opt.noPax;
    this.noMtime = !!opt.noMtime;
    this.onWriteEntry = opt.onWriteEntry;
    this.readEntry = readEntry;
    const { type } = readEntry;
    if (type === "Unsupported") {
      throw new Error("writing entry that should be ignored");
    }
    this.type = type;
    if (this.type === "Directory" && this.portable) {
      this.noMtime = true;
    }
    this.prefix = opt.prefix;
    this.path = normalizeWindowsPath(readEntry.path);
    this.mode = readEntry.mode !== void 0 ? this[MODE](readEntry.mode) : void 0;
    this.uid = this.portable ? void 0 : readEntry.uid;
    this.gid = this.portable ? void 0 : readEntry.gid;
    this.uname = this.portable ? void 0 : readEntry.uname;
    this.gname = this.portable ? void 0 : readEntry.gname;
    this.size = readEntry.size;
    this.mtime = this.noMtime ? void 0 : opt.mtime || readEntry.mtime;
    this.atime = this.portable ? void 0 : readEntry.atime;
    this.ctime = this.portable ? void 0 : readEntry.ctime;
    this.linkpath =
      readEntry.linkpath !== void 0
        ? normalizeWindowsPath(readEntry.linkpath)
        : void 0;
    if (typeof opt.onwarn === "function") {
      this.on("warn", opt.onwarn);
    }
    let pathWarn = false;
    if (!this.preservePaths) {
      const [root, stripped] = stripAbsolutePath(this.path);
      if (root && typeof stripped === "string") {
        this.path = stripped;
        pathWarn = root;
      }
    }
    this.remain = readEntry.size;
    this.blockRemain = readEntry.startBlockSize;
    this.onWriteEntry?.(this);
    this.header = new Header({
      path: this[PREFIX](this.path),
      linkpath:
        this.type === "Link" && this.linkpath !== void 0
          ? this[PREFIX](this.linkpath)
          : this.linkpath,
      // only the permissions and setuid/setgid/sticky bitflags
      // not the higher-order bits that specify file type
      mode: this.mode,
      uid: this.portable ? void 0 : this.uid,
      gid: this.portable ? void 0 : this.gid,
      size: this.size,
      mtime: this.noMtime ? void 0 : this.mtime,
      type: this.type,
      uname: this.portable ? void 0 : this.uname,
      atime: this.portable ? void 0 : this.atime,
      ctime: this.portable ? void 0 : this.ctime,
    });
    if (pathWarn) {
      this.warn("TAR_ENTRY_INFO", `stripping ${pathWarn} from absolute path`, {
        entry: this,
        path: pathWarn + this.path,
      });
    }
    if (this.header.encode() && !this.noPax) {
      super.write(
        new Pax({
          atime: this.portable ? void 0 : this.atime,
          ctime: this.portable ? void 0 : this.ctime,
          gid: this.portable ? void 0 : this.gid,
          mtime: this.noMtime ? void 0 : this.mtime,
          path: this[PREFIX](this.path),
          linkpath:
            this.type === "Link" && this.linkpath !== void 0
              ? this[PREFIX](this.linkpath)
              : this.linkpath,
          size: this.size,
          uid: this.portable ? void 0 : this.uid,
          uname: this.portable ? void 0 : this.uname,
          dev: this.portable ? void 0 : this.readEntry.dev,
          ino: this.portable ? void 0 : this.readEntry.ino,
          nlink: this.portable ? void 0 : this.readEntry.nlink,
        }).encode()
      );
    }
    const b = this.header?.block;
    if (!b) throw new Error("failed to encode header");
    super.write(b);
    readEntry.pipe(this);
  }
  [PREFIX](path8) {
    return prefixPath(path8, this.prefix);
  }
  [MODE](mode) {
    return modeFix(mode, this.type === "Directory", this.portable);
  }
  write(chunk, encoding, cb) {
    if (typeof encoding === "function") {
      cb = encoding;
      encoding = void 0;
    }
    if (typeof chunk === "string") {
      chunk = Buffer.from(
        chunk,
        typeof encoding === "string" ? encoding : "utf8"
      );
    }
    const writeLen = chunk.length;
    if (writeLen > this.blockRemain) {
      throw new Error("writing more to entry than is appropriate");
    }
    this.blockRemain -= writeLen;
    return super.write(chunk, cb);
  }
  end(chunk, encoding, cb) {
    if (this.blockRemain) {
      super.write(Buffer.alloc(this.blockRemain));
    }
    if (typeof chunk === "function") {
      cb = chunk;
      encoding = void 0;
      chunk = void 0;
    }
    if (typeof encoding === "function") {
      cb = encoding;
      encoding = void 0;
    }
    if (typeof chunk === "string") {
      chunk = Buffer.from(chunk, encoding ?? "utf8");
    }
    if (cb) this.once("finish", cb);
    chunk ? super.end(chunk, cb) : super.end(cb);
    return this;
  }
};
var getType = (stat2) =>
  stat2.isFile()
    ? "File"
    : stat2.isDirectory()
    ? "Directory"
    : stat2.isSymbolicLink()
    ? "SymbolicLink"
    : "Unsupported";

// node_modules/tar/dist/esm/pack.js
var import_path3 = __toESM(require("path"), 1);
var PackJob = class {
  path;
  absolute;
  entry;
  stat;
  readdir;
  pending = false;
  ignore = false;
  piped = false;
  constructor(path8, absolute) {
    this.path = path8 || "./";
    this.absolute = absolute;
  }
};
var EOF2 = Buffer.alloc(1024);
var ONSTAT = Symbol("onStat");
var ENDED2 = Symbol("ended");
var QUEUE2 = Symbol("queue");
var CURRENT = Symbol("current");
var PROCESS2 = Symbol("process");
var PROCESSING = Symbol("processing");
var PROCESSJOB = Symbol("processJob");
var JOBS = Symbol("jobs");
var JOBDONE = Symbol("jobDone");
var ADDFSENTRY = Symbol("addFSEntry");
var ADDTARENTRY = Symbol("addTarEntry");
var STAT = Symbol("stat");
var READDIR = Symbol("readdir");
var ONREADDIR = Symbol("onreaddir");
var PIPE = Symbol("pipe");
var ENTRY = Symbol("entry");
var ENTRYOPT = Symbol("entryOpt");
var WRITEENTRYCLASS = Symbol("writeEntryClass");
var WRITE = Symbol("write");
var ONDRAIN2 = Symbol("ondrain");
var Pack = class extends Minipass {
  opt;
  cwd;
  maxReadSize;
  preservePaths;
  strict;
  noPax;
  prefix;
  linkCache;
  statCache;
  file;
  portable;
  zip;
  readdirCache;
  noDirRecurse;
  follow;
  noMtime;
  mtime;
  filter;
  jobs;
  [WRITEENTRYCLASS];
  onWriteEntry;
  [QUEUE2];
  [JOBS] = 0;
  [PROCESSING] = false;
  [ENDED2] = false;
  constructor(opt = {}) {
    super();
    this.opt = opt;
    this.file = opt.file || "";
    this.cwd = opt.cwd || process.cwd();
    this.maxReadSize = opt.maxReadSize;
    this.preservePaths = !!opt.preservePaths;
    this.strict = !!opt.strict;
    this.noPax = !!opt.noPax;
    this.prefix = normalizeWindowsPath(opt.prefix || "");
    this.linkCache = opt.linkCache || /* @__PURE__ */ new Map();
    this.statCache = opt.statCache || /* @__PURE__ */ new Map();
    this.readdirCache = opt.readdirCache || /* @__PURE__ */ new Map();
    this.onWriteEntry = opt.onWriteEntry;
    this[WRITEENTRYCLASS] = WriteEntry;
    if (typeof opt.onwarn === "function") {
      this.on("warn", opt.onwarn);
    }
    this.portable = !!opt.portable;
    if (opt.gzip || opt.brotli) {
      if (opt.gzip && opt.brotli) {
        throw new TypeError("gzip and brotli are mutually exclusive");
      }
      if (opt.gzip) {
        if (typeof opt.gzip !== "object") {
          opt.gzip = {};
        }
        if (this.portable) {
          opt.gzip.portable = true;
        }
        this.zip = new Gzip(opt.gzip);
      }
      if (opt.brotli) {
        if (typeof opt.brotli !== "object") {
          opt.brotli = {};
        }
        this.zip = new BrotliCompress(opt.brotli);
      }
      if (!this.zip) throw new Error("impossible");
      const zip = this.zip;
      zip.on("data", (chunk) => super.write(chunk));
      zip.on("end", () => super.end());
      zip.on("drain", () => this[ONDRAIN2]());
      this.on("resume", () => zip.resume());
    } else {
      this.on("drain", this[ONDRAIN2]);
    }
    this.noDirRecurse = !!opt.noDirRecurse;
    this.follow = !!opt.follow;
    this.noMtime = !!opt.noMtime;
    if (opt.mtime) this.mtime = opt.mtime;
    this.filter = typeof opt.filter === "function" ? opt.filter : () => true;
    this[QUEUE2] = new Yallist();
    this[JOBS] = 0;
    this.jobs = Number(opt.jobs) || 4;
    this[PROCESSING] = false;
    this[ENDED2] = false;
  }
  [WRITE](chunk) {
    return super.write(chunk);
  }
  add(path8) {
    this.write(path8);
    return this;
  }
  end(path8, encoding, cb) {
    if (typeof path8 === "function") {
      cb = path8;
      path8 = void 0;
    }
    if (typeof encoding === "function") {
      cb = encoding;
      encoding = void 0;
    }
    if (path8) {
      this.add(path8);
    }
    this[ENDED2] = true;
    this[PROCESS2]();
    if (cb) cb();
    return this;
  }
  write(path8) {
    if (this[ENDED2]) {
      throw new Error("write after end");
    }
    if (path8 instanceof ReadEntry) {
      this[ADDTARENTRY](path8);
    } else {
      this[ADDFSENTRY](path8);
    }
    return this.flowing;
  }
  [ADDTARENTRY](p) {
    const absolute = normalizeWindowsPath(
      import_path3.default.resolve(this.cwd, p.path)
    );
    if (!this.filter(p.path, p)) {
      p.resume();
    } else {
      const job = new PackJob(p.path, absolute);
      job.entry = new WriteEntryTar(p, this[ENTRYOPT](job));
      job.entry.on("end", () => this[JOBDONE](job));
      this[JOBS] += 1;
      this[QUEUE2].push(job);
    }
    this[PROCESS2]();
  }
  [ADDFSENTRY](p) {
    const absolute = normalizeWindowsPath(
      import_path3.default.resolve(this.cwd, p)
    );
    this[QUEUE2].push(new PackJob(p, absolute));
    this[PROCESS2]();
  }
  [STAT](job) {
    job.pending = true;
    this[JOBS] += 1;
    const stat2 = this.follow ? "stat" : "lstat";
    import_fs3.default[stat2](job.absolute, (er, stat3) => {
      job.pending = false;
      this[JOBS] -= 1;
      if (er) {
        this.emit("error", er);
      } else {
        this[ONSTAT](job, stat3);
      }
    });
  }
  [ONSTAT](job, stat2) {
    this.statCache.set(job.absolute, stat2);
    job.stat = stat2;
    if (!this.filter(job.path, stat2)) {
      job.ignore = true;
    }
    this[PROCESS2]();
  }
  [READDIR](job) {
    job.pending = true;
    this[JOBS] += 1;
    import_fs3.default.readdir(job.absolute, (er, entries) => {
      job.pending = false;
      this[JOBS] -= 1;
      if (er) {
        return this.emit("error", er);
      }
      this[ONREADDIR](job, entries);
    });
  }
  [ONREADDIR](job, entries) {
    this.readdirCache.set(job.absolute, entries);
    job.readdir = entries;
    this[PROCESS2]();
  }
  [PROCESS2]() {
    if (this[PROCESSING]) {
      return;
    }
    this[PROCESSING] = true;
    for (let w = this[QUEUE2].head; !!w && this[JOBS] < this.jobs; w = w.next) {
      this[PROCESSJOB](w.value);
      if (w.value.ignore) {
        const p = w.next;
        this[QUEUE2].removeNode(w);
        w.next = p;
      }
    }
    this[PROCESSING] = false;
    if (this[ENDED2] && !this[QUEUE2].length && this[JOBS] === 0) {
      if (this.zip) {
        this.zip.end(EOF2);
      } else {
        super.write(EOF2);
        super.end();
      }
    }
  }
  get [CURRENT]() {
    return this[QUEUE2] && this[QUEUE2].head && this[QUEUE2].head.value;
  }
  [JOBDONE](_job) {
    this[QUEUE2].shift();
    this[JOBS] -= 1;
    this[PROCESS2]();
  }
  [PROCESSJOB](job) {
    if (job.pending) {
      return;
    }
    if (job.entry) {
      if (job === this[CURRENT] && !job.piped) {
        this[PIPE](job);
      }
      return;
    }
    if (!job.stat) {
      const sc = this.statCache.get(job.absolute);
      if (sc) {
        this[ONSTAT](job, sc);
      } else {
        this[STAT](job);
      }
    }
    if (!job.stat) {
      return;
    }
    if (job.ignore) {
      return;
    }
    if (!this.noDirRecurse && job.stat.isDirectory() && !job.readdir) {
      const rc = this.readdirCache.get(job.absolute);
      if (rc) {
        this[ONREADDIR](job, rc);
      } else {
        this[READDIR](job);
      }
      if (!job.readdir) {
        return;
      }
    }
    job.entry = this[ENTRY](job);
    if (!job.entry) {
      job.ignore = true;
      return;
    }
    if (job === this[CURRENT] && !job.piped) {
      this[PIPE](job);
    }
  }
  [ENTRYOPT](job) {
    return {
      onwarn: (code2, msg, data) => this.warn(code2, msg, data),
      noPax: this.noPax,
      cwd: this.cwd,
      absolute: job.absolute,
      preservePaths: this.preservePaths,
      maxReadSize: this.maxReadSize,
      strict: this.strict,
      portable: this.portable,
      linkCache: this.linkCache,
      statCache: this.statCache,
      noMtime: this.noMtime,
      mtime: this.mtime,
      prefix: this.prefix,
      onWriteEntry: this.onWriteEntry,
    };
  }
  [ENTRY](job) {
    this[JOBS] += 1;
    try {
      const e = new this[WRITEENTRYCLASS](job.path, this[ENTRYOPT](job));
      return e
        .on("end", () => this[JOBDONE](job))
        .on("error", (er) => this.emit("error", er));
    } catch (er) {
      this.emit("error", er);
    }
  }
  [ONDRAIN2]() {
    if (this[CURRENT] && this[CURRENT].entry) {
      this[CURRENT].entry.resume();
    }
  }
  // like .pipe() but using super, because our write() is special
  [PIPE](job) {
    job.piped = true;
    if (job.readdir) {
      job.readdir.forEach((entry) => {
        const p = job.path;
        const base = p === "./" ? "" : p.replace(/\/*$/, "/");
        this[ADDFSENTRY](base + entry);
      });
    }
    const source = job.entry;
    const zip = this.zip;
    if (!source) throw new Error("cannot pipe without source");
    if (zip) {
      source.on("data", (chunk) => {
        if (!zip.write(chunk)) {
          source.pause();
        }
      });
    } else {
      source.on("data", (chunk) => {
        if (!super.write(chunk)) {
          source.pause();
        }
      });
    }
  }
  pause() {
    if (this.zip) {
      this.zip.pause();
    }
    return super.pause();
  }
  warn(code2, message, data = {}) {
    warnMethod(this, code2, message, data);
  }
};
var PackSync = class extends Pack {
  sync = true;
  constructor(opt) {
    super(opt);
    this[WRITEENTRYCLASS] = WriteEntrySync;
  }
  // pause/resume are no-ops in sync streams.
  pause() {}
  resume() {}
  [STAT](job) {
    const stat2 = this.follow ? "statSync" : "lstatSync";
    this[ONSTAT](job, import_fs3.default[stat2](job.absolute));
  }
  [READDIR](job) {
    this[ONREADDIR](job, import_fs3.default.readdirSync(job.absolute));
  }
  // gotta get it all in this tick
  [PIPE](job) {
    const source = job.entry;
    const zip = this.zip;
    if (job.readdir) {
      job.readdir.forEach((entry) => {
        const p = job.path;
        const base = p === "./" ? "" : p.replace(/\/*$/, "/");
        this[ADDFSENTRY](base + entry);
      });
    }
    if (!source) throw new Error("Cannot pipe without source");
    if (zip) {
      source.on("data", (chunk) => {
        zip.write(chunk);
      });
    } else {
      source.on("data", (chunk) => {
        super[WRITE](chunk);
      });
    }
  }
};

// node_modules/tar/dist/esm/create.js
var createFileSync = (opt, files) => {
  const p = new PackSync(opt);
  const stream = new WriteStreamSync(opt.file, {
    mode: opt.mode || 438,
  });
  p.pipe(stream);
  addFilesSync(p, files);
};
var createFile = (opt, files) => {
  const p = new Pack(opt);
  const stream = new WriteStream(opt.file, {
    mode: opt.mode || 438,
  });
  p.pipe(stream);
  const promise = new Promise((res, rej) => {
    stream.on("error", rej);
    stream.on("close", res);
    p.on("error", rej);
  });
  addFilesAsync(p, files);
  return promise;
};
var addFilesSync = (p, files) => {
  files.forEach((file) => {
    if (file.charAt(0) === "@") {
      list({
        file: import_node_path4.default.resolve(p.cwd, file.slice(1)),
        sync: true,
        noResume: true,
        onReadEntry: (entry) => p.add(entry),
      });
    } else {
      p.add(file);
    }
  });
  p.end();
};
var addFilesAsync = async (p, files) => {
  for (let i = 0; i < files.length; i++) {
    const file = String(files[i]);
    if (file.charAt(0) === "@") {
      await list({
        file: import_node_path4.default.resolve(String(p.cwd), file.slice(1)),
        noResume: true,
        onReadEntry: (entry) => {
          p.add(entry);
        },
      });
    } else {
      p.add(file);
    }
  }
  p.end();
};
var createSync = (opt, files) => {
  const p = new PackSync(opt);
  addFilesSync(p, files);
  return p;
};
var createAsync = (opt, files) => {
  const p = new Pack(opt);
  addFilesAsync(p, files);
  return p;
};
var create = makeCommand(
  createFileSync,
  createFile,
  createSync,
  createAsync,
  (_opt, files) => {
    if (!files?.length) {
      throw new TypeError("no paths specified to add to archive");
    }
  }
);

// node_modules/tar/dist/esm/extract.js
var import_node_fs4 = __toESM(require("node:fs"), 1);

// node_modules/tar/dist/esm/unpack.js
var import_node_assert = __toESM(require("node:assert"), 1);
var import_node_crypto = require("node:crypto");
var import_node_fs3 = __toESM(require("node:fs"), 1);
var import_node_path8 = __toESM(require("node:path"), 1);

// node_modules/tar/dist/esm/get-write-flag.js
var import_fs4 = __toESM(require("fs"), 1);
var platform2 = process.env.__FAKE_PLATFORM__ || process.platform;
var isWindows = platform2 === "win32";
var { O_CREAT, O_TRUNC, O_WRONLY } = import_fs4.default.constants;
var UV_FS_O_FILEMAP =
  Number(process.env.__FAKE_FS_O_FILENAME__) ||
  import_fs4.default.constants.UV_FS_O_FILEMAP ||
  0;
var fMapEnabled = isWindows && !!UV_FS_O_FILEMAP;
var fMapLimit = 512 * 1024;
var fMapFlag = UV_FS_O_FILEMAP | O_TRUNC | O_CREAT | O_WRONLY;
var getWriteFlag = !fMapEnabled
  ? () => "w"
  : (size) => (size < fMapLimit ? fMapFlag : "w");

// node_modules/chownr/dist/esm/index.js
var import_node_fs2 = __toESM(require("node:fs"), 1);
var import_node_path5 = __toESM(require("node:path"), 1);
var lchownSync = (path8, uid, gid) => {
  try {
    return import_node_fs2.default.lchownSync(path8, uid, gid);
  } catch (er) {
    if (er?.code !== "ENOENT") throw er;
  }
};
var chown = (cpath, uid, gid, cb) => {
  import_node_fs2.default.lchown(cpath, uid, gid, (er) => {
    cb(er && er?.code !== "ENOENT" ? er : null);
  });
};
var chownrKid = (p, child, uid, gid, cb) => {
  if (child.isDirectory()) {
    chownr(import_node_path5.default.resolve(p, child.name), uid, gid, (er) => {
      if (er) return cb(er);
      const cpath = import_node_path5.default.resolve(p, child.name);
      chown(cpath, uid, gid, cb);
    });
  } else {
    const cpath = import_node_path5.default.resolve(p, child.name);
    chown(cpath, uid, gid, cb);
  }
};
var chownr = (p, uid, gid, cb) => {
  import_node_fs2.default.readdir(
    p,
    { withFileTypes: true },
    (er, children) => {
      if (er) {
        if (er.code === "ENOENT") return cb();
        else if (er.code !== "ENOTDIR" && er.code !== "ENOTSUP") return cb(er);
      }
      if (er || !children.length) return chown(p, uid, gid, cb);
      let len = children.length;
      let errState = null;
      const then = (er2) => {
        if (errState) return;
        if (er2) return cb((errState = er2));
        if (--len === 0) return chown(p, uid, gid, cb);
      };
      for (const child of children) {
        chownrKid(p, child, uid, gid, then);
      }
    }
  );
};
var chownrKidSync = (p, child, uid, gid) => {
  if (child.isDirectory())
    chownrSync(import_node_path5.default.resolve(p, child.name), uid, gid);
  lchownSync(import_node_path5.default.resolve(p, child.name), uid, gid);
};
var chownrSync = (p, uid, gid) => {
  let children;
  try {
    children = import_node_fs2.default.readdirSync(p, { withFileTypes: true });
  } catch (er) {
    const e = er;
    if (e?.code === "ENOENT") return;
    else if (e?.code === "ENOTDIR" || e?.code === "ENOTSUP")
      return lchownSync(p, uid, gid);
    else throw e;
  }
  for (const child of children) {
    chownrKidSync(p, child, uid, gid);
  }
  return lchownSync(p, uid, gid);
};

// node_modules/tar/dist/esm/mkdir.js
var import_fs7 = __toESM(require("fs"), 1);

// node_modules/mkdirp/dist/mjs/mkdirp-manual.js
var import_path4 = require("path");

// node_modules/mkdirp/dist/mjs/opts-arg.js
var import_fs5 = require("fs");
var optsArg = (opts) => {
  if (!opts) {
    opts = { mode: 511 };
  } else if (typeof opts === "object") {
    opts = { mode: 511, ...opts };
  } else if (typeof opts === "number") {
    opts = { mode: opts };
  } else if (typeof opts === "string") {
    opts = { mode: parseInt(opts, 8) };
  } else {
    throw new TypeError("invalid options argument");
  }
  const resolved = opts;
  const optsFs = opts.fs || {};
  opts.mkdir = opts.mkdir || optsFs.mkdir || import_fs5.mkdir;
  opts.mkdirAsync = opts.mkdirAsync
    ? opts.mkdirAsync
    : async (path8, options) => {
        return new Promise((res, rej) =>
          resolved.mkdir(path8, options, (er, made) =>
            er ? rej(er) : res(made)
          )
        );
      };
  opts.stat = opts.stat || optsFs.stat || import_fs5.stat;
  opts.statAsync = opts.statAsync
    ? opts.statAsync
    : async (path8) =>
        new Promise((res, rej) =>
          resolved.stat(path8, (err, stats) => (err ? rej(err) : res(stats)))
        );
  opts.statSync = opts.statSync || optsFs.statSync || import_fs5.statSync;
  opts.mkdirSync = opts.mkdirSync || optsFs.mkdirSync || import_fs5.mkdirSync;
  return resolved;
};

// node_modules/mkdirp/dist/mjs/mkdirp-manual.js
var mkdirpManualSync = (path8, options, made) => {
  const parent = (0, import_path4.dirname)(path8);
  const opts = { ...optsArg(options), recursive: false };
  if (parent === path8) {
    try {
      return opts.mkdirSync(path8, opts);
    } catch (er) {
      const fer = er;
      if (fer && fer.code !== "EISDIR") {
        throw er;
      }
      return;
    }
  }
  try {
    opts.mkdirSync(path8, opts);
    return made || path8;
  } catch (er) {
    const fer = er;
    if (fer && fer.code === "ENOENT") {
      return mkdirpManualSync(
        path8,
        opts,
        mkdirpManualSync(parent, opts, made)
      );
    }
    if (fer && fer.code !== "EEXIST" && fer && fer.code !== "EROFS") {
      throw er;
    }
    try {
      if (!opts.statSync(path8).isDirectory()) throw er;
    } catch (_) {
      throw er;
    }
  }
};
var mkdirpManual = Object.assign(
  async (path8, options, made) => {
    const opts = optsArg(options);
    opts.recursive = false;
    const parent = (0, import_path4.dirname)(path8);
    if (parent === path8) {
      return opts.mkdirAsync(path8, opts).catch((er) => {
        const fer = er;
        if (fer && fer.code !== "EISDIR") {
          throw er;
        }
      });
    }
    return opts.mkdirAsync(path8, opts).then(
      () => made || path8,
      async (er) => {
        const fer = er;
        if (fer && fer.code === "ENOENT") {
          return mkdirpManual(parent, opts).then((made2) =>
            mkdirpManual(path8, opts, made2)
          );
        }
        if (fer && fer.code !== "EEXIST" && fer.code !== "EROFS") {
          throw er;
        }
        return opts.statAsync(path8).then(
          (st) => {
            if (st.isDirectory()) {
              return made;
            } else {
              throw er;
            }
          },
          () => {
            throw er;
          }
        );
      }
    );
  },
  { sync: mkdirpManualSync }
);

// node_modules/mkdirp/dist/mjs/mkdirp-native.js
var import_path6 = require("path");

// node_modules/mkdirp/dist/mjs/find-made.js
var import_path5 = require("path");
var findMade = async (opts, parent, path8) => {
  if (path8 === parent) {
    return;
  }
  return opts.statAsync(parent).then(
    (st) => (st.isDirectory() ? path8 : void 0),
    // will fail later
    // will fail later
    (er) => {
      const fer = er;
      return fer && fer.code === "ENOENT"
        ? findMade(opts, (0, import_path5.dirname)(parent), parent)
        : void 0;
    }
  );
};
var findMadeSync = (opts, parent, path8) => {
  if (path8 === parent) {
    return void 0;
  }
  try {
    return opts.statSync(parent).isDirectory() ? path8 : void 0;
  } catch (er) {
    const fer = er;
    return fer && fer.code === "ENOENT"
      ? findMadeSync(opts, (0, import_path5.dirname)(parent), parent)
      : void 0;
  }
};

// node_modules/mkdirp/dist/mjs/mkdirp-native.js
var mkdirpNativeSync = (path8, options) => {
  const opts = optsArg(options);
  opts.recursive = true;
  const parent = (0, import_path6.dirname)(path8);
  if (parent === path8) {
    return opts.mkdirSync(path8, opts);
  }
  const made = findMadeSync(opts, path8);
  try {
    opts.mkdirSync(path8, opts);
    return made;
  } catch (er) {
    const fer = er;
    if (fer && fer.code === "ENOENT") {
      return mkdirpManualSync(path8, opts);
    } else {
      throw er;
    }
  }
};
var mkdirpNative = Object.assign(
  async (path8, options) => {
    const opts = { ...optsArg(options), recursive: true };
    const parent = (0, import_path6.dirname)(path8);
    if (parent === path8) {
      return await opts.mkdirAsync(path8, opts);
    }
    return findMade(opts, path8).then((made) =>
      opts
        .mkdirAsync(path8, opts)
        .then((m) => made || m)
        .catch((er) => {
          const fer = er;
          if (fer && fer.code === "ENOENT") {
            return mkdirpManual(path8, opts);
          } else {
            throw er;
          }
        })
    );
  },
  { sync: mkdirpNativeSync }
);

// node_modules/mkdirp/dist/mjs/path-arg.js
var import_path7 = require("path");
var platform3 = process.env.__TESTING_MKDIRP_PLATFORM__ || process.platform;
var pathArg = (path8) => {
  if (/\0/.test(path8)) {
    throw Object.assign(
      new TypeError("path must be a string without null bytes"),
      {
        path: path8,
        code: "ERR_INVALID_ARG_VALUE",
      }
    );
  }
  path8 = (0, import_path7.resolve)(path8);
  if (platform3 === "win32") {
    const badWinChars = /[*|"<>?:]/;
    const { root } = (0, import_path7.parse)(path8);
    if (badWinChars.test(path8.substring(root.length))) {
      throw Object.assign(new Error("Illegal characters in path."), {
        path: path8,
        code: "EINVAL",
      });
    }
  }
  return path8;
};

// node_modules/mkdirp/dist/mjs/use-native.js
var import_fs6 = require("fs");
var version = process.env.__TESTING_MKDIRP_NODE_VERSION__ || process.version;
var versArr = version.replace(/^v/, "").split(".");
var hasNative = +versArr[0] > 10 || (+versArr[0] === 10 && +versArr[1] >= 12);
var useNativeSync = !hasNative
  ? () => false
  : (opts) => optsArg(opts).mkdirSync === import_fs6.mkdirSync;
var useNative = Object.assign(
  !hasNative ? () => false : (opts) => optsArg(opts).mkdir === import_fs6.mkdir,
  {
    sync: useNativeSync,
  }
);

// node_modules/mkdirp/dist/mjs/index.js
var mkdirpSync = (path8, opts) => {
  path8 = pathArg(path8);
  const resolved = optsArg(opts);
  return useNativeSync(resolved)
    ? mkdirpNativeSync(path8, resolved)
    : mkdirpManualSync(path8, resolved);
};
var mkdirp = Object.assign(
  async (path8, opts) => {
    path8 = pathArg(path8);
    const resolved = optsArg(opts);
    return useNative(resolved)
      ? mkdirpNative(path8, resolved)
      : mkdirpManual(path8, resolved);
  },
  {
    mkdirpSync,
    mkdirpNative,
    mkdirpNativeSync,
    mkdirpManual,
    mkdirpManualSync,
    sync: mkdirpSync,
    native: mkdirpNative,
    nativeSync: mkdirpNativeSync,
    manual: mkdirpManual,
    manualSync: mkdirpManualSync,
    useNative,
    useNativeSync,
  }
);

// node_modules/tar/dist/esm/mkdir.js
var import_node_path6 = __toESM(require("node:path"), 1);

// node_modules/tar/dist/esm/cwd-error.js
var CwdError = class extends Error {
  path;
  code;
  syscall = "chdir";
  constructor(path8, code2) {
    super(`${code2}: Cannot cd into '${path8}'`);
    this.path = path8;
    this.code = code2;
  }
  get name() {
    return "CwdError";
  }
};

// node_modules/tar/dist/esm/symlink-error.js
var SymlinkError = class extends Error {
  path;
  symlink;
  syscall = "symlink";
  code = "TAR_SYMLINK_ERROR";
  constructor(symlink, path8) {
    super("TAR_SYMLINK_ERROR: Cannot extract through symbolic link");
    this.symlink = symlink;
    this.path = path8;
  }
  get name() {
    return "SymlinkError";
  }
};

// node_modules/tar/dist/esm/mkdir.js
var cGet = (cache, key) => cache.get(normalizeWindowsPath(key));
var cSet = (cache, key, val) => cache.set(normalizeWindowsPath(key), val);
var checkCwd = (dir, cb) => {
  import_fs7.default.stat(dir, (er, st) => {
    if (er || !st.isDirectory()) {
      er = new CwdError(dir, er?.code || "ENOTDIR");
    }
    cb(er);
  });
};
var mkdir3 = (dir, opt, cb) => {
  dir = normalizeWindowsPath(dir);
  const umask = opt.umask ?? 18;
  const mode = opt.mode | 448;
  const needChmod = (mode & umask) !== 0;
  const uid = opt.uid;
  const gid = opt.gid;
  const doChown =
    typeof uid === "number" &&
    typeof gid === "number" &&
    (uid !== opt.processUid || gid !== opt.processGid);
  const preserve = opt.preserve;
  const unlink = opt.unlink;
  const cache = opt.cache;
  const cwd = normalizeWindowsPath(opt.cwd);
  const done = (er, created) => {
    if (er) {
      cb(er);
    } else {
      cSet(cache, dir, true);
      if (created && doChown) {
        chownr(created, uid, gid, (er2) => done(er2));
      } else if (needChmod) {
        import_fs7.default.chmod(dir, mode, cb);
      } else {
        cb();
      }
    }
  };
  if (cache && cGet(cache, dir) === true) {
    return done();
  }
  if (dir === cwd) {
    return checkCwd(dir, done);
  }
  if (preserve) {
    return mkdirp(dir, { mode }).then(
      (made) => done(null, made ?? void 0),
      // oh, ts
      done
    );
  }
  const sub = normalizeWindowsPath(
    import_node_path6.default.relative(cwd, dir)
  );
  const parts = sub.split("/");
  mkdir_(cwd, parts, mode, cache, unlink, cwd, void 0, done);
};
var mkdir_ = (base, parts, mode, cache, unlink, cwd, created, cb) => {
  if (!parts.length) {
    return cb(null, created);
  }
  const p = parts.shift();
  const part = normalizeWindowsPath(
    import_node_path6.default.resolve(base + "/" + p)
  );
  if (cGet(cache, part)) {
    return mkdir_(part, parts, mode, cache, unlink, cwd, created, cb);
  }
  import_fs7.default.mkdir(
    part,
    mode,
    onmkdir(part, parts, mode, cache, unlink, cwd, created, cb)
  );
};
var onmkdir = (part, parts, mode, cache, unlink, cwd, created, cb) => (er) => {
  if (er) {
    import_fs7.default.lstat(part, (statEr, st) => {
      if (statEr) {
        statEr.path = statEr.path && normalizeWindowsPath(statEr.path);
        cb(statEr);
      } else if (st.isDirectory()) {
        mkdir_(part, parts, mode, cache, unlink, cwd, created, cb);
      } else if (unlink) {
        import_fs7.default.unlink(part, (er2) => {
          if (er2) {
            return cb(er2);
          }
          import_fs7.default.mkdir(
            part,
            mode,
            onmkdir(part, parts, mode, cache, unlink, cwd, created, cb)
          );
        });
      } else if (st.isSymbolicLink()) {
        return cb(new SymlinkError(part, part + "/" + parts.join("/")));
      } else {
        cb(er);
      }
    });
  } else {
    created = created || part;
    mkdir_(part, parts, mode, cache, unlink, cwd, created, cb);
  }
};
var checkCwdSync = (dir) => {
  let ok = false;
  let code2 = void 0;
  try {
    ok = import_fs7.default.statSync(dir).isDirectory();
  } catch (er) {
    code2 = er?.code;
  } finally {
    if (!ok) {
      throw new CwdError(dir, code2 ?? "ENOTDIR");
    }
  }
};
var mkdirSync3 = (dir, opt) => {
  dir = normalizeWindowsPath(dir);
  const umask = opt.umask ?? 18;
  const mode = opt.mode | 448;
  const needChmod = (mode & umask) !== 0;
  const uid = opt.uid;
  const gid = opt.gid;
  const doChown =
    typeof uid === "number" &&
    typeof gid === "number" &&
    (uid !== opt.processUid || gid !== opt.processGid);
  const preserve = opt.preserve;
  const unlink = opt.unlink;
  const cache = opt.cache;
  const cwd = normalizeWindowsPath(opt.cwd);
  const done = (created2) => {
    cSet(cache, dir, true);
    if (created2 && doChown) {
      chownrSync(created2, uid, gid);
    }
    if (needChmod) {
      import_fs7.default.chmodSync(dir, mode);
    }
  };
  if (cache && cGet(cache, dir) === true) {
    return done();
  }
  if (dir === cwd) {
    checkCwdSync(cwd);
    return done();
  }
  if (preserve) {
    return done(mkdirpSync(dir, mode) ?? void 0);
  }
  const sub = normalizeWindowsPath(
    import_node_path6.default.relative(cwd, dir)
  );
  const parts = sub.split("/");
  let created = void 0;
  for (
    let p = parts.shift(), part = cwd;
    p && (part += "/" + p);
    p = parts.shift()
  ) {
    part = normalizeWindowsPath(import_node_path6.default.resolve(part));
    if (cGet(cache, part)) {
      continue;
    }
    try {
      import_fs7.default.mkdirSync(part, mode);
      created = created || part;
      cSet(cache, part, true);
    } catch (er) {
      const st = import_fs7.default.lstatSync(part);
      if (st.isDirectory()) {
        cSet(cache, part, true);
        continue;
      } else if (unlink) {
        import_fs7.default.unlinkSync(part);
        import_fs7.default.mkdirSync(part, mode);
        created = created || part;
        cSet(cache, part, true);
        continue;
      } else if (st.isSymbolicLink()) {
        return new SymlinkError(part, part + "/" + parts.join("/"));
      }
    }
  }
  return done(created);
};

// node_modules/tar/dist/esm/normalize-unicode.js
var normalizeCache = /* @__PURE__ */ Object.create(null);
var { hasOwnProperty } = Object.prototype;
var normalizeUnicode = (s) => {
  if (!hasOwnProperty.call(normalizeCache, s)) {
    normalizeCache[s] = s.normalize("NFD");
  }
  return normalizeCache[s];
};

// node_modules/tar/dist/esm/path-reservations.js
var import_node_path7 = require("node:path");
var platform4 = process.env.TESTING_TAR_FAKE_PLATFORM || process.platform;
var isWindows2 = platform4 === "win32";
var getDirs = (path8) => {
  const dirs = path8
    .split("/")
    .slice(0, -1)
    .reduce((set, path9) => {
      const s = set[set.length - 1];
      if (s !== void 0) {
        path9 = (0, import_node_path7.join)(s, path9);
      }
      set.push(path9 || "/");
      return set;
    }, []);
  return dirs;
};
var PathReservations = class {
  // path => [function or Set]
  // A Set object means a directory reservation
  // A fn is a direct reservation on that path
  #queues = /* @__PURE__ */ new Map();
  // fn => {paths:[path,...], dirs:[path, ...]}
  #reservations = /* @__PURE__ */ new Map();
  // functions currently running
  #running = /* @__PURE__ */ new Set();
  reserve(paths, fn) {
    paths = isWindows2
      ? ["win32 parallelization disabled"]
      : paths.map((p) => {
          return stripTrailingSlashes(
            (0, import_node_path7.join)(normalizeUnicode(p))
          ).toLowerCase();
        });
    const dirs = new Set(
      paths.map((path8) => getDirs(path8)).reduce((a, b) => a.concat(b))
    );
    this.#reservations.set(fn, { dirs, paths });
    for (const p of paths) {
      const q = this.#queues.get(p);
      if (!q) {
        this.#queues.set(p, [fn]);
      } else {
        q.push(fn);
      }
    }
    for (const dir of dirs) {
      const q = this.#queues.get(dir);
      if (!q) {
        this.#queues.set(dir, [/* @__PURE__ */ new Set([fn])]);
      } else {
        const l = q[q.length - 1];
        if (l instanceof Set) {
          l.add(fn);
        } else {
          q.push(/* @__PURE__ */ new Set([fn]));
        }
      }
    }
    return this.#run(fn);
  }
  // return the queues for each path the function cares about
  // fn => {paths, dirs}
  #getQueues(fn) {
    const res = this.#reservations.get(fn);
    if (!res) {
      throw new Error("function does not have any path reservations");
    }
    return {
      paths: res.paths.map((path8) => this.#queues.get(path8)),
      dirs: [...res.dirs].map((path8) => this.#queues.get(path8)),
    };
  }
  // check if fn is first in line for all its paths, and is
  // included in the first set for all its dir queues
  check(fn) {
    const { paths, dirs } = this.#getQueues(fn);
    return (
      paths.every((q) => q && q[0] === fn) &&
      dirs.every((q) => q && q[0] instanceof Set && q[0].has(fn))
    );
  }
  // run the function if it's first in line and not already running
  #run(fn) {
    if (this.#running.has(fn) || !this.check(fn)) {
      return false;
    }
    this.#running.add(fn);
    fn(() => this.#clear(fn));
    return true;
  }
  #clear(fn) {
    if (!this.#running.has(fn)) {
      return false;
    }
    const res = this.#reservations.get(fn);
    if (!res) {
      throw new Error("invalid reservation");
    }
    const { paths, dirs } = res;
    const next = /* @__PURE__ */ new Set();
    for (const path8 of paths) {
      const q = this.#queues.get(path8);
      if (!q || q?.[0] !== fn) {
        continue;
      }
      const q0 = q[1];
      if (!q0) {
        this.#queues.delete(path8);
        continue;
      }
      q.shift();
      if (typeof q0 === "function") {
        next.add(q0);
      } else {
        for (const f of q0) {
          next.add(f);
        }
      }
    }
    for (const dir of dirs) {
      const q = this.#queues.get(dir);
      const q0 = q?.[0];
      if (!q || !(q0 instanceof Set)) continue;
      if (q0.size === 1 && q.length === 1) {
        this.#queues.delete(dir);
        continue;
      } else if (q0.size === 1) {
        q.shift();
        const n = q[0];
        if (typeof n === "function") {
          next.add(n);
        }
      } else {
        q0.delete(fn);
      }
    }
    this.#running.delete(fn);
    next.forEach((fn2) => this.#run(fn2));
    return true;
  }
};

// node_modules/tar/dist/esm/unpack.js
var ONENTRY = Symbol("onEntry");
var CHECKFS = Symbol("checkFs");
var CHECKFS2 = Symbol("checkFs2");
var PRUNECACHE = Symbol("pruneCache");
var ISREUSABLE = Symbol("isReusable");
var MAKEFS = Symbol("makeFs");
var FILE2 = Symbol("file");
var DIRECTORY2 = Symbol("directory");
var LINK = Symbol("link");
var SYMLINK2 = Symbol("symlink");
var HARDLINK2 = Symbol("hardlink");
var UNSUPPORTED = Symbol("unsupported");
var CHECKPATH = Symbol("checkPath");
var MKDIR = Symbol("mkdir");
var ONERROR = Symbol("onError");
var PENDING = Symbol("pending");
var PEND = Symbol("pend");
var UNPEND = Symbol("unpend");
var ENDED3 = Symbol("ended");
var MAYBECLOSE = Symbol("maybeClose");
var SKIP = Symbol("skip");
var DOCHOWN = Symbol("doChown");
var UID = Symbol("uid");
var GID = Symbol("gid");
var CHECKED_CWD = Symbol("checkedCwd");
var platform5 = process.env.TESTING_TAR_FAKE_PLATFORM || process.platform;
var isWindows3 = platform5 === "win32";
var DEFAULT_MAX_DEPTH = 1024;
var unlinkFile = (path8, cb) => {
  if (!isWindows3) {
    return import_node_fs3.default.unlink(path8, cb);
  }
  const name2 =
    path8 +
    ".DELETE." +
    (0, import_node_crypto.randomBytes)(16).toString("hex");
  import_node_fs3.default.rename(path8, name2, (er) => {
    if (er) {
      return cb(er);
    }
    import_node_fs3.default.unlink(name2, cb);
  });
};
var unlinkFileSync = (path8) => {
  if (!isWindows3) {
    return import_node_fs3.default.unlinkSync(path8);
  }
  const name2 =
    path8 +
    ".DELETE." +
    (0, import_node_crypto.randomBytes)(16).toString("hex");
  import_node_fs3.default.renameSync(path8, name2);
  import_node_fs3.default.unlinkSync(name2);
};
var uint32 = (a, b, c) =>
  a !== void 0 && a === a >>> 0 ? a : b !== void 0 && b === b >>> 0 ? b : c;
var cacheKeyNormalize = (path8) =>
  stripTrailingSlashes(
    normalizeWindowsPath(normalizeUnicode(path8))
  ).toLowerCase();
var pruneCache = (cache, abs) => {
  abs = cacheKeyNormalize(abs);
  for (const path8 of cache.keys()) {
    const pnorm = cacheKeyNormalize(path8);
    if (pnorm === abs || pnorm.indexOf(abs + "/") === 0) {
      cache.delete(path8);
    }
  }
};
var dropCache = (cache) => {
  for (const key of cache.keys()) {
    cache.delete(key);
  }
};
var Unpack = class extends Parser {
  [ENDED3] = false;
  [CHECKED_CWD] = false;
  [PENDING] = 0;
  reservations = new PathReservations();
  transform;
  writable = true;
  readable = false;
  dirCache;
  uid;
  gid;
  setOwner;
  preserveOwner;
  processGid;
  processUid;
  maxDepth;
  forceChown;
  win32;
  newer;
  keep;
  noMtime;
  preservePaths;
  unlink;
  cwd;
  strip;
  processUmask;
  umask;
  dmode;
  fmode;
  chmod;
  constructor(opt = {}) {
    opt.ondone = () => {
      this[ENDED3] = true;
      this[MAYBECLOSE]();
    };
    super(opt);
    this.transform = opt.transform;
    this.dirCache = opt.dirCache || /* @__PURE__ */ new Map();
    this.chmod = !!opt.chmod;
    if (typeof opt.uid === "number" || typeof opt.gid === "number") {
      if (typeof opt.uid !== "number" || typeof opt.gid !== "number") {
        throw new TypeError("cannot set owner without number uid and gid");
      }
      if (opt.preserveOwner) {
        throw new TypeError(
          "cannot preserve owner in archive and also set owner explicitly"
        );
      }
      this.uid = opt.uid;
      this.gid = opt.gid;
      this.setOwner = true;
    } else {
      this.uid = void 0;
      this.gid = void 0;
      this.setOwner = false;
    }
    if (opt.preserveOwner === void 0 && typeof opt.uid !== "number") {
      this.preserveOwner = !!(process.getuid && process.getuid() === 0);
    } else {
      this.preserveOwner = !!opt.preserveOwner;
    }
    this.processUid =
      (this.preserveOwner || this.setOwner) && process.getuid
        ? process.getuid()
        : void 0;
    this.processGid =
      (this.preserveOwner || this.setOwner) && process.getgid
        ? process.getgid()
        : void 0;
    this.maxDepth =
      typeof opt.maxDepth === "number" ? opt.maxDepth : DEFAULT_MAX_DEPTH;
    this.forceChown = opt.forceChown === true;
    this.win32 = !!opt.win32 || isWindows3;
    this.newer = !!opt.newer;
    this.keep = !!opt.keep;
    this.noMtime = !!opt.noMtime;
    this.preservePaths = !!opt.preservePaths;
    this.unlink = !!opt.unlink;
    this.cwd = normalizeWindowsPath(
      import_node_path8.default.resolve(opt.cwd || process.cwd())
    );
    this.strip = Number(opt.strip) || 0;
    this.processUmask = !this.chmod
      ? 0
      : typeof opt.processUmask === "number"
      ? opt.processUmask
      : process.umask();
    this.umask = typeof opt.umask === "number" ? opt.umask : this.processUmask;
    this.dmode = opt.dmode || 511 & ~this.umask;
    this.fmode = opt.fmode || 438 & ~this.umask;
    this.on("entry", (entry) => this[ONENTRY](entry));
  }
  // a bad or damaged archive is a warning for Parser, but an error
  // when extracting.  Mark those errors as unrecoverable, because
  // the Unpack contract cannot be met.
  warn(code2, msg, data = {}) {
    if (code2 === "TAR_BAD_ARCHIVE" || code2 === "TAR_ABORT") {
      data.recoverable = false;
    }
    return super.warn(code2, msg, data);
  }
  [MAYBECLOSE]() {
    if (this[ENDED3] && this[PENDING] === 0) {
      this.emit("prefinish");
      this.emit("finish");
      this.emit("end");
    }
  }
  [CHECKPATH](entry) {
    const p = normalizeWindowsPath(entry.path);
    const parts = p.split("/");
    if (this.strip) {
      if (parts.length < this.strip) {
        return false;
      }
      if (entry.type === "Link") {
        const linkparts = normalizeWindowsPath(String(entry.linkpath)).split(
          "/"
        );
        if (linkparts.length >= this.strip) {
          entry.linkpath = linkparts.slice(this.strip).join("/");
        } else {
          return false;
        }
      }
      parts.splice(0, this.strip);
      entry.path = parts.join("/");
    }
    if (isFinite(this.maxDepth) && parts.length > this.maxDepth) {
      this.warn("TAR_ENTRY_ERROR", "path excessively deep", {
        entry,
        path: p,
        depth: parts.length,
        maxDepth: this.maxDepth,
      });
      return false;
    }
    if (!this.preservePaths) {
      if (
        parts.includes("..") /* c8 ignore next */ ||
        (isWindows3 && /^[a-z]:\.\.$/i.test(parts[0] ?? ""))
      ) {
        this.warn("TAR_ENTRY_ERROR", `path contains '..'`, {
          entry,
          path: p,
        });
        return false;
      }
      const [root, stripped] = stripAbsolutePath(p);
      if (root) {
        entry.path = String(stripped);
        this.warn("TAR_ENTRY_INFO", `stripping ${root} from absolute path`, {
          entry,
          path: p,
        });
      }
    }
    if (import_node_path8.default.isAbsolute(entry.path)) {
      entry.absolute = normalizeWindowsPath(
        import_node_path8.default.resolve(entry.path)
      );
    } else {
      entry.absolute = normalizeWindowsPath(
        import_node_path8.default.resolve(this.cwd, entry.path)
      );
    }
    if (
      !this.preservePaths &&
      typeof entry.absolute === "string" &&
      entry.absolute.indexOf(this.cwd + "/") !== 0 &&
      entry.absolute !== this.cwd
    ) {
      this.warn("TAR_ENTRY_ERROR", "path escaped extraction target", {
        entry,
        path: normalizeWindowsPath(entry.path),
        resolvedPath: entry.absolute,
        cwd: this.cwd,
      });
      return false;
    }
    if (
      entry.absolute === this.cwd &&
      entry.type !== "Directory" &&
      entry.type !== "GNUDumpDir"
    ) {
      return false;
    }
    if (this.win32) {
      const { root: aRoot } = import_node_path8.default.win32.parse(
        String(entry.absolute)
      );
      entry.absolute =
        aRoot + encode2(String(entry.absolute).slice(aRoot.length));
      const { root: pRoot } = import_node_path8.default.win32.parse(entry.path);
      entry.path = pRoot + encode2(entry.path.slice(pRoot.length));
    }
    return true;
  }
  [ONENTRY](entry) {
    if (!this[CHECKPATH](entry)) {
      return entry.resume();
    }
    import_node_assert.default.equal(typeof entry.absolute, "string");
    switch (entry.type) {
      case "Directory":
      case "GNUDumpDir":
        if (entry.mode) {
          entry.mode = entry.mode | 448;
        }
      // eslint-disable-next-line no-fallthrough
      case "File":
      case "OldFile":
      case "ContiguousFile":
      case "Link":
      case "SymbolicLink":
        return this[CHECKFS](entry);
      case "CharacterDevice":
      case "BlockDevice":
      case "FIFO":
      default:
        return this[UNSUPPORTED](entry);
    }
  }
  [ONERROR](er, entry) {
    if (er.name === "CwdError") {
      this.emit("error", er);
    } else {
      this.warn("TAR_ENTRY_ERROR", er, { entry });
      this[UNPEND]();
      entry.resume();
    }
  }
  [MKDIR](dir, mode, cb) {
    mkdir3(
      normalizeWindowsPath(dir),
      {
        uid: this.uid,
        gid: this.gid,
        processUid: this.processUid,
        processGid: this.processGid,
        umask: this.processUmask,
        preserve: this.preservePaths,
        unlink: this.unlink,
        cache: this.dirCache,
        cwd: this.cwd,
        mode,
      },
      cb
    );
  }
  [DOCHOWN](entry) {
    return (
      this.forceChown ||
      (this.preserveOwner &&
        ((typeof entry.uid === "number" && entry.uid !== this.processUid) ||
          (typeof entry.gid === "number" && entry.gid !== this.processGid))) ||
      (typeof this.uid === "number" && this.uid !== this.processUid) ||
      (typeof this.gid === "number" && this.gid !== this.processGid)
    );
  }
  [UID](entry) {
    return uint32(this.uid, entry.uid, this.processUid);
  }
  [GID](entry) {
    return uint32(this.gid, entry.gid, this.processGid);
  }
  [FILE2](entry, fullyDone) {
    const mode =
      typeof entry.mode === "number" ? entry.mode & 4095 : this.fmode;
    const stream = new WriteStream(String(entry.absolute), {
      // slight lie, but it can be numeric flags
      flags: getWriteFlag(entry.size),
      mode,
      autoClose: false,
    });
    stream.on("error", (er) => {
      if (stream.fd) {
        import_node_fs3.default.close(stream.fd, () => {});
      }
      stream.write = () => true;
      this[ONERROR](er, entry);
      fullyDone();
    });
    let actions = 1;
    const done = (er) => {
      if (er) {
        if (stream.fd) {
          import_node_fs3.default.close(stream.fd, () => {});
        }
        this[ONERROR](er, entry);
        fullyDone();
        return;
      }
      if (--actions === 0) {
        if (stream.fd !== void 0) {
          import_node_fs3.default.close(stream.fd, (er2) => {
            if (er2) {
              this[ONERROR](er2, entry);
            } else {
              this[UNPEND]();
            }
            fullyDone();
          });
        }
      }
    };
    stream.on("finish", () => {
      const abs = String(entry.absolute);
      const fd = stream.fd;
      if (typeof fd === "number" && entry.mtime && !this.noMtime) {
        actions++;
        const atime = entry.atime || /* @__PURE__ */ new Date();
        const mtime = entry.mtime;
        import_node_fs3.default.futimes(fd, atime, mtime, (er) =>
          er
            ? import_node_fs3.default.utimes(abs, atime, mtime, (er2) =>
                done(er2 && er)
              )
            : done()
        );
      }
      if (typeof fd === "number" && this[DOCHOWN](entry)) {
        actions++;
        const uid = this[UID](entry);
        const gid = this[GID](entry);
        if (typeof uid === "number" && typeof gid === "number") {
          import_node_fs3.default.fchown(fd, uid, gid, (er) =>
            er
              ? import_node_fs3.default.chown(abs, uid, gid, (er2) =>
                  done(er2 && er)
                )
              : done()
          );
        }
      }
      done();
    });
    const tx = this.transform ? this.transform(entry) || entry : entry;
    if (tx !== entry) {
      tx.on("error", (er) => {
        this[ONERROR](er, entry);
        fullyDone();
      });
      entry.pipe(tx);
    }
    tx.pipe(stream);
  }
  [DIRECTORY2](entry, fullyDone) {
    const mode =
      typeof entry.mode === "number" ? entry.mode & 4095 : this.dmode;
    this[MKDIR](String(entry.absolute), mode, (er) => {
      if (er) {
        this[ONERROR](er, entry);
        fullyDone();
        return;
      }
      let actions = 1;
      const done = () => {
        if (--actions === 0) {
          fullyDone();
          this[UNPEND]();
          entry.resume();
        }
      };
      if (entry.mtime && !this.noMtime) {
        actions++;
        import_node_fs3.default.utimes(
          String(entry.absolute),
          entry.atime || /* @__PURE__ */ new Date(),
          entry.mtime,
          done
        );
      }
      if (this[DOCHOWN](entry)) {
        actions++;
        import_node_fs3.default.chown(
          String(entry.absolute),
          Number(this[UID](entry)),
          Number(this[GID](entry)),
          done
        );
      }
      done();
    });
  }
  [UNSUPPORTED](entry) {
    entry.unsupported = true;
    this.warn(
      "TAR_ENTRY_UNSUPPORTED",
      `unsupported entry type: ${entry.type}`,
      { entry }
    );
    entry.resume();
  }
  [SYMLINK2](entry, done) {
    this[LINK](entry, String(entry.linkpath), "symlink", done);
  }
  [HARDLINK2](entry, done) {
    const linkpath = normalizeWindowsPath(
      import_node_path8.default.resolve(this.cwd, String(entry.linkpath))
    );
    this[LINK](entry, linkpath, "link", done);
  }
  [PEND]() {
    this[PENDING]++;
  }
  [UNPEND]() {
    this[PENDING]--;
    this[MAYBECLOSE]();
  }
  [SKIP](entry) {
    this[UNPEND]();
    entry.resume();
  }
  // Check if we can reuse an existing filesystem entry safely and
  // overwrite it, rather than unlinking and recreating
  // Windows doesn't report a useful nlink, so we just never reuse entries
  [ISREUSABLE](entry, st) {
    return (
      entry.type === "File" &&
      !this.unlink &&
      st.isFile() &&
      st.nlink <= 1 &&
      !isWindows3
    );
  }
  // check if a thing is there, and if so, try to clobber it
  [CHECKFS](entry) {
    this[PEND]();
    const paths = [entry.path];
    if (entry.linkpath) {
      paths.push(entry.linkpath);
    }
    this.reservations.reserve(paths, (done) => this[CHECKFS2](entry, done));
  }
  [PRUNECACHE](entry) {
    if (entry.type === "SymbolicLink") {
      dropCache(this.dirCache);
    } else if (entry.type !== "Directory") {
      pruneCache(this.dirCache, String(entry.absolute));
    }
  }
  [CHECKFS2](entry, fullyDone) {
    this[PRUNECACHE](entry);
    const done = (er) => {
      this[PRUNECACHE](entry);
      fullyDone(er);
    };
    const checkCwd2 = () => {
      this[MKDIR](this.cwd, this.dmode, (er) => {
        if (er) {
          this[ONERROR](er, entry);
          done();
          return;
        }
        this[CHECKED_CWD] = true;
        start();
      });
    };
    const start = () => {
      if (entry.absolute !== this.cwd) {
        const parent = normalizeWindowsPath(
          import_node_path8.default.dirname(String(entry.absolute))
        );
        if (parent !== this.cwd) {
          return this[MKDIR](parent, this.dmode, (er) => {
            if (er) {
              this[ONERROR](er, entry);
              done();
              return;
            }
            afterMakeParent();
          });
        }
      }
      afterMakeParent();
    };
    const afterMakeParent = () => {
      import_node_fs3.default.lstat(String(entry.absolute), (lstatEr, st) => {
        if (
          st &&
          (this.keep /* c8 ignore next */ ||
            (this.newer && st.mtime > (entry.mtime ?? st.mtime)))
        ) {
          this[SKIP](entry);
          done();
          return;
        }
        if (lstatEr || this[ISREUSABLE](entry, st)) {
          return this[MAKEFS](null, entry, done);
        }
        if (st.isDirectory()) {
          if (entry.type === "Directory") {
            const needChmod =
              this.chmod && entry.mode && (st.mode & 4095) !== entry.mode;
            const afterChmod = (er) => this[MAKEFS](er ?? null, entry, done);
            if (!needChmod) {
              return afterChmod();
            }
            return import_node_fs3.default.chmod(
              String(entry.absolute),
              Number(entry.mode),
              afterChmod
            );
          }
          if (entry.absolute !== this.cwd) {
            return import_node_fs3.default.rmdir(String(entry.absolute), (er) =>
              this[MAKEFS](er ?? null, entry, done)
            );
          }
        }
        if (entry.absolute === this.cwd) {
          return this[MAKEFS](null, entry, done);
        }
        unlinkFile(String(entry.absolute), (er) =>
          this[MAKEFS](er ?? null, entry, done)
        );
      });
    };
    if (this[CHECKED_CWD]) {
      start();
    } else {
      checkCwd2();
    }
  }
  [MAKEFS](er, entry, done) {
    if (er) {
      this[ONERROR](er, entry);
      done();
      return;
    }
    switch (entry.type) {
      case "File":
      case "OldFile":
      case "ContiguousFile":
        return this[FILE2](entry, done);
      case "Link":
        return this[HARDLINK2](entry, done);
      case "SymbolicLink":
        return this[SYMLINK2](entry, done);
      case "Directory":
      case "GNUDumpDir":
        return this[DIRECTORY2](entry, done);
    }
  }
  [LINK](entry, linkpath, link, done) {
    import_node_fs3.default[link](linkpath, String(entry.absolute), (er) => {
      if (er) {
        this[ONERROR](er, entry);
      } else {
        this[UNPEND]();
        entry.resume();
      }
      done();
    });
  }
};
var callSync = (fn) => {
  try {
    return [null, fn()];
  } catch (er) {
    return [er, null];
  }
};
var UnpackSync = class extends Unpack {
  sync = true;
  [MAKEFS](er, entry) {
    return super[MAKEFS](er, entry, () => {});
  }
  [CHECKFS](entry) {
    this[PRUNECACHE](entry);
    if (!this[CHECKED_CWD]) {
      const er2 = this[MKDIR](this.cwd, this.dmode);
      if (er2) {
        return this[ONERROR](er2, entry);
      }
      this[CHECKED_CWD] = true;
    }
    if (entry.absolute !== this.cwd) {
      const parent = normalizeWindowsPath(
        import_node_path8.default.dirname(String(entry.absolute))
      );
      if (parent !== this.cwd) {
        const mkParent = this[MKDIR](parent, this.dmode);
        if (mkParent) {
          return this[ONERROR](mkParent, entry);
        }
      }
    }
    const [lstatEr, st] = callSync(() =>
      import_node_fs3.default.lstatSync(String(entry.absolute))
    );
    if (
      st &&
      (this.keep /* c8 ignore next */ ||
        (this.newer && st.mtime > (entry.mtime ?? st.mtime)))
    ) {
      return this[SKIP](entry);
    }
    if (lstatEr || this[ISREUSABLE](entry, st)) {
      return this[MAKEFS](null, entry);
    }
    if (st.isDirectory()) {
      if (entry.type === "Directory") {
        const needChmod =
          this.chmod && entry.mode && (st.mode & 4095) !== entry.mode;
        const [er3] = needChmod
          ? callSync(() => {
              import_node_fs3.default.chmodSync(
                String(entry.absolute),
                Number(entry.mode)
              );
            })
          : [];
        return this[MAKEFS](er3, entry);
      }
      const [er2] = callSync(() =>
        import_node_fs3.default.rmdirSync(String(entry.absolute))
      );
      this[MAKEFS](er2, entry);
    }
    const [er] =
      entry.absolute === this.cwd
        ? []
        : callSync(() => unlinkFileSync(String(entry.absolute)));
    this[MAKEFS](er, entry);
  }
  [FILE2](entry, done) {
    const mode =
      typeof entry.mode === "number" ? entry.mode & 4095 : this.fmode;
    const oner = (er) => {
      let closeError;
      try {
        import_node_fs3.default.closeSync(fd);
      } catch (e) {
        closeError = e;
      }
      if (er || closeError) {
        this[ONERROR](er || closeError, entry);
      }
      done();
    };
    let fd;
    try {
      fd = import_node_fs3.default.openSync(
        String(entry.absolute),
        getWriteFlag(entry.size),
        mode
      );
    } catch (er) {
      return oner(er);
    }
    const tx = this.transform ? this.transform(entry) || entry : entry;
    if (tx !== entry) {
      tx.on("error", (er) => this[ONERROR](er, entry));
      entry.pipe(tx);
    }
    tx.on("data", (chunk) => {
      try {
        import_node_fs3.default.writeSync(fd, chunk, 0, chunk.length);
      } catch (er) {
        oner(er);
      }
    });
    tx.on("end", () => {
      let er = null;
      if (entry.mtime && !this.noMtime) {
        const atime = entry.atime || /* @__PURE__ */ new Date();
        const mtime = entry.mtime;
        try {
          import_node_fs3.default.futimesSync(fd, atime, mtime);
        } catch (futimeser) {
          try {
            import_node_fs3.default.utimesSync(
              String(entry.absolute),
              atime,
              mtime
            );
          } catch (utimeser) {
            er = futimeser;
          }
        }
      }
      if (this[DOCHOWN](entry)) {
        const uid = this[UID](entry);
        const gid = this[GID](entry);
        try {
          import_node_fs3.default.fchownSync(fd, Number(uid), Number(gid));
        } catch (fchowner) {
          try {
            import_node_fs3.default.chownSync(
              String(entry.absolute),
              Number(uid),
              Number(gid)
            );
          } catch (chowner) {
            er = er || fchowner;
          }
        }
      }
      oner(er);
    });
  }
  [DIRECTORY2](entry, done) {
    const mode =
      typeof entry.mode === "number" ? entry.mode & 4095 : this.dmode;
    const er = this[MKDIR](String(entry.absolute), mode);
    if (er) {
      this[ONERROR](er, entry);
      done();
      return;
    }
    if (entry.mtime && !this.noMtime) {
      try {
        import_node_fs3.default.utimesSync(
          String(entry.absolute),
          entry.atime || /* @__PURE__ */ new Date(),
          entry.mtime
        );
      } catch (er2) {}
    }
    if (this[DOCHOWN](entry)) {
      try {
        import_node_fs3.default.chownSync(
          String(entry.absolute),
          Number(this[UID](entry)),
          Number(this[GID](entry))
        );
      } catch (er2) {}
    }
    done();
    entry.resume();
  }
  [MKDIR](dir, mode) {
    try {
      return mkdirSync3(normalizeWindowsPath(dir), {
        uid: this.uid,
        gid: this.gid,
        processUid: this.processUid,
        processGid: this.processGid,
        umask: this.processUmask,
        preserve: this.preservePaths,
        unlink: this.unlink,
        cache: this.dirCache,
        cwd: this.cwd,
        mode,
      });
    } catch (er) {
      return er;
    }
  }
  [LINK](entry, linkpath, link, done) {
    const ls = `${link}Sync`;
    try {
      import_node_fs3.default[ls](linkpath, String(entry.absolute));
      done();
      entry.resume();
    } catch (er) {
      return this[ONERROR](er, entry);
    }
  }
};

// node_modules/tar/dist/esm/extract.js
var extractFileSync = (opt) => {
  const u = new UnpackSync(opt);
  const file = opt.file;
  const stat2 = import_node_fs4.default.statSync(file);
  const readSize = opt.maxReadSize || 16 * 1024 * 1024;
  const stream = new ReadStreamSync(file, {
    readSize,
    size: stat2.size,
  });
  stream.pipe(u);
};
var extractFile = (opt, _) => {
  const u = new Unpack(opt);
  const readSize = opt.maxReadSize || 16 * 1024 * 1024;
  const file = opt.file;
  const p = new Promise((resolve2, reject) => {
    u.on("error", reject);
    u.on("close", resolve2);
    import_node_fs4.default.stat(file, (er, stat2) => {
      if (er) {
        reject(er);
      } else {
        const stream = new ReadStream(file, {
          readSize,
          size: stat2.size,
        });
        stream.on("error", reject);
        stream.pipe(u);
      }
    });
  });
  return p;
};
var extract = makeCommand(
  extractFileSync,
  extractFile,
  (opt) => new UnpackSync(opt),
  (opt) => new Unpack(opt),
  (opt, files) => {
    if (files?.length) filesFilter(opt, files);
  }
);

// node_modules/tar/dist/esm/replace.js
var import_node_fs5 = __toESM(require("node:fs"), 1);
var import_node_path9 = __toESM(require("node:path"), 1);
var replaceSync = (opt, files) => {
  const p = new PackSync(opt);
  let threw = true;
  let fd;
  let position;
  try {
    try {
      fd = import_node_fs5.default.openSync(opt.file, "r+");
    } catch (er) {
      if (er?.code === "ENOENT") {
        fd = import_node_fs5.default.openSync(opt.file, "w+");
      } else {
        throw er;
      }
    }
    const st = import_node_fs5.default.fstatSync(fd);
    const headBuf = Buffer.alloc(512);
    POSITION: for (position = 0; position < st.size; position += 512) {
      for (let bufPos = 0, bytes = 0; bufPos < 512; bufPos += bytes) {
        bytes = import_node_fs5.default.readSync(
          fd,
          headBuf,
          bufPos,
          headBuf.length - bufPos,
          position + bufPos
        );
        if (position === 0 && headBuf[0] === 31 && headBuf[1] === 139) {
          throw new Error("cannot append to compressed archives");
        }
        if (!bytes) {
          break POSITION;
        }
      }
      const h = new Header(headBuf);
      if (!h.cksumValid) {
        break;
      }
      const entryBlockSize = 512 * Math.ceil((h.size || 0) / 512);
      if (position + entryBlockSize + 512 > st.size) {
        break;
      }
      position += entryBlockSize;
      if (opt.mtimeCache && h.mtime) {
        opt.mtimeCache.set(String(h.path), h.mtime);
      }
    }
    threw = false;
    streamSync(opt, p, position, fd, files);
  } finally {
    if (threw) {
      try {
        import_node_fs5.default.closeSync(fd);
      } catch (er) {}
    }
  }
};
var streamSync = (opt, p, position, fd, files) => {
  const stream = new WriteStreamSync(opt.file, {
    fd,
    start: position,
  });
  p.pipe(stream);
  addFilesSync2(p, files);
};
var replaceAsync = (opt, files) => {
  files = Array.from(files);
  const p = new Pack(opt);
  const getPos = (fd, size, cb_) => {
    const cb = (er, pos2) => {
      if (er) {
        import_node_fs5.default.close(fd, (_) => cb_(er));
      } else {
        cb_(null, pos2);
      }
    };
    let position = 0;
    if (size === 0) {
      return cb(null, 0);
    }
    let bufPos = 0;
    const headBuf = Buffer.alloc(512);
    const onread = (er, bytes) => {
      if (er || typeof bytes === "undefined") {
        return cb(er);
      }
      bufPos += bytes;
      if (bufPos < 512 && bytes) {
        return import_node_fs5.default.read(
          fd,
          headBuf,
          bufPos,
          headBuf.length - bufPos,
          position + bufPos,
          onread
        );
      }
      if (position === 0 && headBuf[0] === 31 && headBuf[1] === 139) {
        return cb(new Error("cannot append to compressed archives"));
      }
      if (bufPos < 512) {
        return cb(null, position);
      }
      const h = new Header(headBuf);
      if (!h.cksumValid) {
        return cb(null, position);
      }
      const entryBlockSize = 512 * Math.ceil((h.size ?? 0) / 512);
      if (position + entryBlockSize + 512 > size) {
        return cb(null, position);
      }
      position += entryBlockSize + 512;
      if (position >= size) {
        return cb(null, position);
      }
      if (opt.mtimeCache && h.mtime) {
        opt.mtimeCache.set(String(h.path), h.mtime);
      }
      bufPos = 0;
      import_node_fs5.default.read(fd, headBuf, 0, 512, position, onread);
    };
    import_node_fs5.default.read(fd, headBuf, 0, 512, position, onread);
  };
  const promise = new Promise((resolve2, reject) => {
    p.on("error", reject);
    let flag = "r+";
    const onopen = (er, fd) => {
      if (er && er.code === "ENOENT" && flag === "r+") {
        flag = "w+";
        return import_node_fs5.default.open(opt.file, flag, onopen);
      }
      if (er || !fd) {
        return reject(er);
      }
      import_node_fs5.default.fstat(fd, (er2, st) => {
        if (er2) {
          return import_node_fs5.default.close(fd, () => reject(er2));
        }
        getPos(fd, st.size, (er3, position) => {
          if (er3) {
            return reject(er3);
          }
          const stream = new WriteStream(opt.file, {
            fd,
            start: position,
          });
          p.pipe(stream);
          stream.on("error", reject);
          stream.on("close", resolve2);
          addFilesAsync2(p, files);
        });
      });
    };
    import_node_fs5.default.open(opt.file, flag, onopen);
  });
  return promise;
};
var addFilesSync2 = (p, files) => {
  files.forEach((file) => {
    if (file.charAt(0) === "@") {
      list({
        file: import_node_path9.default.resolve(p.cwd, file.slice(1)),
        sync: true,
        noResume: true,
        onReadEntry: (entry) => p.add(entry),
      });
    } else {
      p.add(file);
    }
  });
  p.end();
};
var addFilesAsync2 = async (p, files) => {
  for (let i = 0; i < files.length; i++) {
    const file = String(files[i]);
    if (file.charAt(0) === "@") {
      await list({
        file: import_node_path9.default.resolve(String(p.cwd), file.slice(1)),
        noResume: true,
        onReadEntry: (entry) => p.add(entry),
      });
    } else {
      p.add(file);
    }
  }
  p.end();
};
var replace = makeCommand(
  replaceSync,
  replaceAsync,
  /* c8 ignore start */
  () => {
    throw new TypeError("file is required");
  },
  () => {
    throw new TypeError("file is required");
  },
  /* c8 ignore stop */
  (opt, entries) => {
    if (!isFile(opt)) {
      throw new TypeError("file is required");
    }
    if (
      opt.gzip ||
      opt.brotli ||
      opt.file.endsWith(".br") ||
      opt.file.endsWith(".tbr")
    ) {
      throw new TypeError("cannot append to compressed archives");
    }
    if (!entries?.length) {
      throw new TypeError("no paths specified to add/replace");
    }
  }
);

// node_modules/tar/dist/esm/update.js
var update = makeCommand(
  replace.syncFile,
  replace.asyncFile,
  replace.syncNoFile,
  replace.asyncNoFile,
  (opt, entries = []) => {
    replace.validate?.(opt, entries);
    mtimeFilter(opt);
  }
);
var mtimeFilter = (opt) => {
  const filter = opt.filter;
  if (!opt.mtimeCache) {
    opt.mtimeCache = /* @__PURE__ */ new Map();
  }
  opt.filter = filter
    ? (path8, stat2) =>
        filter(path8, stat2) &&
        !(
          /* c8 ignore start */
          (
            (opt.mtimeCache?.get(path8) ?? stat2.mtime ?? 0) >
            (stat2.mtime ?? 0)
          )
        )
    : (path8, stat2) =>
        !(
          /* c8 ignore start */
          (
            (opt.mtimeCache?.get(path8) ?? stat2.mtime ?? 0) >
            (stat2.mtime ?? 0)
          )
        );
};
// Annotate the CommonJS export names for ESM import in node:
0 &&
  (module.exports = {
    Header,
    Pack,
    PackJob,
    PackSync,
    Parser,
    Pax,
    ReadEntry,
    Unpack,
    UnpackSync,
    WriteEntry,
    WriteEntrySync,
    WriteEntryTar,
    c,
    create,
    extract,
    filesFilter,
    list,
    r,
    replace,
    t,
    types,
    u,
    update,
    x,
  });
