use swc_core::ecma::ast::*;

pub fn module_item_to_stmt(input: Vec<ModuleItem>) -> Vec<Stmt> {
  input
    .into_iter()
    .filter_map(|x| -> Option<Stmt> {
      let ModuleItem::Stmt(stmt) = x else {
        return None;
      };
      return Some(stmt);
    }).collect::<Vec<Stmt>>()
}

pub fn stmt_to_module_item(input: Vec<Stmt>) -> Vec<ModuleItem> {
  input
    .into_iter()
    .map(|x| ModuleItem::Stmt(x))
    .collect::<Vec<ModuleItem>>()
}
