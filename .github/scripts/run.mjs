import { parse } from './platform/args.mjs'

try {
  const command = process.argv.splice(2, 1)
  const { main } = await import(`./cmd/${command[0]}.mjs`)
  const args = parse(process.argv.slice(2))
  await main(args.params)
} catch (err) {
  console.error(err)
  process.exit(1)
}