export class Dependency {
  #ref

  constructor(ref) {
    this.#ref = ref
  }

  get id() {
    return globalThis.Mach.ops.getter_dependency(this.#ref, 'id')
  }
  
  get specifier() {
    return globalThis.Mach.ops.getter_dependency(this.#ref, 'specifier')
  }
  
  get specifierType() {
    return globalThis.Mach.ops.getter_dependency(this.#ref, 'specifier_type')
  }
  
  get isEntry() {
    return globalThis.Mach.ops.getter_dependency(this.#ref, 'is_entry')
  }
  
  get priority() {
    return globalThis.Mach.ops.getter_dependency(this.#ref, 'priority')
  }
  
  get sourcePath() {
    return globalThis.Mach.ops.getter_dependency(this.#ref, 'source_path')
  }
  
  get sourceAsset() {
    return globalThis.Mach.ops.getter_dependency(this.#ref, 'source_asset')
  }
  
  get resolveFrom() {
    return globalThis.Mach.ops.getter_dependency(this.#ref, 'resolve_from')
  }
  
  get importedSymbols() {
    return globalThis.Mach.ops.getter_dependency(this.#ref, 'imported_symbols')
  }
  
  get bundleBehavior() {
    return globalThis.Mach.ops.getter_dependency(this.#ref, 'bundle_behavior')
  }
}
