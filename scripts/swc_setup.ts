// Run this to update SWC to the latest
import { resolve, dirname, fromFileUrl, join } from "https://deno.land/std@0.219.0/path/mod.ts"
import { copy } from "https://deno.land/std@0.219.0/fs/mod.ts"
import { parse as toml_parse, stringify as toml_stringify } from "https://deno.land/std@0.219.0/toml/mod.ts";
import { parse as semver_parse, SemVer, greaterThan, format as semver_format } from "https://deno.land/std@0.219.0/semver/mod.ts"
import { decompress } from "https://deno.land/x/zip@v1.2.5/mod.ts";

const __dirname = resolve(dirname(fromFileUrl(import.meta.url)))
const ROOT = (...segs: string[]) => join(__dirname, '..', ...segs)

type CargoToml = {
  package: {
    name: string
    version: string
  },
  dependencies?: Record<string, string | { version?: string, path?: string, package?: string }>
  'dev-dependencies'?: Record<string, string | { version?: string, path?: string, package?: string }>
}

const required_deps = await find_swc_deps(ROOT('mach', 'Cargo.toml'))

//
// Download SWC as a zip file
//
console.log('Downloading latest "main"')
const swc_zip = await fetch('https://github.com/swc-project/swc/archive/refs/heads/main.zip')
if (!swc_zip || !swc_zip.body) {
  throw new Error('Failed to download zip')
}

const tmp = await Deno.makeTempDir({ prefix: 'mach_' })

const file = await Deno.open(join(tmp, 'swc.zip'), { create: true, write: true })
await swc_zip.body.pipeTo(file.writable);

try {
  try { file.close() } catch (_) {}

  console.log('Unpacking "main"')

  await Deno.mkdir(ROOT('vendor'), { recursive: true })

  try {
    await Deno.remove(ROOT('vendor', 'swc'), { recursive: true })
  } catch (_) {}

  await decompress(join(tmp, 'swc.zip'), join(tmp))

  await update_swc_crates(join(tmp, 'swc-main'))

  await copy(join(tmp, 'swc-main', 'crates'), ROOT('vendor', 'swc'))
  await Deno.remove(tmp, { recursive: true })
} catch (e) {
  await Deno.remove(tmp, { recursive: true })
  throw e
}

async function update_swc_crates(root: string) {
  const ROOT = (...segs: string[]) => join(root, ...segs)

  const packages = new Map<string, [SemVer, CargoToml]>()
  const package_names = new Map<string, string>()
  
  for await (const dirname of Deno.readDir(ROOT('crates'))) {
    const toml_path = ROOT('crates', dirname.name, 'Cargo.toml')
    const toml = toml_parse(await Deno.readTextFile(toml_path)) as CargoToml
    const version = semver_parse(toml.package.version)
    packages.set(toml_path, [version, toml])
  }

  const required_scan: string[] = Array.from(required_deps)

  while (required_scan.length) {
    const to_check = required_scan.pop()
    if (!to_check) break
    const toml_path = ROOT('crates', to_check, 'Cargo.toml')
    const new_deps = await find_swc_deps(toml_path)
    for (const [,new_dep] of new_deps.entries()) {
      if (required_deps.has(new_dep)) {
        continue
      }
      required_deps.add(new_dep)
      required_scan.push(new_dep)
    }
  }

  let biggest_version: SemVer = semver_parse('0.0.0')
  
  for (const [version] of packages.values()) {
    if (!greaterThan(version, biggest_version)) continue
    biggest_version = version
  }
  
  const bump_version_to = semver_format(semver_parse(`${biggest_version.major + 1}.0.0`))
  
  console.log(`Bumping packages to: ${bump_version_to}`)
  
  for (const [filepath, [,toml]] of packages.entries()) {
    console.log(`  Package ${toml.package.name}`)
  
    const original_name = toml.package.name
    console.log(`    rename ${original_name} to ad_${toml.package.name}`)
    toml.package.name = `ad_${toml.package.name}`
  
    package_names.set(original_name, toml.package.name)
  
    console.log(`    version ${toml.package.version} to ${bump_version_to}`)
    toml.package.version = bump_version_to
  
    for (const [key, details] of Object.entries(toml.dependencies || {})) {
      if (typeof details === 'string') continue
      if (!details.path) continue
  
      if (!details.package) {
        console.log(`      dep ${details.package}@${details.version} to ad_${key}@${bump_version_to}`)
        details.package = `ad_${key}`
        details.version = bump_version_to
      } else if (!details.package?.startsWith('ad_')) {
        console.log(`      dep ${details.package}@${details.version} to ad_${key}@${bump_version_to}`)
        details.package = `ad_${key}`
        details.version = bump_version_to
      } else {
        console.log(`      dep ${details.package}@${details.version} to ad_${key}@${bump_version_to}`)
        details.version = bump_version_to
      }
    }
  
    for (const [key, details] of Object.entries(toml["dev-dependencies"] || {})) {
      if (typeof details === 'string') continue
      if (!details.path) continue
  
      if (!details.package) {
        console.log(`      dep ${details.package}@${details.version} to ad_${key}@${bump_version_to}`)
        details.package = `ad_${key}`
        details.version = bump_version_to
      } else if (!details.package?.startsWith('ad_')) {
        console.log(`      dep ${details.package}@${details.version} to ad_${key}@${bump_version_to}`)
        details.package = `ad_${key}`
        details.version = bump_version_to
      } else {
        console.log(`      dep ${details.package}@${details.version} to ad_${key}@${bump_version_to}`)
        details.version = bump_version_to
      }
    }
  
    console.log()
  
    await Deno.writeTextFile(filepath, toml_stringify(toml))
  }
  
  
  console.log(`Removing Tests:`)
  
  const rewrite_targets = ["tests"]
  
  for await (const dirname of Deno.readDir(ROOT('crates'))) {
    for await (const rewrite_target of rewrite_targets) {
      try {
        await Deno.remove(ROOT('crates', dirname.name, rewrite_target), { recursive: true })
        console.log('Removed', ROOT('crates', dirname.name, rewrite_target))
      } catch (error) {}
    }
  }


  for await (const dirname of Deno.readDir(ROOT('crates'))) {
    if (!required_deps.has(dirname.name)) {
      await Deno.remove(ROOT('crates', dirname.name), { recursive: true })
    }
  }
  
  console.log()
  console.log(`Packages updated: ${packages.size}`)
}

async function find_swc_deps(src: string): Promise<Set<string>> {
  const mach_cargo = toml_parse(await Deno.readTextFile(src)) as CargoToml
  const required_deps = new Set<string>()
  
  for (const [key, value] of Object.entries(mach_cargo.dependencies || {})) {
    if (key.startsWith('ad_swc')) {
      required_deps.add(key.replace('ad_swc_', 'swc_'))
      continue
    } 
    if (key.startsWith('swc_')) {
      required_deps.add(key)
      continue
    }
    if (typeof value !== 'string' && value.path) {
      required_deps.add(key)
      continue
    }
  };

  for (const [key, value] of Object.entries(mach_cargo["dev-dependencies"] || {})) {
    if (key.startsWith('ad_swc')) {
      required_deps.add(key.replace('ad_swc_', 'swc_'))
      continue
    } 
    if (key.startsWith('swc_')) {
      required_deps.add(key)
      continue
    }
    if (typeof value !== 'string' && value.path) {
      required_deps.add(key)
      continue
    }
  };

  return required_deps
}