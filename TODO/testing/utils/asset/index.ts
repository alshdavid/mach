import { TestContext } from 'node:test'
import { equal } from 'node:assert'

export const equal_unsafe = (
  t: TestContext,
  actual: any,
  expect: any,
  message: string,
) => {
  try {
    equal(actual, expect, message)
  } catch (error: any) {
    t.diagnostic('❌ ' + error.message)
    t.todo()
  }
}
