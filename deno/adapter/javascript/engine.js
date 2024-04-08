import { Resolver } from "./platform/resolver.js";
import { Dependency } from "./platform/dependency.js";
import { MessageType } from "./platform/types.js";

const resolvers = new Map();

export async function main({ thread_id }) {
  globalThis.Mach.ops.connect(async ([action, ...data]) => {
    if (action === MessageType.Ping) {
      const [msg] = data
      console.log(`[${thread_id}] Received:`, msg);
      return
    }

    if (action === MessageType.ResolverLoad) {
      const [specifier] = data
      const { default: resolver } = await import(specifier)
      resolvers.set(specifier, resolver)
      return
    }

    if (action === MessageType.ResolverRun) {
      const [resolver_id, dependency_ref] = data
      const resolver = resolvers.get(resolver_id)
      const dependency = new Dependency(dependency_ref)
      return await resolver.resolve({dependency})
    }
  });
}

globalThis.Mach.plugins = { Resolver };
