import * as path from 'node:path';
import * as fs from 'node:fs';
import * as child_process from 'node:child_process'
import { Paths } from '../platform/paths.mjs'
import { crawlDir, TargetType } from '../platform/crawl.mjs'

const COPIES = process.env.BENCH_COPIES ? parseInt(process.env.BENCH_COPIES, 10) : 1

const bench_dir = path.join(Paths.Benchmarks)
const $ = (cmd, cwd = bench_dir, stdio = 'inherit', options = {}) => {
  console.log(cmd)
  child_process.execSync(cmd, { cwd, stdio, ...options })
}

fs.rmSync(path.join(bench_dir, 'three-js'), { recursive: true, force: true })
fs.mkdirSync(path.join(bench_dir, 'three-js'), { recursive: true })

$(`tar -xzvf three-js.tar.gz -C three-js`)

fs.mkdirSync(path.join(bench_dir, 'three-js', 'src'), { recursive: true })

let index = ''
let count = 0

for (let i = 1; i <= COPIES; i++) {
   index += `import * as three_js_copy_${i} from './copy_${i}/Three.js'; export { three_js_copy_${i} }; window.three_js_copy_${i} = three_js_copy_${i};\n`

   const copy_dir = path.join(bench_dir, 'three-js', 'src', `copy_${i}`);
   fs.cpSync(path.join(bench_dir, 'three-js', '_src'), copy_dir, { recursive: true })

   let results = crawlDir({
    targetPath: copy_dir,
    dontCrawl: [],
    match: [TargetType.FILE]
  })

  for (const filepath of results.keys()) {
    fs.appendFileSync(filepath, `\nexport const unique_id_${count} = ${count};`)
    count += 1
  }
}

fs.writeFileSync(path.join(bench_dir, 'three-js', 'src', 'index.js'), index, 'utf8')

for (const tester of [
  "mach",
  "esbuild",
  "parcel",
  // "webpack",
  // "rspack",
]) {
  fs.cpSync(path.join(bench_dir, 'three-js', 'src'), path.join(bench_dir, tester, 'src'), { recursive: true })
}

$('pnpm install')
