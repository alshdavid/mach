enum BundleAssetExport {}
enum BundleAssetImport {}

struct BundleAsset {
  asset_id: String,
  use_symbols: Vec<String>,
  imports: Vec<BundleAssetImport>,
  exports: Vec<BundleAssetExport>
}

struct Bundle {
  assets: Vec<BundleAsset>
}
