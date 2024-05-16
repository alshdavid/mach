const OS_DEFAULT: string | undefined = ({
  linux: '/usr/bin/google-chrome-stable',
  darwin: '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',
  win32: ''
} as any)[process.platform]

export const CHROME_EXECUTABLE_PATH = process.env.CHROME_EXECUTABLE_PATH || OS_DEFAULT
