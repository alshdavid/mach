// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

use deno_ast::ModuleSpecifier;
use deno_graph::ModuleGraph;
use deno_terminal::colors;

use deno_core::serde::Deserialize;
use deno_core::serde::Deserializer;
use deno_core::serde::Serialize;
use deno_core::serde::Serializer;
use deno_core::sourcemap::SourceMap;
use std::error::Error;
use std::fmt;

const MAX_SOURCE_LINE_LENGTH: usize = 150;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DiagnosticCategory {
  Warning,
  Error,
  Suggestion,
  Message,
}

impl fmt::Display for DiagnosticCategory {
  fn fmt(
    &self,
    f: &mut fmt::Formatter,
  ) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        DiagnosticCategory::Warning => "WARN ",
        DiagnosticCategory::Error => "ERROR ",
        DiagnosticCategory::Suggestion => "",
        DiagnosticCategory::Message => "",
      }
    )
  }
}

impl<'de> Deserialize<'de> for DiagnosticCategory {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s: i64 = Deserialize::deserialize(deserializer)?;
    Ok(DiagnosticCategory::from(s))
  }
}

impl Serialize for DiagnosticCategory {
  fn serialize<S>(
    &self,
    serializer: S,
  ) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let value = match self {
      DiagnosticCategory::Warning => 0_i32,
      DiagnosticCategory::Error => 1_i32,
      DiagnosticCategory::Suggestion => 2_i32,
      DiagnosticCategory::Message => 3_i32,
    };
    Serialize::serialize(&value, serializer)
  }
}

impl From<i64> for DiagnosticCategory {
  fn from(value: i64) -> Self {
    match value {
      0 => DiagnosticCategory::Warning,
      1 => DiagnosticCategory::Error,
      2 => DiagnosticCategory::Suggestion,
      3 => DiagnosticCategory::Message,
      _ => panic!("Unknown value: {value}"),
    }
  }
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticMessageChain {
  message_text: String,
  category: DiagnosticCategory,
  code: i64,
  next: Option<Vec<DiagnosticMessageChain>>,
}

impl DiagnosticMessageChain {
  pub fn format_message(
    &self,
    level: usize,
  ) -> String {
    let mut s = String::new();

    s.push_str(&" ".repeat(level * 2));
    s.push_str(&self.message_text);
    if let Some(next) = &self.next {
      s.push('\n');
      let arr = next.clone();
      for dm in arr {
        s.push_str(&dm.format_message(level + 1));
      }
    }

    s
  }
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Position {
  /// 0-indexed line number
  pub line: u64,
  /// 0-indexed character number
  pub character: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Diagnostic {
  pub category: DiagnosticCategory,
  pub code: u64,
  pub start: Option<Position>,
  pub end: Option<Position>,
  /// Position of this diagnostic in the original non-mapped source.
  ///
  /// This will exist and be different from the `start` for fast
  /// checked modules where the TypeScript source will differ
  /// from the original source.
  #[serde(skip_serializing)]
  pub original_source_start: Option<Position>,
  pub message_text: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub message_chain: Option<DiagnosticMessageChain>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub source: Option<String>,
  pub source_line: Option<String>,
  pub file_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub related_information: Option<Vec<Diagnostic>>,
}

impl Diagnostic {
  fn fmt_category_and_code(
    &self,
    f: &mut fmt::Formatter,
  ) -> fmt::Result {
    let category = match self.category {
      DiagnosticCategory::Error => "ERROR",
      DiagnosticCategory::Warning => "WARN",
      _ => "",
    };

    let code = if self.code >= 900001 {
      "".to_string()
    } else {
      colors::bold(format!("TS{} ", self.code)).to_string()
    };

    if !category.is_empty() {
      write!(f, "{code}[{category}]: ")
    } else {
      Ok(())
    }
  }

  fn fmt_frame(
    &self,
    f: &mut fmt::Formatter,
    level: usize,
  ) -> fmt::Result {
    if let (Some(file_name), Some(start)) = (
      self.file_name.as_ref(),
      self.original_source_start.as_ref().or(self.start.as_ref()),
    ) {
      write!(
        f,
        "\n{:indent$}    at {}:{}:{}",
        "",
        colors::cyan(file_name),
        colors::yellow(&(start.line + 1).to_string()),
        colors::yellow(&(start.character + 1).to_string()),
        indent = level
      )
    } else {
      Ok(())
    }
  }

  fn fmt_message(
    &self,
    f: &mut fmt::Formatter,
    level: usize,
  ) -> fmt::Result {
    if let Some(message_chain) = &self.message_chain {
      write!(f, "{}", message_chain.format_message(level))
    } else {
      write!(
        f,
        "{:indent$}{}",
        "",
        self.message_text.as_deref().unwrap_or_default(),
        indent = level,
      )
    }
  }

  fn fmt_source_line(
    &self,
    f: &mut fmt::Formatter,
    level: usize,
  ) -> fmt::Result {
    if let (Some(source_line), Some(start), Some(end)) = (&self.source_line, &self.start, &self.end)
    {
      if !source_line.is_empty() && source_line.len() <= MAX_SOURCE_LINE_LENGTH {
        write!(f, "\n{:indent$}{}", "", source_line, indent = level)?;
        let length = if start.line == end.line {
          end.character - start.character
        } else {
          1
        };
        let mut s = String::new();
        for i in 0..start.character {
          s.push(if source_line.chars().nth(i as usize).unwrap() == '\t' {
            '\t'
          } else {
            ' '
          });
        }
        // TypeScript always uses `~` when underlining, but v8 always uses `^`.
        // We will use `^` to indicate a single point, or `~` when spanning
        // multiple characters.
        let ch = if length > 1 { '~' } else { '^' };
        for _i in 0..length {
          s.push(ch)
        }
        let underline = if self.is_error() {
          colors::red(&s).to_string()
        } else {
          colors::cyan(&s).to_string()
        };
        write!(f, "\n{:indent$}{}", "", underline, indent = level)?;
      }
    }

    Ok(())
  }

  fn fmt_related_information(
    &self,
    f: &mut fmt::Formatter,
  ) -> fmt::Result {
    if let Some(related_information) = self.related_information.as_ref() {
      if !related_information.is_empty() {
        write!(f, "\n\n")?;
        for info in related_information {
          info.fmt_stack(f, 4)?;
        }
      }
    }

    Ok(())
  }

  fn fmt_stack(
    &self,
    f: &mut fmt::Formatter,
    level: usize,
  ) -> fmt::Result {
    self.fmt_category_and_code(f)?;
    self.fmt_message(f, level)?;
    self.fmt_source_line(f, level)?;
    self.fmt_frame(f, level)
  }

  fn is_error(&self) -> bool {
    self.category == DiagnosticCategory::Error
  }
}

impl fmt::Display for Diagnostic {
  fn fmt(
    &self,
    f: &mut fmt::Formatter,
  ) -> fmt::Result {
    self.fmt_stack(f, 0)?;
    self.fmt_related_information(f)
  }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Diagnostics(Vec<Diagnostic>);

impl Diagnostics {
  #[cfg(test)]
  pub fn new(diagnostics: Vec<Diagnostic>) -> Self {
    Diagnostics(diagnostics)
  }

  /// Return a set of diagnostics where only the values where the predicate
  /// returns `true` are included.
  pub fn filter<P>(
    &self,
    predicate: P,
  ) -> Self
  where
    P: FnMut(&Diagnostic) -> Option<Diagnostic>,
  {
    let diagnostics = self.0.iter().filter_map(predicate).collect();
    Self(diagnostics)
  }

  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  /// Modifies all the diagnostics to have their display positions
  /// modified to point at the original source.
  pub fn apply_fast_check_source_maps(
    &mut self,
    graph: &ModuleGraph,
  ) {
    fn visit_diagnostic(
      d: &mut Diagnostic,
      graph: &ModuleGraph,
    ) {
      if let Some(specifier) = d
        .file_name
        .as_ref()
        .and_then(|n| ModuleSpecifier::parse(n).ok())
      {
        if let Ok(Some(module)) = graph.try_get_prefer_types(&specifier) {
          if let Some(fast_check_module) = module.js().and_then(|m| m.fast_check_module()) {
            // todo(dsherret): use a short lived cache to prevent parsing
            // source maps so often
            if let Ok(source_map) = SourceMap::from_slice(&fast_check_module.source_map) {
              if let Some(start) = d.start.as_mut() {
                let maybe_token =
                  source_map.lookup_token(start.line as u32, start.character as u32);
                if let Some(token) = maybe_token {
                  d.original_source_start = Some(Position {
                    line: token.get_src_line() as u64,
                    character: token.get_src_col() as u64,
                  });
                }
              }
            }
          }
        }
      }

      if let Some(related) = &mut d.related_information {
        for d in related.iter_mut() {
          visit_diagnostic(d, graph);
        }
      }
    }

    for d in &mut self.0 {
      visit_diagnostic(d, graph);
    }
  }
}

impl<'de> Deserialize<'de> for Diagnostics {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let items: Vec<Diagnostic> = Deserialize::deserialize(deserializer)?;
    Ok(Diagnostics(items))
  }
}

impl Serialize for Diagnostics {
  fn serialize<S>(
    &self,
    serializer: S,
  ) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    Serialize::serialize(&self.0, serializer)
  }
}

impl fmt::Display for Diagnostics {
  fn fmt(
    &self,
    f: &mut fmt::Formatter,
  ) -> fmt::Result {
    let mut i = 0;
    for item in &self.0 {
      if i > 0 {
        write!(f, "\n\n")?;
      }
      write!(f, "{item}")?;
      i += 1;
    }

    if i > 1 {
      write!(f, "\n\nFound {i} errors.")?;
    }

    Ok(())
  }
}

impl Error for Diagnostics {}
