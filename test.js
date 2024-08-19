let graph = `
  ROOT -> ./src/index.js
  ROOT -> ./src/index2.js
`

require("diagonjs").init().then(d => console.log(d.translate.graphDAG(graph)))