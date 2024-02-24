import * as fs from "node:fs";
import * as path from "node:path";
import * as url from "node:url";

const __dirname = url.fileURLToPath(new URL(".", import.meta.url));
const { NPM_VERSION, NPM_BIN_TARGET } = process.env

if (!NPM_VERSION || !NPM_BIN_TARGET) {
  console.log('skipping pack')
  process.exit(1) 
}

const packageJson = JSON.parse(fs.readFileSync(path.join(__dirname, '..', '..', 'package.json'), 'utf8'))

packageJson.version = process.env.NPM_VERSION
packageJson.mach = { bin: process.env.NPM_BIN_TARGET }

fs.writeFileSync(path.join(__dirname, '..', '..', 'package.json'), JSON.stringify(packageJson, null, 2), 'utf8')
