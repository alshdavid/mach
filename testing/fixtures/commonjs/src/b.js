// Not actually valid CJS, but this is used by React

const b2 = 'value_b2'
const ident_b4 = 'b4_ident'

exports.b1 = 'value_b1'
exports.b2 = b2
exports.b2 = 'value_b2'
exports['b3'] = 'value_b3'
exports[ident_b4] = 'value_b4'

// TODO
// exports[a4_ident + '_1'] = 'value_a4.1'
// exports[`${a4_ident + '_2'}`] = 'value_a4.2'
