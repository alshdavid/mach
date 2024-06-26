/*
  TODO rewrite this
*/
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;

use normalize_path::NormalizePath;

use super::BuildOptions;
use crate::public::MachConfig;
use crate::public::MachConfigSync;
use crate::public::Machrc;

pub fn parse_config(command: &BuildOptions) -> Result<MachConfigSync, String> {
  let start_time = SystemTime::now();

  // Auto detect project root
  let project_root = 'block: {
    if let Some(project_root) = &command.project_root {
      break 'block get_absolute_path(None, &project_root);
    };
    env::current_dir().unwrap()
  };

  return Ok(Arc::new(MachConfig {
    start_time,
    entries: command.entries.clone(),
    dist_dir: get_dist_dir(&command, &project_root),
    clean_dist_dir: command.clean,
    bundle_splitting: command.bundle_splitting,
    project_root: project_root.clone(),
    machrc: parse_machrc(project_root.join(".machrc"))?,
    // threads: get_threads(&command),
    optimize: command.optimize,
    env: get_env(),
  }));
}

fn get_dist_dir(
  options: &BuildOptions,
  project_root: &Path,
) -> PathBuf {
  let dist_dir_arg = options.out_folder.clone();
  if dist_dir_arg.is_absolute() {
    dist_dir_arg
  } else {
    project_root.join(dist_dir_arg).normalize()
  }
}

fn get_env() -> HashMap<String, String> {
  let mut vars = HashMap::<String, String>::new();
  for (k, v) in std::env::vars() {
    vars.insert(k, v);
  }
  vars
}

// fn find_entry(project_root: &Path) -> Option<PathBuf> {
//   for test in [
//     project_root.join("index.html"),
//     project_root.join("index.tsx"),
//     project_root.join("index.ts"),
//     project_root.join("index.jsx"),
//     project_root.join("index.js"),
//     project_root.join("src").join("index.html"),
//     project_root.join("src").join("index.tsx"),
//     project_root.join("src").join("index.ts"),
//     project_root.join("src").join("index.jsx"),
//     project_root.join("src").join("index.js"),
//   ] {
//     if test.exists() {
//       return Some(test);
//     }
//   }

//   return None;
// }

fn parse_machrc(file_path: PathBuf) -> Result<Machrc, String> {
  if !file_path.exists() {
    return Ok(Machrc::default());
  };

  let mut mach_config = Machrc {
    is_default: false,
    file_path,
    engines: vec!["mach".to_string()],
    resolvers: None,
    transformers: None,
  };

  let Ok(json_file) = fs::read_to_string(&mach_config.file_path) else {
    return Err("Unable to read file".to_string());
  };

  let Ok(json) = serde_json::from_str::<serde_json::Value>(&json_file) else {
    return Err("Unable to parse json".to_string());
  };

  if json_file.contains("\"node:") {
    mach_config.engines.push("node".to_string());
  }

  if let Some(resolvers_value) = json.get("resolvers") {
    let mut resolvers = Vec::<String>::new();
    let Some(resolvers_values) = resolvers_value.as_array() else {
      return Err("'resolvers' should be array".to_string());
    };
    for resolver_value in resolvers_values {
      let Some(resolver_value) = resolver_value.as_str() else {
        return Err("'resolvers[n]' should be string".to_string());
      };
      resolvers.push(resolver_value.to_string());
    }
    mach_config.resolvers = Some(resolvers);
  };

  if let Some(transformers_value) = json.get("transformers") {
    let mut transformers = HashMap::<String, Vec<String>>::new();
    let Some(transformers_value) = transformers_value.as_object() else {
      return Err("'transformers' should be object".to_string());
    };
    for (key, value) in transformers_value {
      let mut values = Vec::<String>::new();
      let Some(value) = value.as_array() else {
        return Err("'transformers[key]' should be array".to_string());
      };
      for value in value {
        let Some(value) = value.as_str() else {
          return Err("'transformers[key][n]' should be string".to_string());
        };
        values.push(value.to_string());
      }
      transformers.insert(key.clone(), values);
    }
    mach_config.transformers = Some(transformers);
  }

  return Ok(mach_config);
}

fn _parse_json_file(target: &PathBuf) -> Result<serde_json::Value, String> {
  let Ok(json_file) = fs::read_to_string(target) else {
    return Err("Unable to read file".to_string());
  };
  let Ok(json) = serde_json::from_str::<serde_json::Value>(&json_file) else {
    return Err("Unable to parse json".to_string());
  };
  return Ok(json);
}

// fn parse_yaml_file(target: &PathBuf) -> Result<serde_yaml::Value, String> {
//   let Ok(yaml_file) = fs::read_to_string(target) else {
//     return Err("Unable to read file".to_string());
//   };
//   let Ok(yaml) = serde_yaml::from_str::<serde_yaml::Value>(&yaml_file) else {
//     return Err("Unable to parse json".to_string());
//   };
//   return Ok(yaml);
// }

// fn find_crawl_up(
//   start: &Path,
//   targets: &[&str],
// ) -> Option<(PathBuf, PathBuf)> {
//   let mut current = start.to_path_buf();

//   loop {
//     for entry in fs::read_dir(&current).unwrap() {
//       let Ok(entry) = entry else {
//         continue;
//       };
//       for target in targets {
//         if entry.file_name() == *target {
//           let complete = current.join(target);
//           return Some((current, complete));
//         }
//       }
//     }
//     if !current.pop() {
//       break;
//     }
//   }

//   None
// }

fn get_absolute_path(
  cwd: Option<PathBuf>,
  target: &Path,
) -> PathBuf {
  let file_path = target.to_path_buf();

  if file_path.is_absolute() {
    return file_path.normalize();
  }

  if let Some(cwd) = cwd {
    return cwd.join(target).normalize();
  }

  std::env::current_dir().unwrap().join(file_path).normalize()
}
