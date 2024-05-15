import {test, describe} from 'node:test';
import assert from 'node:assert';
import { create_page } from '../utils/browser';
import { FIXTURES, build_mach } from '../utils/mach';

describe('javascript', { concurrency: true }, () => {
  test('synchronous passing test', async (t) => {
    await using page = await create_page()
    
    const result = await build_mach({
      cwd: FIXTURES('js-commonjs'),
      entries: ['src/index.js']
    })

    assert.equal(Object.keys(result.assets).length, 2)
  });
})