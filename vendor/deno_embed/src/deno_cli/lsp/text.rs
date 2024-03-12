// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

use deno_core::error::custom_error;
use deno_core::error::AnyError;
use dissimilar::diff;
use dissimilar::Chunk;
use std::collections::HashMap;
use text_size::TextRange;
use text_size::TextSize;
use tower_lsp::jsonrpc;
use tower_lsp::lsp_types as lsp;
use tower_lsp::lsp_types::TextEdit;

fn partition_point<T, P>(slice: &[T], mut predicate: P) -> usize
where
  P: FnMut(&T) -> bool,
{
  let mut left = 0;
  let mut right = slice.len() - 1;

  while left != right {
    let mid = left + (right - left) / 2;
    // SAFETY:
    // When left < right, left <= mid < right.
    // Therefore left always increases and right always decreases,
    // and either of them is selected.
    // In both cases left <= right is satisfied.
    // Therefore if left < right in a step,
    // left <= right is satisfied in the next step.
    // Therefore as long as left != right, 0 <= left < right < len is satisfied
    // and if this case 0 <= mid < len is satisfied too.
    let value = unsafe { slice.get_unchecked(mid) };
    if predicate(value) {
      left = mid + 1;
    } else {
      right = mid;
    }
  }

  left
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Utf16Char {
  pub start: TextSize,
  pub end: TextSize,
}

impl Utf16Char {
  fn len(&self) -> TextSize {
    self.end - self.start
  }

  fn len_utf16(&self) -> usize {
    if self.len() == TextSize::from(4) {
      2
    } else {
      1
    }
  }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct LineIndex {
  utf8_offsets: Vec<TextSize>,
  utf16_lines: HashMap<u32, Vec<Utf16Char>>,
  utf16_offsets: Vec<TextSize>,
}

impl LineIndex {
  pub fn new(text: &str) -> LineIndex {
    let mut utf16_lines = HashMap::new();
    let mut utf16_chars = Vec::new();

    let mut utf8_offsets = vec![0.into()];
    let mut utf16_offsets = vec![0.into()];
    let mut curr_row = 0.into();
    let mut curr_col = 0.into();
    let mut curr_offset_u16 = 0.into();
    let mut line = 0;
    for c in text.chars() {
      let c_len = TextSize::of(c);
      curr_row += c_len;
      curr_offset_u16 += TextSize::from(c.len_utf16() as u32);
      if c == '\n' {
        utf8_offsets.push(curr_row);
        utf16_offsets.push(curr_offset_u16);

        if !utf16_chars.is_empty() {
          utf16_lines.insert(line, utf16_chars);
          utf16_chars = Vec::new();
        }

        curr_col = 0.into();
        line += 1;
        continue;
      }

      if !c.is_ascii() {
        utf16_chars.push(Utf16Char {
          start: curr_col,
          end: curr_col + c_len,
        });
      }
      curr_col += c_len;
    }

    // utf8_offsets and utf16_offsets length is equal to (# of lines + 1)
    utf8_offsets.push(curr_row);
    utf16_offsets.push(curr_offset_u16);

    if !utf16_chars.is_empty() {
      utf16_lines.insert(line, utf16_chars);
    }

    LineIndex {
      utf8_offsets,
      utf16_lines,
      utf16_offsets,
    }
  }

  /// Convert a u16 based range to a u8 TextRange.
  pub fn get_text_range(
    &self,
    range: lsp::Range,
  ) -> Result<TextRange, AnyError> {
    let start = self.offset(range.start)?;
    let end = self.offset(range.end)?;
    Ok(TextRange::new(start, end))
  }

  /// Return a u8 offset based on a u16 position.
  pub fn offset(&self, position: lsp::Position) -> Result<TextSize, AnyError> {
    let col = self.utf16_to_utf8_col(position.line, position.character);
    if let Some(line_offset) = self.utf8_offsets.get(position.line as usize) {
      Ok(line_offset + col)
    } else {
      Err(custom_error("OutOfRange", "The position is out of range."))
    }
  }

  /// Convert an lsp Position into a tsc/TypeScript "position", which is really
  /// an u16 byte offset from the start of the string represented as an u32.
  pub fn offset_tsc(&self, position: lsp::Position) -> jsonrpc::Result<u32> {
    self
      .offset_utf16(position)
      .map(|ts| ts.into())
      .map_err(|err| jsonrpc::Error::invalid_params(err.to_string()))
  }

  fn offset_utf16(
    &self,
    position: lsp::Position,
  ) -> Result<TextSize, AnyError> {
    if let Some(line_offset) = self.utf16_offsets.get(position.line as usize) {
      Ok(line_offset + TextSize::from(position.character))
    } else {
      Err(custom_error("OutOfRange", "The position is out of range."))
    }
  }

  /// Returns a u16 position based on a u16 offset, which TypeScript offsets are
  /// returned as u16.
  pub fn position_tsc(&self, offset: TextSize) -> lsp::Position {
    let line = partition_point(&self.utf16_offsets, |&it| it <= offset) - 1;
    let line_start_offset = self.utf16_offsets[line];
    let col = offset - line_start_offset;

    lsp::Position {
      line: line as u32,
      character: col.into(),
    }
  }

  /// Returns a u16 position based on a u8 offset.
  pub fn position_utf16(&self, offset: TextSize) -> lsp::Position {
    let line = partition_point(&self.utf16_offsets, |&it| it <= offset) - 1;
    let line_start_offset = self.utf16_offsets[line];
    let col = offset - line_start_offset;

    lsp::Position {
      line: line as u32,
      character: col.into(),
    }
  }

  pub fn text_content_length_utf16(&self) -> TextSize {
    *self.utf16_offsets.last().unwrap()
  }

  fn utf16_to_utf8_col(&self, line: u32, mut col: u32) -> TextSize {
    if let Some(utf16_chars) = self.utf16_lines.get(&line) {
      for c in utf16_chars {
        if col > u32::from(c.start) {
          col += u32::from(c.len()) - c.len_utf16() as u32;
        } else {
          break;
        }
      }
    }

    col.into()
  }
}

/// Compare two strings and return a vector of text edit records which are
/// supported by the Language Server Protocol.
pub fn get_edits(a: &str, b: &str, line_index: &LineIndex) -> Vec<TextEdit> {
  if a == b {
    return vec![];
  }
  // Heuristic to detect things like minified files. `diff()` is expensive.
  if b.chars().filter(|c| *c == '\n').count()
    > line_index.utf8_offsets.len() * 3
  {
    return vec![TextEdit {
      range: lsp::Range {
        start: lsp::Position::new(0, 0),
        end: line_index.position_utf16(TextSize::from(a.len() as u32)),
      },
      new_text: b.to_string(),
    }];
  }
  let chunks = diff(a, b);
  let mut text_edits = Vec::<TextEdit>::new();
  let mut iter = chunks.iter().peekable();
  let mut a_pos = TextSize::from(0);
  loop {
    let chunk = iter.next();
    match chunk {
      None => break,
      Some(Chunk::Equal(e)) => {
        a_pos += TextSize::from(e.encode_utf16().count() as u32);
      }
      Some(Chunk::Delete(d)) => {
        let start = line_index.position_utf16(a_pos);
        a_pos += TextSize::from(d.encode_utf16().count() as u32);
        let end = line_index.position_utf16(a_pos);
        let range = lsp::Range { start, end };
        match iter.peek() {
          Some(Chunk::Insert(i)) => {
            iter.next();
            text_edits.push(TextEdit {
              range,
              new_text: i.to_string(),
            });
          }
          _ => text_edits.push(TextEdit {
            range,
            new_text: "".to_string(),
          }),
        }
      }
      Some(Chunk::Insert(i)) => {
        let pos = line_index.position_utf16(a_pos);
        let range = lsp::Range {
          start: pos,
          end: pos,
        };
        text_edits.push(TextEdit {
          range,
          new_text: i.to_string(),
        });
      }
    }
  }

  text_edits
}

