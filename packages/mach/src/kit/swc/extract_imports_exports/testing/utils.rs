use std::fs;
use std::path::Path;
use std::path::PathBuf;

use once_cell::sync::Lazy;

use super::super::super::parse_program;
use super::super::extract_imports_exports;
use crate::kit::swc::extract_imports_exports::AnalyzeFileResult;

pub static CARGO_DIR: Lazy<PathBuf> = Lazy::new(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")));
pub static FIXTURES: Lazy<PathBuf> = Lazy::new(|| {
  CARGO_DIR
    .join("src")
    .join("kit")
    .join("swc")
    .join("analyze_file")
    .join("testing")
    .join("__fixtures")
});
pub static SOURCES: Lazy<PathBuf> = Lazy::new(|| FIXTURES.join("sources"));
pub static SNAPSHOTS: Lazy<PathBuf> = Lazy::new(|| FIXTURES.join("snapshots"));

// #[test]
fn _test_update_snapshots() {
  for entry in std::fs::read_dir(&*SOURCES).unwrap() {
    let entry = entry.unwrap();
    let fixture_filepath = entry.path();
    _update_snapshot(&fixture_filepath);
  }
}

pub fn build_fixture(fixture_name: &str) -> AnalyzeFileResult {
  let fixture_filepath = SOURCES.join(fixture_name);
  test_analyze_file(&fixture_filepath)
}

fn test_analyze_file(fixture_filepath: &Path) -> AnalyzeFileResult {
  let fixture_name = get_fixture_name(&fixture_filepath);
  let contents = fs::read_to_string(&fixture_filepath).unwrap();
  let module = parse_program(&fixture_filepath, &contents, Default::default()).unwrap();
  let results = extract_imports_exports(&module.program);

  println!("{}", fixture_name);

  let snapshot = serde_json::from_str::<AnalyzeFileResult>(
    &fs::read_to_string(&SNAPSHOTS.join(format!("{}.json", &fixture_name))).unwrap(),
  )
  .unwrap();

  assert!(
    snapshot.len() == results.len(),
    "Error: {}\n\tMismatching module symbols\nFixture\n{}\nExpected:\n{:#?}Got:\n{:#?}",
    fixture_name,
    contents,
    snapshot,
    results
  );

  for result in results.iter() {
    assert!(
      snapshot.iter().find(|m| *m == result).is_some(),
      "Error: {}\n\tMismatching module entry\nFixture\n{}\nExpected:\n{:#?}Got:\n{:#?}",
      fixture_name,
      contents,
      snapshot,
      results
    );
  }

  results
}

pub fn get_fixture_name(fixture_filepath: &Path) -> String {
  fixture_filepath
    .file_name()
    .unwrap()
    .to_str()
    .unwrap()
    .to_string()
}

pub fn _update_snapshot(fixture_filepath: &Path) {
  let fixture_name = get_fixture_name(&fixture_filepath);
  let contents = fs::read_to_string(&fixture_filepath).unwrap();
  let module = parse_program(&fixture_filepath, &contents, Default::default()).unwrap();
  let result = extract_imports_exports(&module.program);

  let snapshot = serde_json::to_string_pretty(&result).unwrap();

  fs::write(&SNAPSHOTS.join(format!("{}.json", fixture_name)), snapshot).unwrap();
}
