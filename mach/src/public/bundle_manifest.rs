use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

pub type BundleManifestSync = Arc<RwLock<HashMap<String, String>>>;
pub type BundleManifest = HashMap<String, String>;
