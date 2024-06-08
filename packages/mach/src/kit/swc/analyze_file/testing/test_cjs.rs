use super::utils::build_fixture;

#[test]
fn export_default_commonjs_cjs() {
  build_fixture("export_default_commonjs.cjs");
}

#[test]
fn export_default_commonjs_bad_cjs() {
  build_fixture("export_default_commonjs_bad.cjs");
}

#[test]
fn export_named_commonjs_cjs() {
  build_fixture("export_named_commonjs.cjs");
}

#[test]
fn export_named_commonjs_2_cjs() {
  build_fixture("export_named_commonjs_2.cjs");
}

#[test]
fn export_named_commonjs_3_cjs() {
  build_fixture("export_named_commonjs_3.cjs");
}

#[test]
fn export_named_commonjs_bad_cjs() {
  build_fixture("export_named_commonjs_bad.cjs");
}

#[test]
fn export_named_commonjs_bad_2_cjs() {
  build_fixture("export_named_commonjs_bad_2.cjs");
}

#[test]
fn export_named_commonjs_bad_3_cjs() {
  build_fixture("export_named_commonjs_bad_3.cjs");
}

#[test]
fn import_require_cjs() {
  build_fixture("import_require.cjs");
}

#[test]
fn import_require_assignment_cjs() {
  build_fixture("import_require_assignment.cjs");
}

#[test]
fn import_require_destructured_cjs() {
  build_fixture("import_require_destructured.cjs");
}
