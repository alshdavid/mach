import {test, describe} from 'node:test';
import * as assert from 'node:assert';
import { Mach } from '@alshdavid/mach';
import { FIXTURES } from '../utils/mach/index.js';
import { NodejsContext } from '../utils/nodejs/index.js';

describe('javascript', { concurrency: true }, () => {
  test('synchronous passing test', async (t) => {    
    const report = await Mach.build({
      projectRoot: FIXTURES('js-commonjs'),
      entries: ['src/index.js']
    })

    await using nodejs = new NodejsContext({ type: 'commonjs' })
    
    await nodejs.import(FIXTURES('js-commonjs', report.output['src/index.js']))
    await nodejs.get_global('onready')

    const values = {
      a1: 'value_a1',
      a2: 'value_a2',
      a3: 'value_a3',
      a4_ident: 'value_a4',
      a4_ident_1: 'value_a4.1',
      a4_ident_2: 'value_a4.2',
      a5: 'function',
      b1: 'value_b1',
      b2: 'value_b2',
      b3: 'value_b3',
      b4_ident: 'value_b4',
      b4_ident_1: undefined,
      b5: 'function',
      nested_b1: 'value_b1',
      nested_b2: 'value_b2',
      nested_b3: 'value_b3',
      nested_b4_ident: 'value_b4',
      nested_b4_ident_1: undefined,
      nested_b5: 'function',
      nested_b6: 'function',
      c1: 'c1',
    }

    for (const [key, expect] of Object.entries(values)) {
      const result = await nodejs.get_global(key)
      assert.equal(
        result, 
        expect,
        `Expect global key "${key}" to be "${expect}", got "${result}"`
      )
    }
  });
})
