const a2 = 'value_a2'
const ident_a4 = 'a4_ident'

module.exports.a1 = 'value_a1'
module.exports.a2 = a2
module.exports.a2 = 'value_a2'
module.exports['a3'] = 'value_a3'
module.exports[ident_a4] = 'value_a4'

// TODO
// module.exports[a4_ident + '_1'] = 'value_a4.1'
// module.exports[`${a4_ident + '_2'}`] = 'value_a4.2'
