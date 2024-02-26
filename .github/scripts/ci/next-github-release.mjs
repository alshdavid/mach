const BRANCH_NAME = process.env.BRANCH_NAME
const URL_BASE = 'https://api.github.com/repos/alshdavid/mach'

async function get_release(tag) {
  const response = await fetch(`${URL_BASE}/releases/tags/${tag}`, options)
  if (response.status === 404) {
    return undefined
  }
  if (!response.ok) {
    throw new Error(await response.text())
  }
  return await response.json()
}

async function* get_releases() {
  let page = 1
  while (true) {
    const response = await fetch(`${URL_BASE}/releases?per_page=100&page=${page}`, options)
    if (!response.ok) {
      throw new Error(await response.text())
    }
    const results = await response.json()
    if (!results.length) {
      break
    }
    for (const result of results) {
      yield result
    }
    page += 1
  }
}

const tag_exists = !!(await get_release(`${BRANCH_NAME}-1`))
if (!tag_exists) {
  console.log(`${BRANCH_NAME}-1`)
  process.exit(0)
}

for await (const release of get_releases()) {
  if (release.tag_name.startsWith(BRANCH_NAME)) {
    const [,build] = release.tag_name.split('-')
    console.log(`${BRANCH_NAME}-${parseInt(build, 10) + 1}`)
    break
  }
}
