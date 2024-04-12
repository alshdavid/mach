export function resolver_resolve(
  /** @type {unknown} */ data,
  /** @type {import('../../types/response.ts').Response} */ res,
) {
  console.log(data)
  res()
}
