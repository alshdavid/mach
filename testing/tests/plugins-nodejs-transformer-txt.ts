import { test, describe, before } from 'node:test'
import * as assert from 'node:assert'
import { BuildReport, Mach } from '@alshdavid/mach'
import { FIXTURES_PATH_FN } from '../utils/mach/index.js'
import { NodejsContext } from '../utils/nodejs/index.js'
import { install_npm } from '../utils/npm.js'
import { equal_unsafe } from '../utils/asset/index.js'

const FIXTURE_NAME = 'plugins-nodejs-transformer-txt'
const FIXTURE = FIXTURES_PATH_FN(FIXTURE_NAME)

describe.skip(`${FIXTURE_NAME} (continue on failure)`, { concurrency: true }, () => {
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

    let result: string | undefined = undefined
    try {
      result = (await nodejs.get_global('content_key')) as any
    } catch (error) {}
    equal_unsafe(
      t,
      result,
      'content',
      `Expect "Foo" to be "foo", got "${result}"`,
    )
  })
})
