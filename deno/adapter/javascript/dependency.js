export class Dependency {
  #ref

  constructor(ref) {
    this.#ref = ref
  }

  get id() {
    return globalThis.Mach.ops.getter_dependency(this.#ref, 0)
  }
  
  get specifier() {
    return globalThis.Mach.ops.getter_dependency(this.#ref, 1)
  }
  
  get specifierType() {
    return globalThis.Mach.ops.getter_dependency(this.#ref, 2)
  }
  
  get isEntry() {
    return globalThis.Mach.ops.getter_dependency(this.#ref, 3)
  }
  
  get priority() {
    return globalThis.Mach.ops.getter_dependency(this.#ref, 4)
  }
  
  get sourcePath() {
    return globalThis.Mach.ops.getter_dependency(this.#ref, 5)
  }
  
  get sourceAsset() {
    return globalThis.Mach.ops.getter_dependency(this.#ref, 6)
  }
  
  get resolveFrom() {
    return globalThis.Mach.ops.getter_dependency(this.#ref, 7)
  }
  
  get importedSymbols() {
    return globalThis.Mach.ops.getter_dependency(this.#ref, 8)
  }
  
  get bundleBehavior() {
    return globalThis.Mach.ops.getter_dependency(this.#ref, 9)
  }
}
