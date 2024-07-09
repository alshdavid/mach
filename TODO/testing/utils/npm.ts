import path from 'node:path'
import child_process from 'node:child_process'
import fs from 'node:fs'

export async function install_npm(cwd: string): Promise<void> {
  if (!fs.existsSync(cwd)) {
    throw new Error(`Error Does Not Exist: ${cwd}`)
  }

  if (!fs.existsSync(path.join(cwd, 'node_modules'))) {
    await new Promise((resolve, reject) => {
      child_process.exec(
        `npm install --no-package-lock`,
        { cwd },
        (err, stdout) => {
          if (err) return reject(err)
          resolve(stdout)
        },
      )
    })
  }
}
