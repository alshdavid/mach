use super::{Dependency, DependencyMap};

pub fn get_dependency_for_specifier<'a>(
  dependency_map: &'a DependencyMap,
  specifier: &str,
) -> (&'a String, &'a Dependency) {
  let (dependency_id, dependency) = 'block: {
    for (dependency_id, dependency) in &dependency_map.dependencies {
      if dependency.specifier == *specifier {
        break 'block (dependency_id, dependency);
      }
    }
    panic!(
      "Could not find dependency for specifier\n  {}",
      specifier
    );
  };

  return (dependency_id, dependency);
}
