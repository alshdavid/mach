import { test, describe, before } from 'node:test'
import * as assert from 'node:assert'
import { BuildReport, Mach } from '@alshdavid/mach'
import { FIXTURES_PATH_FN } from '../utils/mach/index.js'
import { NodejsContext } from '../utils/nodejs/index.js'
import { install_npm } from '../utils/npm.js'
import { equal_unsafe } from '../utils/asset/index.js'

const VALUES = {
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

const FIXTURE_NAME = 'commonjs-basic'
const FIXTURE = FIXTURES_PATH_FN(FIXTURE_NAME)

describe.skip(`${FIXTURE_NAME} (continue on failure)`, { concurrency: true }, () => {
  describe(`${FIXTURE_NAME} - Sanity`, { concurrency: true }, () => {
    test('Run in nodejs', async (t) => {
      await using nodejs = new NodejsContext({
        type: 'module',
        entry: FIXTURE('src', 'index.js'),
      })

      await nodejs.get_global('onready')

      for (const [key, expect] of Object.entries(VALUES)) {
        const result = await nodejs.get_global('output', key)
        assert.equal(
          result,
          expect,
          `Expect global key "${key}" to be "${expect}", got "${result}"`,
        )
      }
    })
  })

  describe(`${FIXTURE_NAME}`, { concurrency: true }, () => {
    let report: BuildReport

    before(async () => {
      install_npm(FIXTURE())

      report = await Mach.build({
        projectRoot: FIXTURE(),
        entries: ['src/index.js'],
        outFolder: 'dist',
        clean: true,
      })
    })

    for (const [key, expect] of Object.entries(VALUES)) {
      test(key, async (t) => {
        await using nodejs = new NodejsContext({
          type: 'commonjs',
          entry: FIXTURE('dist', report.entries['src/index.js']),
        })

        await nodejs.get_global('onready')

        const result = await nodejs.get_global('output', key)
        equal_unsafe(
          t,
          result,
          expect,
          `Expect "${key}" to be "${expect}", got "${result}"`,
        )
      })
    }
  })
})
