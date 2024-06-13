use super::snapshot::_generate_project_snapshot;
use super::utils::FIXTURES;

// Uncomment this and run it to update snapshots
// #[test]
fn _test_update_snapshots() {
  for dir in std::fs::read_dir(&*FIXTURES).unwrap() {
    let project_root = dir.unwrap().path();
    let project_name = project_root.file_stem().unwrap().to_str().unwrap();
    if project_name.starts_with("skip_") {
      continue;
    }
    println!("UPDATE: {}", project_name);
    _generate_project_snapshot(&project_root)
  }
}
