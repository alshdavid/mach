use std::fs;
use std::path::PathBuf;

use clap::Parser;
use normalize_path::NormalizePath;

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

#[derive(Parser, Debug)]
pub struct VersionCommand {}

pub fn main() {
  let exe_path = std::env::current_exe().expect("Cannot find executable path");
  let possible_package_json = exe_path.join("../../../package.json").normalize();
  let mut npm_version = None::<String>;
  if possible_package_json.exists() {
    let package_json = parse_json_file(&possible_package_json).expect("package.json malformed");
    if let Some(version) = package_json.get("version") {
      let version = version.as_str().expect("package.json#version is invalid type");
      npm_version = Some(version.to_string());
    }
  }

  print!(r#"{color_red}{style_bold}"#);
  println!(r#"___  ___           _     "#);
  println!(r#"|  \/  |          | |    "#);
  println!(r#"| .  . | __ _  ___| |__  "#);
  println!(r#"| |\/| |/ _` |/ __| '_ \ "#);
  println!(r#"| |  | | (_| | (__| | | |"#);
  println!(r#"\_|  |_/\__,_|\___|_| |_|"#);
  print!(r#"{color_reset}{style_reset}"#);
  println!(r#""#);
  println!(r#"{style_bold}Description{style_reset}   {DESCRIPTION}"#);
  println!(r#"{style_bold}Repository{style_reset}    {REPOSITORY}"#);
  if let Some(npm_version) = npm_version {
    println!(r#"{style_bold}NPM Version{style_reset}   {npm_version}"#);
    println!(r#"{style_bold}Bin Version{style_reset}   {VERSION}"#);
  } else {
    println!(r#"{style_bold}Version{style_reset}       {VERSION}"#);
  }
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