import * as path from 'node:path';
import * as fs from 'node:fs';
import * as crypto from 'node:crypto'
import { crawlDir, TargetType } from '../platform/crawl.mjs'
import { Paths } from '../platform/paths.mjs'

let [,, action] = process.argv
if (action !== 'read' || action !== 'set') {

}

let results = crawlDir({
  targetPath: path.join(Paths.Root, 'npm', 'node-adapter'),
  dontCrawl: ['node_modules', 'lib', 'types'],
  match: [TargetType.FILE]
})

let hashes = {}

for (const filepath of results.keys()) {
  let contents = fs.readFileSync(filepath)
  let hash = crypto.createHash('sha256').update(JSON.stringify(contents)).digest('hex')
  hashes[filepath] = hash
}

let sum = crypto.createHash('sha256').update(JSON.stringify(hashes)).digest('hex')
let target_file = path.join(Paths.Scripts, '.sums', 'node-adapter')

if (action === 'set') {
  fs.mkdirSync(path.dirname(target_file), { recursive: true })
  fs.writeFileSync(target_file, sum, 'utf8')
  process.exit()
}

if (fs.existsSync(target_file)) {
  let existing = fs.readFileSync(target_file, 'utf8')
  if (existing === sum) {
    // don't build
    console.log(false)
    process.exit()
  }
}

// do a build
console.log(true)