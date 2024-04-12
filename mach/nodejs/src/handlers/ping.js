export function ping(
  /** @type {unknown} */ data,
  /** @type {import('../../types/response.ts').Response} */ res,
) {
  res("pong")
}
