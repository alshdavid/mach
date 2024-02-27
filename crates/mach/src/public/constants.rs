use std::path::PathBuf;

use once_cell::sync::Lazy;

pub static ENTRY_ASSET: Lazy<PathBuf> = Lazy::new(|| PathBuf::from(""));
pub static NO_ASSET: Lazy<PathBuf> = Lazy::new(|| PathBuf::from(""));
pub static ID_TRUNC: usize = 15;
