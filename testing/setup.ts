import '@shigen/polyfill-symbol-dispose'
import * as reporter from 'node:test/reporters';
import { run } from 'node:test';
import path from 'node:path';
import { new_browser } from './utils/browser';

void async function() {
  await using _browser = await new_browser();

  await new Promise(res => {
    run({ 
      files: [path.resolve('./tests/javascript.ts')],
    })
      .on('test:fail', () => {
        process.exitCode = 1;
      })
      .on('end', res)
      .compose(new reporter.spec())
      .pipe(process.stdout)
  })
}()
