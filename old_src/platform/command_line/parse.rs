use std::collections::HashMap;

pub fn parse_command_line_args(cli_args: &str) -> (HashMap<String, Vec<String>>, String) {
  let mut args = HashMap::new();

  let mut key_buff = String::new();
  let mut value_buff = String::new();
  let mut cmd_buff = String::new();

  let mut pos = ENTRY;
  for c in cli_args.chars() {
    let c = c.to_string();

    if c == " " || c == "=" {
      pos = TABLE[pos][SPACE]
    } else if c == "-" {
      pos = TABLE[pos][DASH]
    } else {
      pos = TABLE[pos][CHAR]
    }

    if pos == KEY_END {
      if !args.contains_key(&key_buff) {
        args.insert(key_buff.clone(), vec![]);
      }
    }

    if pos == KEY_DATA {
      key_buff.push_str(c.as_str());
    }

    if pos == KEY_START {
      key_buff = String::new();
    }

    if pos == VALUE_END {
      args.get_mut(&key_buff).unwrap().push(value_buff.clone());
      value_buff = String::new();
      key_buff = String::new();
    }

    if pos == VALUE_DATA {
      value_buff.push_str(c.as_str());
    }

    if pos == VALUE_START {
      value_buff = String::from(c.clone());
    }

    if pos == COMMAND {
      cmd_buff.push_str(c.as_str());
    }
  }

  if key_buff.len() != 0 {
    if !args.contains_key(&key_buff) {
      args.insert(key_buff.clone(), vec![]);
    }
  }

  if value_buff.len() != 0 {
    args.get_mut(&key_buff).unwrap().push(value_buff.clone());
  }

  return (args, cmd_buff);
}

#[rustfmt::skip]
static TABLE: &'static [&'static [usize]] = &[
  //                char   /s  dash
  &[____,          ____, ____, ____],
  &[ENTRY,         CMND, ENTR, KSRT], // Start here
  &[COMMAND,       CMND, ENTR, CMND],
  &[KEY_START,     KDAT, ENTR, KSRT],
  &[KEY_DATA,      KDAT, KEND, KDAT],
  &[KEY_END,       VSRT, KEND, KSRT],
  &[VALUE_START,   VDAT, VEND, VDAT],
  &[VALUE_DATA,    VDAT, VEND, VDAT],
  &[VALUE_END,     CMND, ENTR, KSRT],
];

// const STOP: usize = 0;

// Tokens
const ____: usize = 0;
const ENTR: usize = 1;
const CMND: usize = 2;
const KSRT: usize = 3;
const KDAT: usize = 4;
const KEND: usize = 5;
const VSRT: usize = 6;
const VDAT: usize = 7;
const VEND: usize = 8;

// States
const ENTRY: usize = 1;
const COMMAND: usize = 2;
const KEY_START: usize = 3;
const KEY_DATA: usize = 4;
const KEY_END: usize = 5;
const VALUE_START: usize = 6;
const VALUE_DATA: usize = 7;
const VALUE_END: usize = 8;

// Columns
const CHAR: usize = 1;
const SPACE: usize = 2;
const DASH: usize = 3;
