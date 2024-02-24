require('fs').writeFileSync('./foo', '#!/bin/bash\necho hello', 'utf8')

// const packageJson = require('./package.json')
// if (packageJson.version === '0.0.0' || process.env.MACH_SKIP_INSTALL === 'true') {
//   console.log('skip download')
//   process.exit(0) 
// }

// console.log('download')