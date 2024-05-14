export * from './b'
import('./c').then((m) => console.log(m.c))
export const a = 'a'
