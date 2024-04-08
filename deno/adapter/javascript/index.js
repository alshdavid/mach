const THREADS = Deno.args[0] ? parseInt(Deno.args[0], 10): 1

const workers = []

// the main thread is 1 thread
for (let i = 1; i < THREADS; i++) {
  workers.push(new Worker(import.meta.resolve(`./index.worker.js?i=${i+1}`), { type: "module" }));
}

await (await import('./engine.js')).main({ thread_id: 1 })

for (const worker of workers) {
  worker.terminate()
}
