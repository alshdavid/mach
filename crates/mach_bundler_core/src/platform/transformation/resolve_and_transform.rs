use super::run_resolvers::run_resolvers;
use crate::platform::config::PluginContainerSync;
use crate::platform::config::ROOT_NODE;
use crate::public::Compilation;
use crate::public::Dependency;
use crate::public::MachConfigSync;

pub fn resolve_and_transform(
  config: MachConfigSync,
  plugins: PluginContainerSync,
  compilation: Compilation,
) -> Result<(), String> {
  let mut queue = vec![];

  queue.push(Dependency {
    specifier: config.entry_point.to_str().unwrap().to_string(),
    is_entry: true,
    source_path: ROOT_NODE.clone(),
    resolve_from: ROOT_NODE.clone(),
    ..Dependency::default()
  });

  while let Some(dependency) = queue.pop() {
    let resolve_result = run_resolvers(&plugins, &dependency)?;

    println!("{:?}", resolve_result);
  }

  Ok(())
}
