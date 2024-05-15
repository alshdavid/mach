const a2 = 'value_a2'
const ident_a4 = 'a4_ident'
function a5() {}

module.exports.a1 = 'value_a1'
module.exports.a2 = a2
module.exports.a2 = 'value_a2'
module.exports['a3'] = 'value_a3'
module.exports[ident_a4] = 'value_a4'
module.exports[ident_a4 + '_1'] = 'value_a4.1'
module.exports[`${ident_a4 + '_2'}`] = 'value_a4.2'
module.exports.a5 = a5
module.exports.a5()
