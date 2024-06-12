import { Mach } from '@alshdavid/mach'

const mach = new Mach({
  nodeWorkers: 1,
})

const result = await mach.build({
  entries: ['./index.js'],
})


await new Promise(res => setTimeout(res, 2000))
// console.log({ result })
