import '@shigen/polyfill-symbol-dispose'
import * as reporter from 'node:test/reporters';
import { run } from 'node:test';
import * as path from 'node:path';
import * as fs from 'node:fs';
import { new_browser } from './utils/browser/index.js';
import * as url from 'node:url'
import fsAsync from 'node:fs/promises';

const __dirname = url.fileURLToPath(new URL('.', import.meta.url))
void async function() {
  await using _browser = await new_browser();

  const files = fs.readdirSync(path.join(__dirname, 'tests'))
    .filter(test => !test.startsWith('_'))
    .map(test => path.join(__dirname, 'tests', test))

  await new Promise(res => {
    run({ 
      files,
      concurrency: true,
    })
      .on('test:fail', () => {
        process.exitCode = 1;
      })
      .on('end', res)
      .compose(new reporter.spec())
      .pipe(process.stdout)
  })
}()
