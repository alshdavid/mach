const { 
  a1, 
  a2, 
  a3, 
  a4_ident,
  a4_ident_1,
  a4_ident_2,
} = require('./a')

console.log({
  a1, 
  a2,
  a3,
  a4_ident,
  a4_ident_1,
  a4_ident_2,
})

const { 
  b1, 
  b2, 
  b3, 
  b4_ident,
  b4_ident_1,
} = require('./b')

console.log({
  b1, 
  b2,
  b3,
  b4_ident,
  b4_ident_1,
})

;(() => {
  const { 
    b1, 
    b2, 
    b3, 
    b4_ident,
    b4_ident_1,
  } = require('./b-nested')

  console.log({
    b1, 
    b2,
    b3,
    b4_ident,
    b4_ident_1,
  })
})()

console.log(require('./c'))