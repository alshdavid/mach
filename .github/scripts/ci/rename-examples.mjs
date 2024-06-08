import * as path from 'node:path';
import * as fs from 'node:fs';
import { Paths } from '../platform/paths.mjs'

// for (const example_name of fs.readdirSync(Paths.Examples)) {
//   console.log(example_name)

//   const json_path = path.join(Paths.Examples, example_name, 'package.json')
//   const json = JSON.parse(fs.readFileSync(json_path))
//   json.name = `@workspace/${example_name}`

//   json.scripts = json.scripts || {}
//   json.scripts['serve'] = 'npx http-server -c=-1 -p=3000 .'
//   json.scripts['build'] = 'npx mach build'
//   json.scripts = sort_object(json.scripts)

//   json.devDependencies = json.devDependencies || {}
//   json.devDependencies['http-server'] = "*"
//   json.devDependencies['@alshdavid/mach'] = "workspace:*"
//   json.devDependencies = sort_object(json.devDependencies)
  
//   fs.writeFileSync(json_path, JSON.stringify(json, null, 2))
// }

for (const example_name of fs.readdirSync(Paths.TestingFixtures)) {
  // console.log(example_name)

  const json_path = path.join(Paths.TestingFixtures, example_name, 'package.json')
  let json = {}
  try {
    json = JSON.parse(fs.readFileSync(json_path) || '{}')
  } catch (err) {

  }

  // fs.mkdirSync(path.join(Paths.TestingFixtures, example_name, 'src'), { recursive: true })
  // if (fs.existsSync(path.join(Paths.TestingFixtures, example_name, 'src'))) {
  //   continue
  // }
  // console.log(example_name)
  // fs.mkdirSync(path.join(Paths.TestingFixtures, example_name, 'src'), { recursive: true })
  // for (const file_name of fs.readdirSync(path.join(Paths.TestingFixtures, example_name))) {
  //   if (file_name.startsWith('.')) continue
  //   if (file_name.startsWith('src')) continue
  //   if (file_name.startsWith('package.json')) continue
  //   if (file_name.startsWith('tsconfig.json')) continue
  //   fs.renameSync(
  //     path.join(Paths.TestingFixtures, example_name, file_name),
  //     path.join(Paths.TestingFixtures, example_name, 'src', file_name),
  //   )
  // }

  json.name = `@workspace/integration-test-${example_name}`

  json.scripts = json.scripts || {}
  json.scripts['serve'] = 'npx http-server -c=-1 -p=3000 .'
  json.scripts['build'] = 'npx mach build'
  json.scripts = sort_object(json.scripts)

  json.devDependencies = json.devDependencies || {}
  delete json.devDependencies['http-server']
  json.devDependencies['@alshdavid/mach'] = "../../../npm/mach"
  json.devDependencies = sort_object(json.devDependencies)
  
  fs.writeFileSync(json_path, JSON.stringify(json, null, 2))
}

function sort_object(obj) {
  const sorted_keys = Object.keys(obj).sort();

  const sorted_obj = {};

  for (const key of sorted_keys) {
    sorted_obj[key] = obj[key];
    if (Array.isArray(obj[key])) {
      sorted_obj[key] = obj[key].sort()
      continue
    }
    if (typeof obj[key] === "object") {
      sort_object(obj[key])
      continue
    }
  }

  return sorted_obj
}
