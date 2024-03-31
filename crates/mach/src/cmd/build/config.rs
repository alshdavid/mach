use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;

use clap::Parser;
use normalize_path::NormalizePath;

use libmach::MachConfig;
use libmach::Machrc;

type FileIndex = HashMap<String, Vec<PathBuf>>;

#[derive(Parser, Debug)]
pub struct BuildCommand {
  /// Input file to build
  pub entry: Option<Vec<PathBuf>>,

  /// Output folder
  #[arg(short = 'o', long = "dist", default_value = "dist")]
  pub out_folder: PathBuf,

  /// Delete output folder before emitting files
  #[arg(short = 'c', long = "clean")]
  pub clean: bool,

  /// Disable optimizations
  #[arg(long = "no-optimize")]
  pub no_optimize: bool,

  /// Enable bundle splitting (experimental)
  #[arg(long = "bundle-splitting")]
  pub bundle_splitting: bool,

  /// How many threads to use for compilation
  #[arg(short = 't', long = "threads", env = "MACH_THREADS")]
  pub threads: Option<usize>,

  /// How many Node.js workers to spawn for plugins
  #[arg(long = "node-workers", env = "MACH_NODE_WORKERS", default_value = "1")]
  pub node_workers: Option<usize>,

  /// Enable logging debug info
  #[arg(long = "debug")]
  pub debug: bool,
}

pub fn parse_config(command: BuildCommand) -> Result<MachConfig, String> {
  let start_time = SystemTime::now();

  let entry_arg = 'block: {
    if let Some(args) = command.entry {
      break 'block args[0].clone();
    }
    break 'block std::env::current_dir().unwrap();
  };
  // Ignore multiple entries for now
  let Some(entry_point) = get_entry(&entry_arg) else {
    return Err("Could not find entry point".to_string());
  };

  // Find these points of interest
  let file_index = find_file_by_name(
    &entry_point,
    &["package.json", ".machrc", "pnpm-workspace.yaml"],
  );

  let machrc = parse_machrc(&file_index).expect("Cannot parse .machrc");

  let mut node_workers = command.node_workers.unwrap();
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
    let dist_dir_arg = command.out_folder;
    if dist_dir_arg.is_absolute() {
      dist_dir_arg
    } else {
      project_root.join(dist_dir_arg).normalize()
    }
  };

  let threads = {
    if let Some(threads) = command.threads {
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

  return Ok(MachConfig {
    start_time,
    entry_point,
    dist_dir,
    clean_dist_dir: command.clean,
    // TODO
    workspace_root: None,
    // TODO
    workspace_kind: None,
    bundle_splitting: command.bundle_splitting,
    project_root,
    package_json: Arc::new(package_json),
    machrc,
    threads,
    node_workers,
    optimize: !command.no_optimize,
    env,
    debug: command.debug,
  });
}

fn get_entry(entry_arg: &Path) -> Option<PathBuf> {
  let absolute = get_absolute_path(entry_arg);

  if absolute.is_file() {
    return Some(absolute.to_path_buf());
  }

  for test in [
    absolute.join("index.html"),
    absolute.join("index.tsx"),
    absolute.join("index.ts"),
    absolute.join("index.jsx"),
    absolute.join("index.js"),
    absolute.join("src").join("index.html"),
    absolute.join("src").join("index.tsx"),
    absolute.join("src").join("index.ts"),
    absolute.join("src").join("index.jsx"),
    absolute.join("src").join("index.js"),
  ] {
    if test.exists() {
      return Some(test);
    }
  }

  return None;
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

fn get_absolute_path(target: &Path) -> PathBuf {
  let mut file_path = PathBuf::from(&target);
  if !file_path.is_absolute() {
    file_path = std::env::current_dir().unwrap().join(file_path);
  }
  return file_path.normalize();
}
