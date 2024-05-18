import { test, describe, before } from 'node:test'
import * as assert from 'node:assert'
import { BuildReport, Mach } from '@alshdavid/mach'
import { FIXTURES_FN } from '../utils/mach/index.js'
import { NodejsContext } from '../utils/nodejs/index.js'
import { install_npm } from '../utils/npm.js'

const FIXTURE = FIXTURES_FN('plugins-nodejs-resolver-noop')

describe('plugins-nodejs-resolver-noop', { concurrency: true }, () => {
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

  test('Should set exports correctly ', async (t) => {
    await using nodejs = new NodejsContext({
      type: 'commonjs',
      entry: FIXTURE('dist', report.entries['src/index.js']),
    })

    const result = await nodejs.get_global('foo')
    assert.equal(
      result,
      'foo',
      `Expect global key "Foo" to be "foo", got "${result}"`,
    )
  })
})
