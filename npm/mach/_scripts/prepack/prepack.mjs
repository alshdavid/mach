import * as fs from "node:fs";
import * as path from "node:path";
import * as http from "node:https";
import * as url from "node:url";
import * as child_process from "node:child_process";

const __dirname = url.fileURLToPath(new URL(".", import.meta.url));

const NPM_VERSION = fs.existsSync(path.join(__dirname, 'npm_version.txt')) 
  ? fs.readFileSync(path.join(__dirname, 'npm_version.txt'), 'utf8').trim() 
  : ''

if (NPM_VERSION == '') {
  console.log('skipping pack')
  process.exit(1) 
}

const packageJson = JSON.parse(fs.readFileSync(path.join(__dirname, '..', '..', 'package.json'), 'utf8'))

packageJson.version = NPM_VERSION

fs.writeFileSync(path.join(__dirname, '..', '..', 'package.json'), JSON.stringify(packageJson, null, 2), 'utf8')
