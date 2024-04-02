const fs = require('node:fs')
const path = require('node:path')
const child_process = require('node:child_process')

if (process.env.npm_config_user_agent.startsWith('npm/')) {
  child_process.execSync(`npm install rxjs`, { cwd: __dirname, stdio: 'ignore' })
}
else if (process.env.npm_config_user_agent.startsWith('pnpm/')) {
  child_process.execSync(`pnpm install rxjs`, { cwd: __dirname, stdio: 'ignore' })
}
else if (process.env.npm_config_user_agent.startsWith('yarn/')) {
  child_process.execSync(`yarn add rxjs`, { cwd: __dirname, stdio: 'ignore' })
} else {
  child_process.execSync(`npm install rxjs`, { cwd: __dirname, stdio: 'ignore' })
}
