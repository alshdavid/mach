import * as semver from 'semver'

const NPM_TAG = process.env.NPM_TAG

const response = await fetch(`https://registry.npmjs.org/@alshdavid/mach/${NPM_TAG}`)
if (response.ok === false) {
  if (NPM_TAG === 'latest') {
    console.log(`0.0.1`)
  } else {
    console.log(`0.0.1-${NPM_TAG}`)
  }
  process.exit(0)
}

const details = await response.json()
const ver = semver.parse(details.version)
ver.patch += 1
console.log(ver.format())
