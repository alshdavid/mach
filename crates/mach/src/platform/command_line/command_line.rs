use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

use normalize_path::NormalizePath;

#[allow(dead_code)]
#[derive(Debug)]
pub struct CommandArgs {
  args: HashMap<String, Vec<String>>,
  command: String,
}

#[allow(dead_code)]
impl CommandArgs {
  pub fn from_args(env_args: env::Args) -> Self {
    let cli_args = env_args.skip(1).collect::<Vec<String>>().join(" ");
    return CommandArgs::from_string(&cli_args);
  }

  pub fn from_string(cli_args: &str) -> Self {
    let (args, command) = super::parse::parse_command_line_args(&cli_args);
    CommandArgs { args, command }
  }
  pub fn get_command(&self) -> Option<String> {
    if self.command == "" {
      return None;
    }
    return Some(self.command.clone());
  }

  pub fn get_command_as_path(&self) -> Option<PathBuf> {
    if self.command == "" {
      return None;
    }
    let mut file_path = PathBuf::from(&self.command);
    if !file_path.is_absolute() {
      file_path = env::current_dir().unwrap().join(file_path);
    }
    return Some(file_path.normalize());
  }

  pub fn get_string(&self, key: &str) -> Option<String> {
    let Some(mut values) = self.get_strings(key) else {
      return None;
    };
    if let Some(value) = values.pop() {
      return Some(value);
    }
    return None;
  }

  pub fn get_strings(&self, key: &str) -> Option<Vec<String>> {
    return self.get_all_strings(&[key]);
  }

  pub fn get_all_strings(&self, keys: &[&str]) -> Option<Vec<String>> {
    let mut collection = Vec::<Vec<String>>::new();

    for key in keys {
      let Some(values) = self.args.get(*key) else {
        continue;
      };
      collection.push(values.clone());
    }

    if collection.len() == 0 {
      return None;
    }

    return Some(collection.concat());
  }

  pub fn get_num(&self, key: &str) -> CommandLineParseResult<usize> {
    match self.get_nums(key) {
      CommandLineParseResult::Ok(v) => {
        if v.len() > 0 {
          return CommandLineParseResult::Ok(v[0]);
        }
        return CommandLineParseResult::MissingKey;
      }
      CommandLineParseResult::Err(e) => {
        return CommandLineParseResult::Err(e);
      }
      CommandLineParseResult::MissingKey => {
        return CommandLineParseResult::MissingKey;
      }
      CommandLineParseResult::MissingValue => {
        return CommandLineParseResult::MissingValue;
      }
    };
  }

  pub fn get_nums(&self, key: &str) -> CommandLineParseResult<Vec<usize>> {
    return self.get_all_nums(&[key]);
  }

  pub fn get_all_nums(&self, keys: &[&str]) -> CommandLineParseResult<Vec<usize>> {
    let Some(values) = self.get_all_strings(keys) else {
      return CommandLineParseResult::MissingKey;
    };

    let mut nums = Vec::<usize>::new();
    if values.len() == 0 {
      return CommandLineParseResult::MissingValue;
    }

    for value in values {
      let Ok(value) = value.parse::<usize>() else {
        return CommandLineParseResult::Err(format!("{}", value));
      };
      nums.push(value);
    }
    return CommandLineParseResult::Ok(nums);
  }

  pub fn get_bool(&self, key: &str) -> bool {
    return self.get_all_bool(&[key]);
  }

  pub fn get_all_bool(&self, keys: &[&str]) -> bool {
    for key in keys {
      let Some(values) = self.args.get(*key) else {
        continue;
      };
      if values.len() == 0 {
        return true;
      }
      for value in values {
        return value == "true";
      }
    }

    return false;
  }

  pub fn get_filepath(&self, key: &str) -> Result<PathBuf, String> {
    let mut values = match self.get_filepaths(key) {
      Ok(v) => v,
      Err(e) => return Err(e),
    };
    return Ok(values.pop().unwrap());
  }

  pub fn get_filepaths(&self, key: &str) -> Result<Vec<PathBuf>, String> {
    return self.get_all_filepaths(&[key]);
  }

  pub fn get_all_filepaths(&self, keys: &[&str]) -> Result<Vec<PathBuf>, String> {
    let Some(values) = self.get_all_strings(keys) else {
      return Err(format!("ArgsGetFilePath: Missing keys: {:?}", keys));
    };

    let mut file_paths = Vec::<PathBuf>::new();
    for value in values {
      let mut file_path = PathBuf::from(&value);
      if !file_path.is_absolute() {
        file_path = env::current_dir().unwrap().join(file_path);
      }
      file_paths.push(file_path.normalize());
    }

    return Ok(file_paths);
  }
}

pub enum CommandLineParseResult<T> {
  Err(String),
  Ok(T),
  MissingKey,
  MissingValue,
}
