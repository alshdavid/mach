// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

use crate::deno_cli::args::CompileFlags;
use crate::deno_cli::args::Flags;
use crate::deno_cli::factory::CliFactory;
use crate::deno_cli::standalone::is_standalone_binary;
use crate::deno_cli::util::path::path_has_trailing_slash;
use deno_core::anyhow::bail;
use deno_core::anyhow::Context;
use deno_core::error::generic_error;
use deno_core::error::AnyError;
use deno_core::resolve_url_or_path;
use deno_graph::GraphKind;
use deno_terminal::colors;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use super::installer::infer_name_from_url;

pub async fn compile(
  flags: Flags,
  compile_flags: CompileFlags,
) -> Result<(), AnyError> {
  let factory = CliFactory::from_flags(flags).await?;
  let cli_options = factory.cli_options();
  let module_graph_creator = factory.module_graph_creator().await?;
  let parsed_source_cache = factory.parsed_source_cache();
  let binary_writer = factory.create_compile_binary_writer().await?;
  let module_specifier = cli_options.resolve_main_module()?;
  let module_roots = {
    let mut vec = Vec::with_capacity(compile_flags.include.len() + 1);
    vec.push(module_specifier.clone());
    for side_module in &compile_flags.include {
      vec.push(resolve_url_or_path(side_module, cli_options.initial_cwd())?);
    }
    vec
  };

  // this is not supported, so show a warning about it, but don't error in order
  // to allow someone to still run `deno compile` when this is in a deno.json
  if cli_options.unstable_sloppy_imports() {
    log::warn!(
      concat!(
        "{} Sloppy imports are not supported in deno compile. ",
        "The compiled executable may encouter runtime errors.",
      ),
      crate::deno_cli::colors::yellow("Warning"),
    );
  }

  let output_path =
    resolve_compile_executable_output_path(&compile_flags, cli_options.initial_cwd()).await?;

  let graph = Arc::try_unwrap(
    module_graph_creator
      .create_graph_and_maybe_check(module_roots.clone())
      .await?,
  )
  .unwrap();
  let graph = if cli_options.type_check_mode().is_true() {
    // In this case, the previous graph creation did type checking, which will
    // create a module graph with types information in it. We don't want to
    // store that in the eszip so create a code only module graph from scratch.
    module_graph_creator
      .create_graph(GraphKind::CodeOnly, module_roots)
      .await?
  } else {
    graph
  };

  let ts_config_for_emit =
    cli_options.resolve_ts_config_for_emit(deno_config::TsConfigType::Emit)?;
  let emit_options = crate::deno_cli::args::ts_config_to_emit_options(ts_config_for_emit.ts_config);
  let parser = parsed_source_cache.as_capturing_parser();
  let eszip = eszip::EszipV2::from_graph(graph, &parser, emit_options)?;

  log::info!(
    "{} {} to {}",
    colors::green("Compile"),
    module_specifier.to_string(),
    output_path.display(),
  );
  validate_output_path(&output_path)?;

  let mut file = std::fs::File::create(&output_path)
    .with_context(|| format!("Opening file '{}'", output_path.display()))?;
  let write_result = binary_writer
    .write_bin(
      &mut file,
      eszip,
      &module_specifier,
      &compile_flags,
      cli_options,
    )
    .await
    .with_context(|| format!("Writing {}", output_path.display()));
  drop(file);
  if let Err(err) = write_result {
    // errored, so attempt to remove the output path
    let _ = std::fs::remove_file(output_path);
    return Err(err);
  }

  // set it as executable
  #[cfg(unix)]
  {
    use std::os::unix::fs::PermissionsExt;
    let perms = std::fs::Permissions::from_mode(0o777);
    std::fs::set_permissions(output_path, perms)?;
  }

  Ok(())
}

/// This function writes out a final binary to specified path. If output path
/// is not already standalone binary it will return error instead.
fn validate_output_path(output_path: &Path) -> Result<(), AnyError> {
  if output_path.exists() {
    // If the output is a directory, throw error
    if output_path.is_dir() {
      bail!(
        concat!(
          "Could not compile to file '{}' because a directory exists with ",
          "the same name. You can use the `--output <file-path>` flag to ",
          "provide an alternative name."
        ),
        output_path.display()
      );
    }

    // Make sure we don't overwrite any file not created by Deno compiler because
    // this filename is chosen automatically in some cases.
    if !is_standalone_binary(output_path) {
      bail!(
        concat!(
          "Could not compile to file '{}' because the file already exists ",
          "and cannot be overwritten. Please delete the existing file or ",
          "use the `--output <file-path>` flag to provide an alternative name."
        ),
        output_path.display()
      );
    }

    // Remove file if it was indeed a deno compiled binary, to avoid corruption
    // (see https://github.com/denoland/deno/issues/10310)
    std::fs::remove_file(output_path)?;
  } else {
    let output_base = &output_path.parent().unwrap();
    if output_base.exists() && output_base.is_file() {
      bail!(
        concat!(
          "Could not compile to file '{}' because its parent directory ",
          "is an existing file. You can use the `--output <file-path>` flag to ",
          "provide an alternative name.",
        ),
        output_base.display(),
      );
    }
    std::fs::create_dir_all(output_base)?;
  }

  Ok(())
}

async fn resolve_compile_executable_output_path(
  compile_flags: &CompileFlags,
  current_dir: &Path,
) -> Result<PathBuf, AnyError> {
  let module_specifier = resolve_url_or_path(&compile_flags.source_file, current_dir)?;

  let mut output = compile_flags.output.clone();

  if let Some(out) = output.as_ref() {
    if path_has_trailing_slash(out) {
      if let Some(infer_file_name) = infer_name_from_url(&module_specifier)
        .await
        .map(PathBuf::from)
      {
        output = Some(out.join(infer_file_name));
      }
    } else {
      output = Some(out.to_path_buf());
    }
  }

  if output.is_none() {
    output = infer_name_from_url(&module_specifier)
      .await
      .map(PathBuf::from)
  }

  output
    .ok_or_else(|| {
      generic_error(
        "An executable name was not provided. One could not be inferred from the URL. Aborting.",
      )
    })
    .map(|output| get_os_specific_filepath(output, &compile_flags.target))
}

fn get_os_specific_filepath(
  output: PathBuf,
  target: &Option<String>,
) -> PathBuf {
  let is_windows = match target {
    Some(target) => target.contains("windows"),
    None => cfg!(windows),
  };
  if is_windows && output.extension().unwrap_or_default() != "exe" {
    if let Some(ext) = output.extension() {
      // keep version in my-exe-0.1.0 -> my-exe-0.1.0.exe
      output.with_extension(format!("{}.exe", ext.to_string_lossy()))
    } else {
      output.with_extension("exe")
    }
  } else {
    output
  }
}
