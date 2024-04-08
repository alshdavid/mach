const THREAD_ID = parseInt(new URL(import.meta.url).searchParams.get('i'), 10) || 0

await (await import('./engine.js')).main({ thread_id: THREAD_ID })
