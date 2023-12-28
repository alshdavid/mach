use std::env;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;

#[derive(Clone)]
pub struct AppConfig {
  pub entry_point: PathBuf,
  pub dist_dir: PathBuf,
  pub project_root: PathBuf,
  pub threads: usize,
  pub optimize: bool,
}

impl AppConfig {
  pub fn new() -> Self {
    let entry_point = get_entry();
    let project_root = find_project_root(&entry_point);
    let dist_dir = get_dist_dir(&project_root);
    let threads = get_threads();
    let optimize = get_optimize();

    return AppConfig {
      entry_point,
      dist_dir,
      project_root,
      threads,
      optimize,
    };
  }
}

fn get_entry() -> PathBuf {
  let filepath_str = std::env::args().nth(1).expect("No filepath given");
  let filepath = PathBuf::from(&filepath_str);
  if filepath.is_absolute() {
    return filepath.to_owned();
  }
  let cwd = env::current_dir().unwrap();
  let absolute_file_path = normalize_path(&cwd.join(filepath.clone()));
  return absolute_file_path;
}

fn get_dist_dir(project_root: &PathBuf) -> PathBuf {
  let Some(filepath_str) = std::env::args().nth(2) else {
    return project_root.join("dist");
  };
  let filepath = PathBuf::from(&filepath_str);
  if filepath.is_absolute() {
    return filepath.to_owned();
  }
  let cwd = env::current_dir().unwrap();
  let absolute_file_path = normalize_path(&cwd.join(filepath.clone()));
  return absolute_file_path;
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

fn get_threads() -> usize {
  let mut threads = num_cpus::get();
  let threads_res = env::var("MACH_THREADS");

  if threads_res.is_ok() {
    let parse_res = parse_usize(&threads_res.unwrap());
    if parse_res.is_err() {
      panic!("Unable to parse MACH_THREADS variable - not an int")
    }
    threads = parse_res.unwrap();
    if threads == 0 {
      panic!("Threads must be more than 0");
    }
  }
  return threads;
}

fn get_optimize() -> bool {
  return match env::var("MACH_OPTIMIZE") {
    Ok(v) => v == "true",
    Err(_) => false,
  };
}

fn parse_usize(str: &str) -> Result<usize, ()> {
  let parse_opt: Result<usize, _> = str.parse();
  if parse_opt.is_err() {
    return Err(());
  }
  return Ok(parse_opt.unwrap());
}

fn normalize_path(path: &Path) -> PathBuf {
  let mut components = path.components().peekable();
  let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().cloned() {
    components.next();
    PathBuf::from(c.as_os_str())
  } else {
    PathBuf::new()
  };

  for component in components {
    match component {
      Component::Prefix(..) => unreachable!(),
      Component::RootDir => {
        ret.push(component.as_os_str());
      }
      Component::CurDir => {}
      Component::ParentDir => {
        ret.pop();
      }
      Component::Normal(c) => {
        ret.push(c);
      }
    }
  }
  return ret;
}
