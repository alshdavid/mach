use std::path::PathBuf;

use once_cell::sync::Lazy;

pub static ROOT_NODE: Lazy<PathBuf> = Lazy::new(|| PathBuf::from(""));
