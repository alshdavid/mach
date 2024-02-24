use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

use clap::Parser;
use normalize_path::NormalizePath;

use crate::args::Args;
use crate::public::Config;
use crate::public::Machrc;

type FileIndex = HashMap<String, Vec<PathBuf>>;

pub fn parse_config() -> Result<Config, String> {
  let start_time = SystemTime::now();

  let args = Args::parse();

  // Ignore multiple entries for now
  let entry_point = get_absolute_path(&args.entry[0].clone());

  // Find these points of interest
  let file_index = find_file_by_name(
    &entry_point,
    &["package.json", ".machrc", "pnpm-workspace.yaml"],
  );

  let machrc = parse_machrc(&file_index).expect("Cannot parse .machrc");

  let mut node_workers = args.node_workers.unwrap();
  if !machrc.engines.contains(&"node".to_string()) {
    node_workers = 0;
  }

  // Project root is the location of the first package.json
  let package_json_path = file_index
    .get("package.json")
    .unwrap()
    .get(0)
    .unwrap()
    .clone();
  let package_json = parse_json_file(&package_json_path).expect("Cannot parse package.json");

  let project_root = package_json_path.parent().unwrap().to_path_buf();

  let dist_dir = {
    let dist_dir_arg = args.out_folder.unwrap();
    if dist_dir_arg.is_absolute() {
      dist_dir_arg
    } else {
      project_root.join(dist_dir_arg).normalize()
    }
  };

  let threads = {
    if let Some(threads) = args.threads {
      threads
    } else {
      num_cpus::get()
    }
  };

  let env = {
    let mut vars = HashMap::<String, String>::new();
    for (k, v) in std::env::vars() {
      vars.insert(k, v);
    }
    vars
  };

  return Ok(Config {
    start_time,
    entry_point,
    dist_dir,
    // TODO
    workspace_root: None,
    // TODO
    workspace_kind: None,
    project_root,
    package_json,
    machrc,
    threads,
    node_workers,
    optimize: args.optimize.unwrap(),
    env,
  });
}

fn parse_machrc(file_index: &FileIndex) -> Result<Machrc, String> {
  let Some(mach_configs) = file_index.get(".machrc") else {
    return Ok(Machrc::default());
  };

  if mach_configs.len() == 0 {
    return Ok(Machrc::default());
  }

  let file_path = mach_configs[0].clone();

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

fn parse_json_file(target: &PathBuf) -> Result<serde_json::Value, String> {
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

fn find_file_by_name(
  start_path: &PathBuf,
  targets: &[&str],
) -> FileIndex {
  let mut found = FileIndex::new();
  for target in targets {
    found.insert(target.to_string(), vec![]);
  }

  let mut current_test = start_path.clone();

  let mut paths_to_test = Vec::<PathBuf>::new();
  paths_to_test.push(current_test.clone());
  while current_test.pop() {
    paths_to_test.push(current_test.clone())
  }
  paths_to_test.reverse();

  while let Some(path) = paths_to_test.pop() {
    if path.is_file() {
      let Some(file_name) = path.file_name() else {
        continue;
      };
      let file_name = file_name.to_str().unwrap();

      for target in targets {
        let target = target.to_string();
        if file_name == target {
          found.get_mut(&target).unwrap().push(path.join(target));
          continue;
        }
      }
    } else if path.is_dir() {
      let Ok(ls) = path.read_dir() else {
        panic!("failed to ls");
      };

      for item in ls {
        let Ok(item) = item else {
          panic!("failed to ls");
        };

        let file_name = item.file_name();
        let file_name = file_name.to_str().unwrap();

        for target in targets {
          let target = target.to_string();
          if file_name == target {
            found.get_mut(&target).unwrap().push(path.join(target));
            continue;
          }
        }
      }
    }
  }
  return found;
}

fn get_absolute_path(target: &PathBuf) -> PathBuf {
  let mut file_path = PathBuf::from(&target);
  if !file_path.is_absolute() {
    file_path = std::env::current_dir().unwrap().join(file_path);
  }
  return file_path.normalize();
}