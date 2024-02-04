use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

use glob_match::glob_match;
use normalize_path::NormalizePath;

use crate::platform::CommandArgs;
use crate::platform::CommandLineParseResult;
use crate::public::Config;
use crate::public::Machrc;
use crate::public::WorkspaceKind;

type FileIndex = HashMap<String, Vec<PathBuf>>;

pub fn app_config() -> Result<Config, String> {
  let args = CommandArgs::from_args(env::args());

  let Some(entry_point) = get_entry(&args) else {
    return Err(
      "Missing entry point\n\tusage:\n\t\t--entry ./filepath\n\t\t-i ./filepath".to_string(),
    );
  };

  let project_root = find_project_root(&entry_point);

  let Ok((workspace_root, workspace_kind)) = find_project_workspace(&project_root) else {
    return Err("Error while looking up workspace".to_string());
  };

  let dist_dir = get_dist_dir(&args, &project_root);

  let threads = match get_threads(&args) {
    Ok(v) => v,
    Err(err) => return Err(err),
  };

  let node_workers = match get_node_workers(&args) {
    Ok(v) => v,
    Err(err) => return Err(err),
  };

  let optimize = get_optimize(&args);

  let file_index = find_file_by_name(&project_root, &["package.json", ".machrc", "pnpm-workspace.yaml"]);
  let mach_config = parse_machrc(&file_index).unwrap();

  return Ok(Config {
    entry_point,
    dist_dir,
    workspace_root,
    workspace_kind,
    project_root,
    threads,
    node_workers,
    optimize,
    env: get_env(),
    package_json: None,
    mach_config,
  });
}

fn get_entry(args: &CommandArgs) -> Option<PathBuf> {
  let Ok(filepaths) = args.get_all_filepaths(&["i", "entry"]) else {
    if let Some(cmd) = args.get_command_as_path() {
      return Some(cmd);
    }
    return None;
  };
  return Some(filepaths[0].clone());
}

fn get_dist_dir(
  args: &CommandArgs,
  project_root: &PathBuf,
) -> PathBuf {
  let Ok(filepaths) = args.get_all_filepaths(&["o", "dist-dir"]) else {
    return project_root.join("dist");
  };
  return filepaths[0].clone();
}

fn get_threads(args: &CommandArgs) -> Result<usize, String> {
  // Use env
  if let Ok(threads_env) = env::var("MACH_THREADS") {
    let Ok(parse_res) = threads_env.parse::<usize>() else {
      return Err(format!("Unable to parse threads from env: {}", threads_env));
    };
    return Ok(parse_res);
  };

  // From CLI
  match args.get_all_nums(&["t", "threads"]) {
    CommandLineParseResult::Ok(threads_cli) => {
      if threads_cli.len() > 0 {
        return Ok(threads_cli[0]);
      }
    }
    CommandLineParseResult::Err(err) => {
      return Err(format!("Unable to parse threads from cli args: {}", err))
    }
    CommandLineParseResult::MissingValue => {
      return Err(format!(
        "Unable to parse threads from cli args: missing value"
      ))
    }
    CommandLineParseResult::MissingKey => {}
  };

  return Ok(num_cpus::get());
}

fn get_node_workers(args: &CommandArgs) -> Result<usize, String> {
  // Use env
  if let Ok(threads_env) = env::var("MACH_NODE_WORKERS") {
    let Ok(parse_res) = threads_env.parse::<usize>() else {
      return Err(format!("Unable to parse workers from env: {}", threads_env));
    };
    return Ok(parse_res);
  };

  // From CLI
  match args.get_all_nums(&["node_workers"]) {
    CommandLineParseResult::Ok(threads_cli) => {
      if threads_cli.len() > 0 {
        return Ok(threads_cli[0]);
      }
    }
    CommandLineParseResult::Err(err) => {
      return Err(format!("Unable to parse threads from cli args: {}", err))
    }
    CommandLineParseResult::MissingValue => {
      return Err(format!(
        "Unable to parse threads from cli args: missing value"
      ))
    }
    CommandLineParseResult::MissingKey => {}
  };

  return Ok(1);
}

fn get_optimize(args: &CommandArgs) -> bool {
  if let Ok(value) = env::var("MACH_OPTIMIZE") {
    return value == "true";
  }

  return args.get_all_bool(&["z", "optimize"]);
}

fn get_env() -> HashMap<String, String> {
  let mut vars = HashMap::<String, String>::new();
  for (k, v) in env::vars() {
    vars.insert(k, v);
  }
  return vars;
}

fn find_project_root(entry: &PathBuf) -> PathBuf {
  let mut current_test = entry.clone();
  loop {
    if current_test.join("package.json").exists() {
      break;
    }
    if !current_test.pop() {
      return env::current_dir().unwrap();
    }
  }
  return current_test;
}

// TODO use index to figure this out
fn find_project_workspace(
  project_root: &PathBuf,
) -> Result<(Option<PathBuf>, WorkspaceKind), String> {
  let mut current_test = project_root.clone();

  loop {
    // PNPM Workspaces
    let possible_pnpm_yaml = current_test.join("pnpm-workspace.yaml");
    if possible_pnpm_yaml.exists() {
      let Ok(yaml) = parse_yaml_file(&possible_pnpm_yaml) else {
        return Err("Unable to parse json".to_string());
      };
      let Some(workspaces_value) = yaml.get("packages") else {
        if !current_test.pop() {
          return Ok((None, WorkspaceKind::None));
        }
        continue;
      };
      let Some(workspaces) = workspaces_value.as_sequence() else {
        if !current_test.pop() {
          return Ok((None, WorkspaceKind::None));
        }
        continue;
      };
      for workspace in workspaces {
        let pattern_path = current_test.join(workspace.as_str().unwrap()).normalize();
        let pattern = pattern_path.to_str().unwrap();
        let has_match = glob_match(pattern, project_root.to_str().unwrap());
        if has_match {
          return Ok((Some(current_test), WorkspaceKind::Pnpm));
        }
      }
    }

    // Yarn and NPM Workspaces
    let possible_package_json = current_test.join("package.json");
    if possible_package_json.exists() {
      let Ok(json) = parse_json_file(&possible_package_json) else {
        return Err("Unable to parse json".to_string());
      };
      let Some(workspaces_value) = json.get("workspaces") else {
        if !current_test.pop() {
          return Ok((None, WorkspaceKind::None));
        }
        continue;
      };
      let Some(workspaces) = workspaces_value.as_array() else {
        if !current_test.pop() {
          return Ok((None, WorkspaceKind::None));
        }
        continue;
      };
      for workspace in workspaces {
        let pattern_path = current_test.join(workspace.as_str().unwrap()).normalize();
        let pattern = pattern_path.to_str().unwrap();
        let has_match = glob_match(pattern, project_root.to_str().unwrap());
        if has_match {
          return Ok((Some(current_test), WorkspaceKind::NpmOrYarn));
        }
      }
    }

    if !current_test.pop() {
      return Ok((None, WorkspaceKind::None));
    }
  }
}

fn parse_machrc(file_index: &FileIndex) -> Result<Option<Machrc>, String> {
  let Some(mach_configs) = file_index.get(".machrc") else {
    return Ok(None);
  };

  if mach_configs.len() == 0 {
    return Ok(None);
  }

  let file_path = mach_configs[0].clone();

  let mut mach_config = Machrc {
    file_path,
    resolvers: None,
  };

  let json = parse_json_file(&mach_config.file_path).unwrap();

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

  return Ok(Some(mach_config));
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

fn parse_yaml_file(target: &PathBuf) -> Result<serde_yaml::Value, String> {
  let Ok(yaml_file) = fs::read_to_string(target) else {
    return Err("Unable to read file".to_string());
  };
  let Ok(yaml) = serde_yaml::from_str::<serde_yaml::Value>(&yaml_file) else {
    return Err("Unable to parse json".to_string());
  };
  return Ok(yaml);
}

fn find_file_by_name(start_path: &PathBuf, targets: &[&str]) -> FileIndex {
  let mut found = FileIndex::new();
  let mut current_test = start_path.clone();

  for target in targets {
    found.insert(target.to_string(), vec![]);
  }

  for target in targets {
    let Some(file_name) = current_test.file_name() else {
      break;
    };

    let target = target.to_string();

    if file_name.to_str().unwrap() == target {
      found.get_mut(&target).unwrap().push(current_test.clone());
      if !current_test.pop() {
        return found;
      }
      break;
    }
  }
  
  loop {
    let Ok(ls) = current_test.read_dir() else {
      return found;
    };

    for item in ls {
      let Ok(item) = item else {
        continue;
      };

      let file_name = item.file_name();

      for target in targets {
        let target = target.to_string();
        if file_name.to_str().unwrap() == target {
          found.get_mut(&target).unwrap().push(current_test.join(target));
          continue;
        }
      }
    }

    if !current_test.pop() {
      return found;
    }
  }
}
