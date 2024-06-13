use super::utils::{generate_project_snapshot, FIXTURES};

// Uncomment this and run it to update snapshots
// #[test]
fn _test_update_snapshots() {
  for dir in std::fs::read_dir(&*FIXTURES).unwrap() {
    let project_root = dir.unwrap().path();
    generate_project_snapshot(&project_root)
  }
}