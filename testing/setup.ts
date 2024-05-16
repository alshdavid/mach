import '@shigen/polyfill-symbol-dispose'
import * as reporter from 'node:test/reporters';
import { run } from 'node:test';
import * as path from 'node:path';
import * as fs from 'node:fs';
import * as url from 'node:url';
import * as puppeteer from 'puppeteer-core';
import { finished } from 'node:stream';
import { CHROME_EXECUTABLE_PATH } from './utils/browser/executable.js';

const __dirname = url.fileURLToPath(new URL('.', import.meta.url))

void async function() {
  const files = fs.readdirSync(path.join(__dirname, 'tests'))
    .filter(test => !test.startsWith('_'))
    .map(test => path.join(__dirname, 'tests', test))
  
  const test_stream = run({ 
    files,
    concurrency: true,
  })
    .on('test:fail', () => {
      process.exitCode = 1;
    })
    .compose(new reporter.spec())

  test_stream.pipe(process.stdout)
  await new Promise(res => finished(test_stream, res))
}()
