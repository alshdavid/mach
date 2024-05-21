use std::collections::HashMap;

use super::AssetId;

pub type AssetContents = HashMap<AssetId, Vec<u8>>;
