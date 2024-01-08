use std::env;
use std::fs;
use std::path::PathBuf;

use normalize_path::NormalizePath;
use glob_match::glob_match;

use crate::platform::CommandArgs;
use crate::platform::CommandLineParseResult;

#[derive(Clone, Debug)]
pub enum WorkspaceKind {
  Pnpm,
  NpmOrYarn,
  None,
}

#[derive(Clone, Debug)]
pub struct AppConfig {
  /// CLI:
  ///   -i --entry
  ///   [cmd]
  pub entry_point: PathBuf,
  /// CLI
  ///   -o
  ///   --dist-dir
  pub dist_dir: PathBuf,
  /// Automatic
  pub workspace_root: Option<PathBuf>,
  /// Automatic
  pub workspace_kind: WorkspaceKind,
  /// Automatic
  pub project_root: PathBuf,
  /// CLI:
  ///   -t [i32]
  ///   --threads [i32]
  /// ENV:
  ///   env MACH_THREADS=[i32]
  pub threads: usize,
  /// CLI:
  ///   -z
  ///   --optimize
  /// ENV:
  ///   env MACH_OPTIMIZE=true
  pub optimize: bool,
}

impl AppConfig {
  pub fn from_env() -> Result<Self, String> {
    let args = CommandArgs::from_args(env::args());

    let Some(entry_point) = get_entry(&args) else {
      return Err("Missing entry point\n\tusage:\n\t\t--entry ./filepath\n\t\t-i ./filepath".to_string());
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

    let optimize = get_optimize(&args);

    return Ok(AppConfig {
      entry_point,
      dist_dir,
      workspace_root,
      workspace_kind,
      project_root,
      threads,
      optimize,
    });
  }
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

fn get_dist_dir(args: &CommandArgs, project_root: &PathBuf) -> PathBuf {
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
      },
      CommandLineParseResult::Err(err) => {
        return Err(format!("Unable to parse threads from cli args: {}", err))
      },
      CommandLineParseResult::MissingValue => {
        return Err(format!("Unable to parse threads from cli args: missing value"))
      },
      CommandLineParseResult::MissingKey => {},
  };

  return Ok(num_cpus::get());
}

fn get_optimize(args: &CommandArgs) -> bool {
  if let Ok(value) = env::var("MACH_OPTIMIZE") {
    return value == "true"
  }
  
  return args.get_all_bool(&["z", "optimize"]);
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

fn find_project_workspace(project_root: &PathBuf) -> Result<(Option<PathBuf>, WorkspaceKind), String> {
  let mut current_test = project_root.clone();

  loop {
    // PNPM Workspaces
    let possible_pnpm_yaml = current_test.join("pnpm-workspace.yaml");
    if possible_pnpm_yaml.exists() {
      let Ok(pnpm_yaml_file) = fs::read_to_string(possible_pnpm_yaml) else {
        return Err("Unable to read file".to_string());
      };
      let Ok(yaml) = serde_yaml::from_str::<serde_yaml::Value>(&pnpm_yaml_file) else {
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
      let Ok(package_json_file) = fs::read_to_string(possible_package_json) else {
        return Err("Unable to read file".to_string());
      };
      let Ok(json) = serde_json::from_str::<serde_json::Value>(&package_json_file) else {
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
