use super::utils::build_fixture;

#[test]
fn combination_mjs() {
  build_fixture("combination.mjs");
}

#[test]
fn export_default_mjs() {
  build_fixture("export_default.mjs");
}

#[test]
fn export_default_class_anon_mjs() {
  build_fixture("export_default_class_anon.mjs");
}

#[test]
fn export_default_class_named_mjs() {
  build_fixture("export_default_class_named.mjs");
}

#[test]
fn export_default_func_anon_mjs() {
  build_fixture("export_default_func_anon.mjs");
}

#[test]
fn export_default_func_named_mjs() {
  build_fixture("export_default_func_named.mjs");
}

#[test]
fn export_default_var_mjs() {
  build_fixture("export_default_var.mjs");
}

#[test]
fn export_destructured_array_mjs() {
  build_fixture("export_destructured_array.mjs");
}

#[test]
fn export_destructured_array_2_mjs() {
  build_fixture("export_destructured_array_2.mjs");
}

#[test]
fn export_destructured_obj_mjs() {
  build_fixture("export_destructured_obj.mjs");
}

#[test]
fn export_destructured_obj_computed_mjs() {
  build_fixture("export_destructured_obj_computed.mjs");
}

#[test]
fn export_destructured_obj_computed_2_mjs() {
  build_fixture("export_destructured_obj_computed_2.mjs");
}

#[test]
fn export_destructured_obj_renamed_mjs() {
  build_fixture("export_destructured_obj_renamed.mjs");
}

#[test]
fn export_named_mjs() {
  build_fixture("export_named.mjs");
}

#[test]
fn export_named_class_mjs() {
  build_fixture("export_named_class.mjs");
}

#[test]
fn export_named_func_mjs() {
  build_fixture("export_named_func.mjs");
}

#[test]
fn export_named_var_mjs() {
  build_fixture("export_named_var.mjs");
}

#[test]
fn export_renamed_mjs() {
  build_fixture("export_renamed.mjs");
}

#[test]
fn import_default_mjs() {
  build_fixture("import_default.mjs");
}

#[test]
fn import_direct_mjs() {
  build_fixture("import_direct.mjs");
}

#[test]
fn import_dynamic_mjs() {
  build_fixture("import_dynamic.mjs");
}

#[test]
fn import_dynamic_2_mjs() {
  build_fixture("import_dynamic_2.mjs");
}

#[test]
fn import_dynamic_assignment_mjs() {
  build_fixture("import_dynamic_assignment.mjs");
}

#[test]
fn import_dynamic_destructured_mjs() {
  build_fixture("import_dynamic_destructured.mjs");
}

#[test]
fn import_dynamic_destructured_2_mjs() {
  build_fixture("import_dynamic_destructured_2.mjs");
}

#[test]
fn import_named_mjs() {
  build_fixture("import_named.mjs");
}

#[test]
fn import_named_multiple_mjs() {
  build_fixture("import_named_multiple.mjs");
}

#[test]
fn import_named_renamed_mjs() {
  build_fixture("import_named_renamed.mjs");
}

#[test]
fn import_namespace_mjs() {
  build_fixture("import_namespace.mjs");
}

#[test]
fn import_renamed_mjs() {
  build_fixture("import_renamed.mjs");
}

#[test]
fn import_renamed_multiple_mjs() {
  build_fixture("import_renamed_multiple.mjs");
}

#[test]
fn reexport_all_mjs() {
  build_fixture("reexport_all.mjs");
}

#[test]
fn reexport_named_mjs() {
  build_fixture("reexport_named.mjs");
}

#[test]
fn reexport_namespace_mjs() {
  build_fixture("reexport_namespace.mjs");
}

#[test]
fn reexport_renamed_mjs() {
  build_fixture("reexport_renamed.mjs");
}
