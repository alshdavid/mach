import fs from 'node:fs'
import path from 'node:path'

let cwd = process.cwd();
let [,,target, from, to] = process.argv

if (!path.isAbsolute(target)) {
  target = path.join(cwd, target)
}

const original = fs.readFileSync(target, 'utf8')
const update = original.replaceAll(from, to)
fs.writeFileSync(target, update, 'utf8')
