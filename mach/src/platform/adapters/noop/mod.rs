use std::{collections::HashMap, path::PathBuf};

use crate::public::{Adapter, Dependency, ResolveResult, Resolver};
use async_trait::async_trait;

pub struct NoopAdapter {}

impl Adapter for NoopAdapter {
    fn bootstrap(_: std::collections::HashMap<String, String>) -> Self {
        return NoopAdapter{};
    }
    
    fn get_resolver(&self, _: HashMap<String, String>) -> Box<dyn Resolver> {
        return Box::new(NoopResolver{});
    }

}

#[derive(Debug)]
pub struct NoopResolver {}

#[async_trait]
impl Resolver for NoopResolver {
    async fn resolve(&self, _: &Dependency) -> Result<Option<ResolveResult>, String> {
        return Ok(None);
    }
}
