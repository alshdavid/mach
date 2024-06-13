use super::snapshot::generate_project_snapshot;
use super::utils::FIXTURES;

// Uncomment this and run it to update snapshots
// #[test]
fn _test_update_snapshots() {
  for dir in std::fs::read_dir(&*FIXTURES).unwrap() {
    let project_root = dir.unwrap().path();
    generate_project_snapshot(&project_root)
  }
}
