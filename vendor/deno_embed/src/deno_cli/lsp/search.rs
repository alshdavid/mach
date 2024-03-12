// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

use deno_core::error::AnyError;
use deno_semver::package::PackageNv;
use deno_semver::Version;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait PackageSearchApi {
  async fn search(&self, query: &str) -> Result<Arc<Vec<String>>, AnyError>;
  async fn versions(&self, name: &str) -> Result<Arc<Vec<Version>>, AnyError>;
  async fn exports(&self, nv: &PackageNv)
    -> Result<Arc<Vec<String>>, AnyError>;
}
