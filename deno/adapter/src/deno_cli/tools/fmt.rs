// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

//! This module provides file formatting utilities using
//! [`dprint-plugin-typescript`](https://github.com/dprint/dprint-plugin-typescript).
//!
//! At the moment it is only consumed using CLI but in
//! the future it can be easily extended to provide
//! the same functions as ops available in JS runtime.

use crate::deno_cli::args::CliOptions;
use crate::deno_cli::args::Flags;
use crate::deno_cli::args::FmtFlags;
use crate::deno_cli::args::FmtOptions;
use crate::deno_cli::args::FmtOptionsConfig;
use crate::deno_cli::args::ProseWrap;
use crate::deno_cli::colors;
use crate::deno_cli::factory::CliFactory;
use crate::deno_cli::util::diff::diff;
use crate::deno_cli::util::file_watcher;
use crate::deno_cli::util::fs::canonicalize_path;
use crate::deno_cli::util::fs::FileCollector;
use crate::deno_cli::util::path::get_extension;
use deno_ast::ParsedSource;
use deno_config::glob::FilePatterns;
use deno_core::anyhow::anyhow;
use deno_core::anyhow::bail;
use deno_core::anyhow::Context;
use deno_core::error::generic_error;
use deno_core::error::AnyError;
use deno_core::futures;
use deno_core::parking_lot::Mutex;
use deno_core::unsync::spawn_blocking;
use log::debug;
use log::info;
use log::warn;
use std::fs;
use std::io::stdin;
use std::io::stdout;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use crate::deno_cli::cache::IncrementalCache;

/// Format JavaScript/TypeScript files.
pub async fn format(
  flags: Flags,
  fmt_flags: FmtFlags,
) -> Result<(), AnyError> {
  if fmt_flags.is_stdin() {
    let cli_options = CliOptions::from_flags(flags)?;
    let fmt_options = cli_options.resolve_fmt_options(fmt_flags)?;
    return format_stdin(
      fmt_options,
      cli_options
        .ext_flag()
        .as_ref()
        .map(|s| s.as_str())
        .unwrap_or("ts"),
    );
  }

  if let Some(watch_flags) = &fmt_flags.watch {
    file_watcher::watch_func(
      flags,
      file_watcher::PrintConfig::new("Fmt", !watch_flags.no_clear_screen),
      move |flags, watcher_communicator, changed_paths| {
        let fmt_flags = fmt_flags.clone();
        Ok(async move {
          let factory = CliFactory::from_flags(flags).await?;
          let cli_options = factory.cli_options();
          let fmt_options = cli_options.resolve_fmt_options(fmt_flags)?;
          let files = collect_fmt_files(fmt_options.files.clone()).and_then(|files| {
            if files.is_empty() {
              Err(generic_error("No target files found."))
            } else {
              Ok(files)
            }
          })?;
          let _ = watcher_communicator.watch_paths(files.clone());
          let refmt_files = if let Some(paths) = changed_paths {
            if fmt_options.check {
              // check all files on any changed (https://github.com/denoland/deno/issues/12446)
              files
                .iter()
                .any(|path| {
                  canonicalize_path(path)
                    .map(|path| paths.contains(&path))
                    .unwrap_or(false)
                })
                .then_some(files)
                .unwrap_or_else(|| [].to_vec())
            } else {
              files
                .into_iter()
                .filter(|path| {
                  canonicalize_path(path)
                    .map(|path| paths.contains(&path))
                    .unwrap_or(false)
                })
                .collect::<Vec<_>>()
            }
          } else {
            files
          };
          format_files(factory, fmt_options, refmt_files).await?;

          Ok(())
        })
      },
    )
    .await?;
  } else {
    let factory = CliFactory::from_flags(flags).await?;
    let cli_options = factory.cli_options();
    let fmt_options = cli_options.resolve_fmt_options(fmt_flags)?;
    let files = collect_fmt_files(fmt_options.files.clone()).and_then(|files| {
      if files.is_empty() {
        Err(generic_error("No target files found."))
      } else {
        Ok(files)
      }
    })?;
    format_files(factory, fmt_options, files).await?;
  }

  Ok(())
}

async fn format_files(
  factory: CliFactory,
  fmt_options: FmtOptions,
  paths: Vec<PathBuf>,
) -> Result<(), AnyError> {
  let caches = factory.caches()?;
  let check = fmt_options.check;
  let incremental_cache = Arc::new(IncrementalCache::new(
    caches.fmt_incremental_cache_db(),
    &fmt_options.options,
    &paths,
  ));
  if check {
    check_source_files(paths, fmt_options.options, incremental_cache.clone()).await?;
  } else {
    format_source_files(paths, fmt_options.options, incremental_cache.clone()).await?;
  }
  incremental_cache.wait_completion().await;
  Ok(())
}

fn collect_fmt_files(files: FilePatterns) -> Result<Vec<PathBuf>, AnyError> {
  FileCollector::new(|e| is_supported_ext_fmt(e.path))
    .ignore_git_folder()
    .ignore_node_modules()
    .ignore_vendor_folder()
    .collect_file_patterns(files)
}

/// Formats markdown (using <https://github.com/dprint/dprint-plugin-markdown>) and its code blocks
/// (ts/tsx, js/jsx).
fn format_markdown(
  file_text: &str,
  fmt_options: &FmtOptionsConfig,
) -> Result<Option<String>, AnyError> {
  let markdown_config = get_resolved_markdown_config(fmt_options);
  dprint_plugin_markdown::format_text(file_text, &markdown_config, move |tag, text, line_width| {
    let tag = tag.to_lowercase();
    if matches!(
      tag.as_str(),
      "ts"
        | "tsx"
        | "js"
        | "jsx"
        | "cjs"
        | "cts"
        | "mjs"
        | "mts"
        | "javascript"
        | "typescript"
        | "json"
        | "jsonc"
    ) {
      // It's important to tell dprint proper file extension, otherwise
      // it might parse the file twice.
      let extension = match tag.as_str() {
        "javascript" => "js",
        "typescript" => "ts",
        rest => rest,
      };

      let fake_filename = PathBuf::from(format!("deno_fmt_stdin.{extension}"));
      if matches!(extension, "json" | "jsonc") {
        let mut json_config = get_resolved_json_config(fmt_options);
        json_config.line_width = line_width;
        dprint_plugin_json::format_text(&fake_filename, text, &json_config)
      } else {
        let mut codeblock_config = get_resolved_typescript_config(fmt_options);
        codeblock_config.line_width = line_width;
        dprint_plugin_typescript::format_text(&fake_filename, text, &codeblock_config)
      }
    } else {
      Ok(None)
    }
  })
}

/// Formats JSON and JSONC using the rules provided by .deno()
/// of configuration builder of <https://github.com/dprint/dprint-plugin-json>.
/// See <https://github.com/dprint/dprint-plugin-json/blob/cfa1052dbfa0b54eb3d814318034cdc514c813d7/src/configuration/builder.rs#L87> for configuration.
pub fn format_json(
  file_path: &Path,
  file_text: &str,
  fmt_options: &FmtOptionsConfig,
) -> Result<Option<String>, AnyError> {
  let config = get_resolved_json_config(fmt_options);
  dprint_plugin_json::format_text(file_path, file_text, &config)
}

/// Formats a single TS, TSX, JS, JSX, JSONC, JSON, MD, or IPYNB file.
pub fn format_file(
  file_path: &Path,
  file_text: &str,
  fmt_options: &FmtOptionsConfig,
) -> Result<Option<String>, AnyError> {
  let ext = get_extension(file_path).unwrap_or_default();

  match ext.as_str() {
    "md" | "mkd" | "mkdn" | "mdwn" | "mdown" | "markdown" => {
      format_markdown(file_text, fmt_options)
    }
    "json" | "jsonc" => format_json(file_path, file_text, fmt_options),
    "ipynb" => {
      dprint_plugin_jupyter::format_text(file_text, |file_path: &Path, file_text: String| {
        format_file(file_path, &file_text, fmt_options)
      })
    }
    _ => {
      let config = get_resolved_typescript_config(fmt_options);
      dprint_plugin_typescript::format_text(file_path, file_text, &config)
    }
  }
}

pub fn format_parsed_source(
  parsed_source: &ParsedSource,
  fmt_options: &FmtOptionsConfig,
) -> Result<Option<String>, AnyError> {
  dprint_plugin_typescript::format_parsed_source(
    parsed_source,
    &get_resolved_typescript_config(fmt_options),
  )
}

async fn check_source_files(
  paths: Vec<PathBuf>,
  fmt_options: FmtOptionsConfig,
  incremental_cache: Arc<IncrementalCache>,
) -> Result<(), AnyError> {
  let not_formatted_files_count = Arc::new(AtomicUsize::new(0));
  let checked_files_count = Arc::new(AtomicUsize::new(0));

  // prevent threads outputting at the same time
  let output_lock = Arc::new(Mutex::new(0));

  run_parallelized(paths, {
    let not_formatted_files_count = not_formatted_files_count.clone();
    let checked_files_count = checked_files_count.clone();
    move |file_path| {
      checked_files_count.fetch_add(1, Ordering::Relaxed);
      let file_text = read_file_contents(&file_path)?.text;

      // skip checking the file if we know it's formatted
      if incremental_cache.is_file_same(&file_path, &file_text) {
        return Ok(());
      }

      match format_file(&file_path, &file_text, &fmt_options) {
        Ok(Some(formatted_text)) => {
          not_formatted_files_count.fetch_add(1, Ordering::Relaxed);
          let _g = output_lock.lock();
          let diff = diff(&file_text, &formatted_text);
          info!("");
          info!("{} {}:", colors::bold("from"), file_path.display());
          info!("{}", diff);
        }
        Ok(None) => {
          // When checking formatting, only update the incremental cache when
          // the file is the same since we don't bother checking for stable
          // formatting here. Additionally, ensure this is done during check
          // so that CIs that cache the DENO_DIR will get the benefit of
          // incremental formatting
          incremental_cache.update_file(&file_path, &file_text);
        }
        Err(e) => {
          not_formatted_files_count.fetch_add(1, Ordering::Relaxed);
          let _g = output_lock.lock();
          warn!("Error checking: {}", file_path.to_string_lossy());
          warn!(
            "{}",
            format!("{e}")
              .split('\n')
              .map(|l| {
                if l.trim().is_empty() {
                  String::new()
                } else {
                  format!("  {l}")
                }
              })
              .collect::<Vec<_>>()
              .join("\n")
          );
        }
      }
      Ok(())
    }
  })
  .await?;

  let not_formatted_files_count = not_formatted_files_count.load(Ordering::Relaxed);
  let checked_files_count = checked_files_count.load(Ordering::Relaxed);
  let checked_files_str = format!("{} {}", checked_files_count, files_str(checked_files_count));
  if not_formatted_files_count == 0 {
    info!("Checked {}", checked_files_str);
    Ok(())
  } else {
    let not_formatted_files_str = files_str(not_formatted_files_count);
    Err(generic_error(format!(
      "Found {not_formatted_files_count} not formatted {not_formatted_files_str} in {checked_files_str}",
    )))
  }
}

async fn format_source_files(
  paths: Vec<PathBuf>,
  fmt_options: FmtOptionsConfig,
  incremental_cache: Arc<IncrementalCache>,
) -> Result<(), AnyError> {
  let formatted_files_count = Arc::new(AtomicUsize::new(0));
  let checked_files_count = Arc::new(AtomicUsize::new(0));
  let output_lock = Arc::new(Mutex::new(0)); // prevent threads outputting at the same time

  run_parallelized(paths, {
    let formatted_files_count = formatted_files_count.clone();
    let checked_files_count = checked_files_count.clone();
    move |file_path| {
      checked_files_count.fetch_add(1, Ordering::Relaxed);
      let file_contents = read_file_contents(&file_path)?;

      // skip formatting the file if we know it's formatted
      if incremental_cache.is_file_same(&file_path, &file_contents.text) {
        return Ok(());
      }

      match format_ensure_stable(&file_path, &file_contents.text, &fmt_options, format_file) {
        Ok(Some(formatted_text)) => {
          incremental_cache.update_file(&file_path, &formatted_text);
          write_file_contents(
            &file_path,
            FileContents {
              had_bom: file_contents.had_bom,
              text: formatted_text,
            },
          )?;
          formatted_files_count.fetch_add(1, Ordering::Relaxed);
          let _g = output_lock.lock();
          info!("{}", file_path.to_string_lossy());
        }
        Ok(None) => {
          incremental_cache.update_file(&file_path, &file_contents.text);
        }
        Err(e) => {
          let _g = output_lock.lock();
          eprintln!("Error formatting: {}", file_path.to_string_lossy());
          eprintln!("   {e}");
        }
      }
      Ok(())
    }
  })
  .await?;

  let formatted_files_count = formatted_files_count.load(Ordering::Relaxed);
  debug!(
    "Formatted {} {}",
    formatted_files_count,
    files_str(formatted_files_count),
  );

  let checked_files_count = checked_files_count.load(Ordering::Relaxed);
  info!(
    "Checked {} {}",
    checked_files_count,
    files_str(checked_files_count)
  );

  Ok(())
}

/// When storing any formatted text in the incremental cache, we want
/// to ensure that anything stored when formatted will have itself as
/// the output as well. This is to prevent "double format" issues where
/// a user formats their code locally and it fails on the CI afterwards.
fn format_ensure_stable(
  file_path: &Path,
  file_text: &str,
  fmt_options: &FmtOptionsConfig,
  fmt_func: impl Fn(&Path, &str, &FmtOptionsConfig) -> Result<Option<String>, AnyError>,
) -> Result<Option<String>, AnyError> {
  let formatted_text = fmt_func(file_path, file_text, fmt_options)?;

  match formatted_text {
    Some(mut current_text) => {
      let mut count = 0;
      loop {
        match fmt_func(file_path, &current_text, fmt_options) {
          Ok(Some(next_pass_text)) => {
            // just in case
            if next_pass_text == current_text {
              return Ok(Some(next_pass_text));
            }
            current_text = next_pass_text;
          }
          Ok(None) => {
            return Ok(Some(current_text));
          }
          Err(err) => {
            panic!(
              concat!(
                "Formatting succeeded initially, but failed when ensuring a ",
                "stable format. This indicates a bug in the formatter where ",
                "the text it produces is not syntactically correct. As a temporary ",
                "workaround you can ignore this file ({}).\n\n{:#}"
              ),
              file_path.display(),
              err,
            )
          }
        }
        count += 1;
        if count == 5 {
          panic!(
            concat!(
              "Formatting not stable. Bailed after {} tries. This indicates a bug ",
              "in the formatter where it formats the file ({}) differently each time. As a ",
              "temporary workaround you can ignore this file."
            ),
            count,
            file_path.display(),
          )
        }
      }
    }
    None => Ok(None),
  }
}

/// Format stdin and write result to stdout.
/// Treats input as set by `--ext` flag.
/// Compatible with `--check` flag.
fn format_stdin(
  fmt_options: FmtOptions,
  ext: &str,
) -> Result<(), AnyError> {
  let mut source = String::new();
  if stdin().read_to_string(&mut source).is_err() {
    bail!("Failed to read from stdin");
  }
  let file_path = PathBuf::from(format!("_stdin.{ext}"));
  let formatted_text = format_file(&file_path, &source, &fmt_options.options)?;
  if fmt_options.check {
    if formatted_text.is_some() {
      println!("Not formatted stdin");
    }
  } else {
    stdout().write_all(formatted_text.unwrap_or(source).as_bytes())?;
  }
  Ok(())
}

fn files_str(len: usize) -> &'static str {
  if len <= 1 {
    "file"
  } else {
    "files"
  }
}

fn get_resolved_typescript_config(
  options: &FmtOptionsConfig
) -> dprint_plugin_typescript::configuration::Configuration {
  let mut builder = dprint_plugin_typescript::configuration::ConfigurationBuilder::new();
  builder.deno();

  if let Some(use_tabs) = options.use_tabs {
    builder.use_tabs(use_tabs);
  }

  if let Some(line_width) = options.line_width {
    builder.line_width(line_width);
  }

  if let Some(indent_width) = options.indent_width {
    builder.indent_width(indent_width);
  }

  if let Some(single_quote) = options.single_quote {
    if single_quote {
      builder.quote_style(dprint_plugin_typescript::configuration::QuoteStyle::PreferSingle);
    }
  }

  if let Some(semi_colons) = options.semi_colons {
    builder.semi_colons(match semi_colons {
      true => dprint_plugin_typescript::configuration::SemiColons::Prefer,
      false => dprint_plugin_typescript::configuration::SemiColons::Asi,
    });
  }

  builder.build()
}

fn get_resolved_markdown_config(
  options: &FmtOptionsConfig
) -> dprint_plugin_markdown::configuration::Configuration {
  let mut builder = dprint_plugin_markdown::configuration::ConfigurationBuilder::new();

  builder.deno();

  if let Some(line_width) = options.line_width {
    builder.line_width(line_width);
  }

  if let Some(prose_wrap) = options.prose_wrap {
    builder.text_wrap(match prose_wrap {
      ProseWrap::Always => dprint_plugin_markdown::configuration::TextWrap::Always,
      ProseWrap::Never => dprint_plugin_markdown::configuration::TextWrap::Never,
      ProseWrap::Preserve => dprint_plugin_markdown::configuration::TextWrap::Maintain,
    });
  }

  builder.build()
}

fn get_resolved_json_config(
  options: &FmtOptionsConfig
) -> dprint_plugin_json::configuration::Configuration {
  let mut builder = dprint_plugin_json::configuration::ConfigurationBuilder::new();

  builder.deno();

  if let Some(use_tabs) = options.use_tabs {
    builder.use_tabs(use_tabs);
  }

  if let Some(line_width) = options.line_width {
    builder.line_width(line_width);
  }

  if let Some(indent_width) = options.indent_width {
    builder.indent_width(indent_width);
  }

  builder.build()
}

struct FileContents {
  text: String,
  had_bom: bool,
}

fn read_file_contents(file_path: &Path) -> Result<FileContents, AnyError> {
  let file_bytes =
    fs::read(file_path).with_context(|| format!("Error reading {}", file_path.display()))?;
  let had_bom = file_bytes.starts_with(&[0xEF, 0xBB, 0xBF]);
  // will have the BOM stripped
  let text = deno_graph::source::decode_owned_file_source(file_bytes)
    .with_context(|| anyhow!("{} is not a valid UTF-8 file", file_path.display()))?;

  Ok(FileContents { text, had_bom })
}

fn write_file_contents(
  file_path: &Path,
  mut file_contents: FileContents,
) -> Result<(), AnyError> {
  let file_text = if file_contents.had_bom {
    // add back the BOM
    file_contents.text.insert(0, '\u{FEFF}');
    file_contents.text
  } else {
    file_contents.text
  };

  Ok(fs::write(file_path, file_text)?)
}

pub async fn run_parallelized<F>(
  file_paths: Vec<PathBuf>,
  f: F,
) -> Result<(), AnyError>
where
  F: FnOnce(PathBuf) -> Result<(), AnyError> + Send + 'static + Clone,
{
  let handles = file_paths.iter().map(|file_path| {
    let f = f.clone();
    let file_path = file_path.clone();
    spawn_blocking(move || f(file_path))
  });
  let join_results = futures::future::join_all(handles).await;

  // find the tasks that panicked and let the user know which files
  let panic_file_paths = join_results
    .iter()
    .enumerate()
    .filter_map(|(i, join_result)| {
      join_result
        .as_ref()
        .err()
        .map(|_| file_paths[i].to_string_lossy())
    })
    .collect::<Vec<_>>();
  if !panic_file_paths.is_empty() {
    panic!("Panic formatting: {}", panic_file_paths.join(", "))
  }

  // check for any errors and if so return the first one
  let mut errors = join_results.into_iter().filter_map(|join_result| {
    join_result
      .ok()
      .and_then(|handle_result| handle_result.err())
  });

  if let Some(e) = errors.next() {
    Err(e)
  } else {
    Ok(())
  }
}

/// This function is similar to is_supported_ext but adds additional extensions
/// supported by `deno fmt`.
fn is_supported_ext_fmt(path: &Path) -> bool {
  get_extension(path).is_some_and(|ext| {
    matches!(
      ext.as_str(),
      "ts"
        | "tsx"
        | "js"
        | "jsx"
        | "cjs"
        | "cts"
        | "mjs"
        | "mts"
        | "json"
        | "jsonc"
        | "md"
        | "mkd"
        | "mkdn"
        | "mdwn"
        | "mdown"
        | "markdown"
        | "ipynb"
    )
  })
}
