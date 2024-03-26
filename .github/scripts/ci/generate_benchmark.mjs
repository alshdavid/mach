import * as fs from 'node:fs';
import * as child_process from 'node:child_process'
import { Paths as ProjectPaths, genPath } from '../platform/paths.mjs'
import { crawlDir, TargetType } from '../platform/crawl.mjs'

const PROJECT = process.env.PROJECT
const COPIES = process.env.BENCH_COPIES ? parseInt(process.env.BENCH_COPIES, 10) : 50

if (!PROJECT) {
  console.error("No PROJECT selected")
  process.exit(1)
}

const Paths = {
  Output: genPath(ProjectPaths.Root('benchmarks', `${PROJECT}_${COPIES}`)),
  OutputSrc: genPath(ProjectPaths.Root('benchmarks', `${PROJECT}_${COPIES}`, 'src')),
  SourceFixture: genPath(genPath(ProjectPaths.Root('benchmarks', PROJECT))),
  BenchmarksSource: genPath(ProjectPaths.Root('.github', 'benchmarks')),
  VendorTar: genPath(ProjectPaths.Root('.github', 'benchmarks', 'three-js.tar.gz')),
  VendorDir: genPath(ProjectPaths.Root('.github', 'benchmarks', 'three-js')),
}

const $ = (cmd, cwd, stdio = 'inherit', options = {}) => {
  console.log(cwd, cmd)
  child_process.execSync(cmd, { cwd, stdio, ...options })
}

if (!fs.existsSync(Paths.VendorDir())) {
  console.log("Extracting fixtures")
  fs.mkdirSync(Paths.VendorDir(), { recursive: true })
  $(`tar -xzvf three-js.tar.gz -C three-js`, Paths.BenchmarksSource())
}

if (!fs.existsSync(Paths.Output())) {
  console.log("Generating fixtures")
  
  if (fs.existsSync(Paths.Output())) {
    fs.rmSync(Paths.Output(), { recursive: true, force: true })
  }
  fs.cpSync(Paths.BenchmarksSource(PROJECT), Paths.Output(), { recursive: true })

  let index = ''
  let count = 0

  for (let i = 1; i <= COPIES; i++) {
    process.stdout.write(`${i} `)
    index += `import * as three_js_copy_${i} from './copy_${i}/Three.js'; export { three_js_copy_${i} }; window.three_js_copy_${i} = three_js_copy_${i};\n`

    const copy_dir = Paths.OutputSrc(`copy_${i}`)
    fs.cpSync(Paths.VendorDir('_src'), copy_dir, { recursive: true })

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
  process.stdout.write(`\n`)

  fs.writeFileSync(Paths.OutputSrc('index.js'), index, 'utf8')

  $('pnpm install', Paths.Output())
}

console.log('Complete')
