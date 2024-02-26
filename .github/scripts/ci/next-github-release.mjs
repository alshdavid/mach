export async function* get_gh_releases(GH_API_URL) {
  let page = 1
  while (true) {
    const response = await fetch(`${GH_API_URL}/releases?per_page=100&page=${page}`)
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
