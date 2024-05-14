import './style.css'

const m = await import('./a.js')
console.log({ value: m.value, env_value: m.env_value })
