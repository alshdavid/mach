import * as path from 'node:path';
import * as fs from 'node:fs';
import * as child_process from 'node:child_process'
import { Paths } from '../platform/paths.mjs'
import { crawlDir, TargetType } from '../platform/crawl.mjs'

const COPIES = process.env.BENCH_COPIES ? parseInt(process.env.BENCH_COPIES, 10) : 50
const STDIO = process.env.STDIO | 'ignore'
const GENERATE_FIXTURE = process.env.GENERATE_FIXTURE
const NO_OPTIMIZE = process.env.NO_OPTIMIZE

const TESTERS = [
  "mach",
  "esbuild",
  "parcel",
  "webpack",
  "rspack",
]

let generate_fixture = true

if (fs.existsSync(path.join(Paths.ScriptsTmp, 'bench_copies'))) {
  let current_copies = fs.readFileSync(path.join(Paths.ScriptsTmp, 'bench_copies'), 'utf8')
  if (current_copies == COPIES) {
    generate_fixture = false
  } else {
    fs.rmSync(path.join(Paths.ScriptsTmp, 'bench_copies'), { recursive: true, force: true })
  }
}

const bench_dir = path.join(Paths.Benchmarks)
const $ = (cmd, cwd = bench_dir, stdio = 'inherit', options = {}) => {
  console.log(cmd)
  child_process.execSync(cmd, { cwd, stdio, ...options })
}

if (GENERATE_FIXTURE || generate_fixture) {
  console.log("Generating fixtures")

  fs.rmSync(path.join(bench_dir, 'three-js'), { recursive: true, force: true })
  for (const tester of TESTERS) {
    fs.rmSync(path.join(bench_dir, tester, 'src'), { recursive: true, force: true })
  }

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

  for (const tester of TESTERS) {
    fs.cpSync(path.join(bench_dir, 'three-js', 'src'), path.join(bench_dir, tester, 'src'), { recursive: true })
  }
} else {
  console.log("Skip: Generating fixtures")
}

if (!fs.existsSync(path.join(Paths.Benchmarks, 'node_modules'))) {
  $('pnpm install')
}

fs.writeFileSync(path.join(Paths.ScriptsTmp, 'bench_copies'), `${COPIES}`, 'utf8')

const timings = {}

for (const tester of TESTERS) {
  console.log('Running:', tester)
  console.log()
  let package_json = JSON.parse(fs.readFileSync(path.join(Paths.Benchmarks, tester, 'package.json'), 'utf8'))
  $(package_json.scripts['clean'], path.join(Paths.Benchmarks, tester), STDIO)

  let start_time = Date.now()
  let cmd = NO_OPTIMIZE != undefined ? 'build' : 'build:optimize' 
  $(package_json.scripts[cmd], path.join(Paths.Benchmarks, tester), STDIO)
  let end_time = Date.now() - start_time
  timings[tester] = end_time / 1000
  console.log()
}

console.table(timings)
