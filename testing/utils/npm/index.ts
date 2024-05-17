import path from 'node:path';
import child_process from 'node:child_process';
import fs from 'node:fs';

export function install_npm(cwd: string): void {
  if (!fs.existsSync(cwd)) {
    throw new Error(`Error Does Not Exist: ${cwd}`)
  }

  if (!fs.existsSync(path.join(cwd, 'node_modules'))) {
    child_process.execSync(`npm install --no-package-lock`, { cwd })
  }
}
