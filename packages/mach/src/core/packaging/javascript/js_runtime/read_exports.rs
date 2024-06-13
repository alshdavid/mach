use swc_core::ecma::ast::*;

/*
    export const foo = ''
    export function foo() {}
    export class foo {}
*/
pub fn read_exports(export_decl: ExportDecl) -> Vec<String> {
  let mut exports = Vec::<String>::new();

  match export_decl.decl {
    Decl::Var(decl) => {
      for decl in decl.decls.into_iter() {
        let Pat::Ident(name) = decl.name.clone() else {
          continue;
        };
        exports.push(name.id.sym.to_string());
      }
    }
    Decl::Fn(decl) => {
      exports.push(decl.ident.sym.to_string());
    }
    Decl::Class(decl) => {
      exports.push(decl.ident.sym.to_string());
    }
    Decl::Using(_) => panic!("Decl::Using"),
    Decl::TsInterface(_) => panic!("Decl::TsInterface"),
    Decl::TsTypeAlias(_) => panic!("Decl::TsTypeAlias"),
    Decl::TsEnum(_) => panic!("Decl::TsEnum"),
    Decl::TsModule(_) => panic!("Decl::TsModule"),
  }

  return exports;
}
