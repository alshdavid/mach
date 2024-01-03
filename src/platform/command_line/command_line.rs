    use std::collections::HashMap;
use std::env;

#[allow(dead_code)]

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
      CommandArgs{
        args,
        command,
      }
    }

    pub fn get_string(&self, key: &str) -> Result<String, String> {
      let values = match self.get_strings(key) {
        Ok(v) => v,
        Err(e) => return Err(e),
      };
      return Ok(values[0].clone());
    }

    pub fn get_strings(&self, key: &str) -> Result<Vec<String>, String> {
      let Some(values) = self.args.get(key) else {
        return Err("Arg not found".to_string());
      };
      if values.len() < 1 {
        return Err("Incorrect type".to_string());
      }
      return Ok(values.clone());
    }

    pub fn get_num(&self, key: &str) -> Result<usize, String> {
      let values = match self.get_nums(key) {
        Ok(v) => v,
        Err(e) => return Err(e),
      };
      return Ok(values[0]);
    }

    pub fn get_nums(&self, key: &str) -> Result<Vec<usize>, String> {
      let Some(values) = self.args.get(key) else {
        return Err("Arg not found".to_string());
      };
      if values.len() < 1 {
        return Err("Incorrect type".to_string());
      }
      let mut nums = Vec::<usize>::new();
      for value in values {
        let Ok(value) = value.parse::<usize>() else {
          return Err("Incorrect type".to_string());
        };
        nums.push(value);
      }
      return Ok(nums);
    }
}
