// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

use std::time::Duration;

use deno_terminal::colors;

use crate::deno_cli::util::display::human_download_size;

use super::ProgressMessagePrompt;

#[derive(Clone)]
pub struct ProgressDataDisplayEntry {
  pub prompt: ProgressMessagePrompt,
  pub message: String,
  pub position: u64,
  pub total_size: u64,
}

#[derive(Clone)]
pub struct ProgressData {
  pub terminal_width: u32,
  pub display_entry: ProgressDataDisplayEntry,
  pub pending_entries: usize,
  pub percent_done: f64,
  pub total_entries: usize,
  pub duration: Duration,
}

pub trait ProgressBarRenderer: Send + Sync + std::fmt::Debug {
  fn render(&self, data: ProgressData) -> String;
}

/// Indicatif style progress bar.
#[derive(Debug)]
pub struct BarProgressBarRenderer;

impl ProgressBarRenderer for BarProgressBarRenderer {
  fn render(&self, data: ProgressData) -> String {
    let (bytes_text, bytes_text_max_width) = {
      let total_size = data.display_entry.total_size;
      let pos = data.display_entry.position;
      if total_size == 0 {
        (String::new(), 0)
      } else {
        let total_size_str = human_download_size(total_size, total_size);
        (
          format!(
            " {}/{}",
            human_download_size(pos, total_size),
            total_size_str,
          ),
          2 + total_size_str.len() * 2,
        )
      }
    };
    let (total_text, total_text_max_width) = if data.total_entries <= 1 {
      (String::new(), 0)
    } else {
      let total_entries_str = data.total_entries.to_string();
      (
        format!(
          " ({}/{})",
          data.total_entries - data.pending_entries,
          data.total_entries
        ),
        4 + total_entries_str.len() * 2,
      )
    };

    let elapsed_text = get_elapsed_text(data.duration);
    let mut text = String::new();
    if !data.display_entry.message.is_empty() {
      text.push_str(&format!(
        "{} {}{}\n",
        colors::green("Download"),
        data.display_entry.message,
        bytes_text,
      ));
    }
    text.push_str(&elapsed_text);
    let max_width = (data.terminal_width as i32 - 5).clamp(10, 75) as usize;
    let same_line_text_width =
      elapsed_text.len() + total_text_max_width + bytes_text_max_width + 3; // space, open and close brace
    let total_bars = if same_line_text_width > max_width {
      1
    } else {
      max_width - same_line_text_width
    };
    let completed_bars =
      (total_bars as f64 * data.percent_done).floor() as usize;
    text.push_str(" [");
    if completed_bars != total_bars {
      if completed_bars > 0 {
        text.push_str(&format!(
          "{}",
          colors::cyan(format!("{}{}", "#".repeat(completed_bars - 1), ">"))
        ))
      }
      text.push_str(&format!(
        "{}",
        colors::intense_blue("-".repeat(total_bars - completed_bars))
      ))
    } else {
      text.push_str(&format!("{}", colors::cyan("#".repeat(completed_bars))))
    }
    text.push(']');

    // suffix
    if data.display_entry.message.is_empty() {
      text.push_str(&colors::gray(bytes_text).to_string());
    }
    text.push_str(&colors::gray(total_text).to_string());

    text
  }
}

#[derive(Debug)]
pub struct TextOnlyProgressBarRenderer;

impl ProgressBarRenderer for TextOnlyProgressBarRenderer {
  fn render(&self, data: ProgressData) -> String {
    let bytes_text = {
      let total_size = data.display_entry.total_size;
      let pos = data.display_entry.position;
      if total_size == 0 {
        String::new()
      } else {
        format!(
          " {}/{}",
          human_download_size(pos, total_size),
          human_download_size(total_size, total_size)
        )
      }
    };
    let total_text = if data.total_entries <= 1 {
      String::new()
    } else {
      format!(
        " ({}/{})",
        data.total_entries - data.pending_entries,
        data.total_entries
      )
    };

    format!(
      "{} {}{}{}",
      data.display_entry.prompt.as_text(),
      data.display_entry.message,
      colors::gray(bytes_text),
      colors::gray(total_text),
    )
  }
}

fn get_elapsed_text(elapsed: Duration) -> String {
  let elapsed_secs = elapsed.as_secs();
  let seconds = elapsed_secs % 60;
  let minutes = elapsed_secs / 60;
  format!("[{minutes:0>2}:{seconds:0>2}]")
}
