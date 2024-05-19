import { test, describe, before } from 'node:test'
import * as assert from 'node:assert'
import { BuildReport, Mach } from '@alshdavid/mach'
import { FIXTURES_PATH_FN } from '../utils/mach/index.js'
import { NodejsContext } from '../utils/nodejs/index.js'
import { install_npm } from '../utils/npm.js'
import { equal_unsafe } from '../utils/asset/index.js'

const FIXTURE_NAME = 'esm-basic'
const FIXTURE = FIXTURES_PATH_FN(FIXTURE_NAME)

const VALUES = {
  array_anon_default_typeof: 'array',
  array_anon_default_0: 'array_anon',
  array_anon_namespace_default_0: 'array_anon',
  array_named_typeof: 'array',
  array_named_0: 'array_named',
  array_named_namespace_array_named_0: 'array_named',
  class_anon_default_typeof: 'class_constructor',
  class_anon_default_new_data: 'function_anon_default',
  class_anon_namespace_default_new_data: 'function_anon_default',
  class_named_class_named_typeof: 'class_constructor',
  class_named_class_named_new_data: 'class_named',
  class_named_default_typeof: 'class_constructor',
  class_named_default_new_data: 'function_named_default',
  class_named_namespace_class_named_typeof: 'class_constructor',
  class_named_namespace_class_named_new_data: 'class_named',
  class_named_namespace_default_typeof: 'class_constructor',
  class_named_namespace_default_new_data: 'function_named_default',
  function_anon_arrow_default_typeof: 'function_arrow',
  function_anon_arrow_default_return: 'function_anon_arrow_default',
  function_anon_arrow_namespace_default_typeof: 'function_arrow',
  function_anon_arrow_namespace_default_return: 'function_anon_arrow_default',
  function_anon_default_typeof: 'function',
  function_anon_default_return: 'function_anon',
  function_anon_namespace_default_typeof: 'function',
  function_anon_namespace_default_return: 'function_anon',
  function_named_arrow_typeof: 'function_arrow',
  function_named_arrow_return: 'function_named_arrow',
  function_named_arrow_namespace_function_named_arrow_typeof: 'function_arrow',
  function_named_arrow_namespace_function_named_arrow_return: 'function_named_arrow',
  function_named_function_named_typeof: 'function',
  function_named_function_named_return: 'function_named',
  function_named_default_typeof: 'function',
  function_named_default_return: 'function_named_default',
  function_named_namespace_function_named_typeof: 'function',
  function_named_namespace_function_named_return: 'function_named',
  function_named_namespace_default_typeof: 'function',
  function_named_namespace_default_return: 'function_named_default',
  object_anon_typeof: 'object',
  object_anon_data: 'object_anon',
  object_anon_namespace_object_anon_data: 'object_anon',
  object_named_typeof: 'object',
  object_named_data: 'object_named',
  object_named_namespace_object_named_typeof: 'object',
  object_named_namespace_object_named_data: 'object_named',
  object_named_namespace_default_typeof: 'object',
  object_named_namespace_default_data: 'object_named'
}

describe(`${FIXTURE_NAME} (continue on failure)`, { concurrency: true }, () => {
  describe(`${FIXTURE_NAME} - Sanity`, { concurrency: true }, () => {
    test('Run in nodejs', async (t) => {
      await using nodejs = new NodejsContext({
        type: 'module',
        entry: FIXTURE('src', 'index.js'),
      })
  
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

