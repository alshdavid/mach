import { Mach } from '@alshdavid/mach'

const mach = new Mach()

const result = await mach.build({
  entries: ['./index.js']
})

// console.log({ result })