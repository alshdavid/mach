use crate::platform::config::PluginContainerSync;
use crate::public::Dependency;
use crate::public::ResolveResult;

pub fn run_resolvers(
  plugins: &PluginContainerSync,
  dependency: &Dependency,
) -> Result<ResolveResult, String> {
  for resolver in &plugins.resolvers {
    if let Some(resolve_result) = resolver.resolve(&dependency)? {
      return Ok(resolve_result);
    }
  }
  return Err(format!("Unable to resolve file: \n\tSpecifier: {}\n\tFrom: {:?}", dependency.specifier, dependency.resolve_from));
}
