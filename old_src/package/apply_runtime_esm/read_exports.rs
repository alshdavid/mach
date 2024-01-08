use swc_core::ecma::ast::*;

/*
    export const foo = ''
    export function foo() {}
    export class foo {}
*/
pub fn read_exports(export_decl: &ExportDecl) -> Vec<String> {
  let mut exports = Vec::<String>::new();

  match &export_decl.decl {
    Decl::Var(decl) => {
      for decl in decl.decls.iter() {
        if let Pat::Ident(name) = &decl.name {
          exports.push(name.id.sym.to_string());
        }
      }
    }
    Decl::Fn(decl) => {
      exports.push(decl.ident.sym.to_string());
    }
    Decl::Class(decl) => {
      exports.push(decl.ident.sym.to_string());
    }
    _ => {
      todo!()
    }
  }

  return exports;
}
