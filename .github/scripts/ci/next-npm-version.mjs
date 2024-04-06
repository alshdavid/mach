import * as semver from 'semver'

const NPM_TAG = process.env.NPM_TAG

const PACKAGES = [
  "@alshdavid/mach",
  "@alshdavid/mach-linux-amd64",
  "@alshdavid/mach-linux-arm64",
  "@alshdavid/mach-macos-amd64",
  "@alshdavid/mach-macos-arm64",
  "@alshdavid/mach-windows-amd64",
  "@alshdavid/mach-windows-arm64",
]

async function get_version_for_tag(package_name, tag) {
  const response = await fetch(`https://registry.npmjs.org/${package_name}/${tag}`)
  if (response.ok === false) {
    if (tag === 'latest') {
      return semver.parse(`0.0.1`)
    } else {
      return semver.parse(`0.0.1-${tag}`)
    }
  }

  const details = await response.json()
  return semver.parse(details.version)
}

let highest_version = undefined

for (const package_name of PACKAGES) {
  const ver = await get_version_for_tag(package_name, NPM_TAG)
  if (!ver) {
    console.error("unable to find version for:", package_name)
    process.exit(1)
  }
  if (!highest_version) {
    highest_version = ver
    continue
  }
  if (ver.compare(highest_version) === 1) {
    highest_version = ver
  }
}

highest_version.patch += 1
console.log(highest_version.format())
