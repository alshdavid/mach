import path from 'node:path'
import * as url from 'node:url';

const __dirname = url.fileURLToPath(new URL('.', import.meta.url));
const __root = path.resolve(__dirname, '..', '..', '..')

export const Paths = Object.freeze({
  Root: __root,
  Scripts: path.join(__root, '.github', 'scripts'),
  ScriptsTmp: path.join(__root, '.github', 'scripts', 'tmp'),
  Benchmarks: path.join(__root, '.github', 'benchmarks'),
  CargoOutput: path.join(__root, 'target', '.cargo'),
  Output: path.join(__root, 'target'),
  Testing: path.join(__root, 'testing'),
  Fixtures: path.join(__root, 'testing', 'fixtures'),
})
