#![allow(unused)]

use super::Mach;

#[allow(non_upper_case_globals)]
const color_red: &str = "\x1B[31m";
#[allow(non_upper_case_globals)]
const color_reset: &str = "\x1B[39m";
#[allow(non_upper_case_globals)]
const style_bold: &str = "\x1B[1m";
#[allow(non_upper_case_globals)]
const style_reset: &str = "\x1B[0m";

const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

pub struct VersionOptions {}

pub struct VersionResult {
  pub description: String,
  pub version: String,
  pub repository: String,
  pub pretty: String,
}

impl Mach {
  pub fn version(
    &self,
    options: VersionOptions,
  ) -> VersionResult {
    let mut pretty = String::new();

    pretty.push_str(&format!(r#"{color_red}{style_bold}"#));
    pretty.push_str(&format!(r#"___  ___           _     {}"#, "\n"));
    pretty.push_str(&format!(r#"|  \/  |          | |    {}"#, "\n"));
    pretty.push_str(&format!(r#"| .  . | __ _  ___| |__  {}"#, "\n"));
    pretty.push_str(&format!(r#"| |\/| |/ _` |/ __| '_ \ {}"#, "\n"));
    pretty.push_str(&format!(r#"| |  | | (_| | (__| | | |{}"#, "\n"));
    pretty.push_str(&format!(r#"\_|  |_/\__,_|\___|_| |_|{}"#, "\n"));
    pretty.push_str(&format!(r#"{color_reset}{style_reset}"#));
    pretty.push_str(&format!(r#"{}"#, "\n"));
    pretty.push_str(&format!(
      r#"{style_bold}Description{style_reset}   {DESCRIPTION}{}"#,
      "\n"
    ));
    pretty.push_str(&format!(
      r#"{style_bold}Repository{style_reset}    {REPOSITORY}{}"#,
      "\n"
    ));
    pretty.push_str(&format!(
      r#"{style_bold}Version{style_reset}       {VERSION}{}"#,
      "\n"
    ));

    VersionResult {
      description: DESCRIPTION.to_string(),
      version: VERSION.to_string(),
      repository: REPOSITORY.to_string(),
      pretty,
    }
  }
}
