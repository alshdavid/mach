use clap::Parser;

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
  println!(r#"{style_bold}Version{style_reset}       {VERSION}"#);
}
