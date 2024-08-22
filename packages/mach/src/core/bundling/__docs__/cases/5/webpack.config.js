import * as fs from 'node:fs';
import * as path from 'node:path';
import * as url from 'node:url';

const __dirname = url.fileURLToPath(new URL('.', import.meta.url));

const entry = {}
const sources = JSON.parse(fs.readFileSync(path.join(__dirname, 'package.json'), 'utf8')).targets.default.source
for (const source of sources) {
  let parsed = path.parse(source)
  // @ts-expect-error
  entry[parsed.name] = path.join(__dirname, source)
}

export default [
  {
    mode: 'production',
    entry,
    output: {
      path: path.join(__dirname, 'dist-webpack')
    },
    devtool: false,
    optimization: {
      minimize: false
    },
  },
];
