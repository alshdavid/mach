use std::path::PathBuf;

use once_cell::sync::Lazy;

pub static ENTRY_ASSET: Lazy<PathBuf> = Lazy::new(|| PathBuf::from(""));
