#![allow(unused_imports)]
#![allow(dead_code)]

use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::time::SystemTime;

use dashmap::mapref::one::Ref;
use dashmap::DashMap;
use normalize_path::NormalizePath;
use once_cell::sync::Lazy;
use serde::Serialize;

pub static PROFILER: Lazy<Profiler> = Lazy::new(|| Profiler::new());

#[derive(Default, Clone)]
pub struct Profiler {
  start_times: Arc<DashMap<String, SystemTime>>,
  end_times: Arc<DashMap<String, Vec<Duration>>>,
  log_file: Arc<Mutex<Option<File>>>,
  logs: Vec<String>,
}

impl Profiler {
  pub fn new() -> Self {
    let mut profiler = Self::default();
    if let Ok(value) = std::env::var("mach_profiler") {
      let mut path = PathBuf::from(value);
      if path.is_relative() {
        path = std::env::current_dir().unwrap().join(path).normalize();
      }
      println!("profiler writing to: {:?}", path);
      let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)
        .unwrap();

      profiler.log_file = Arc::new(Mutex::new(Some(file)));
    }
    return profiler;
  }

  pub fn start(
    &self,
    name: &str,
  ) {
    let start_time = SystemTime::now();
    self.start_times.insert(name.to_string(), start_time);
  }

  pub fn lap(
    &self,
    name: &str,
  ) {
    if !self.start_times.contains_key(name) {
      self.start(name);
    }
    let end_time = self
      .start_times
      .get(name)
      .unwrap()
      .elapsed()
      .expect(&format!("could not find performance mark \"{}\"", name));
    if let Some(mut end_times) = self.end_times.get_mut(name) {
      end_times.push(end_time);
    } else {
      self.end_times.insert(name.to_string(), vec![end_time]);
    }
  }

  pub fn get_nanos_average(
    &self,
    name: &str,
  ) -> u128 {
    let end_times = self.get_end_times(name).clone();
    let mut total: u128 = 0;
    for end_time in end_times.iter() {
      total += end_time.as_nanos();
    }
    return total / end_times.len() as u128;
  }

  pub fn get_nanos_median(
    &self,
    name: &str,
  ) -> u128 {
    if !self.end_times.contains_key(name) {
      self.lap(name);
    }
    let end_times = self.get_end_times(name).clone();
    let mut end_times = end_times
      .iter()
      .map(|d| d.as_nanos())
      .collect::<Vec<u128>>();
    end_times.sort();
    let mid = end_times.len() / 2;
    end_times[mid]
  }

  pub fn get_nanos_total(
    &self,
    name: &str,
  ) -> u128 {
    if !self.end_times.contains_key(name) {
      self.lap(name);
    }
    let end_times = self.get_end_times(name);
    let mut total: u128 = 0;
    for end_time in end_times.iter() {
      total += end_time.as_nanos();
    }
    total
  }

  pub fn get_nanos_max(
    &self,
    name: &str,
  ) -> u128 {
    if !self.end_times.contains_key(name) {
      self.lap(name);
    }
    let end_times = self.get_end_times(name);
    let mut max: u128 = 0;
    for end_time in end_times.iter() {
      let elapsed = end_time.as_nanos();
      if elapsed > max {
        max = elapsed
      }
    }
    max
  }

  pub fn get_nanos_min(
    &self,
    name: &str,
  ) -> u128 {
    if !self.end_times.contains_key(name) {
      self.lap(name);
    }
    let end_times = self.get_end_times(name);
    let mut min: u128 = end_times[0].clone().as_nanos();
    for end_time in end_times.iter() {
      let elapsed = end_time.as_nanos();
      if elapsed < min {
        min = elapsed
      }
    }
    min
  }

  pub fn get_millis_average(
    &self,
    name: &str,
  ) -> f64 {
    let elapsed_ns = self.get_nanos_average(name);
    let elapsed_ms = elapsed_ns as f64 / 1_000_000 as f64;
    return elapsed_ms;
  }

  pub fn get_millis_median(
    &self,
    name: &str,
  ) -> f64 {
    let elapsed = self.get_nanos_median(name);
    let elapsed_ns = elapsed as f64;
    let elapsed_ms = elapsed_ns as f64 / 1_000_000 as f64;
    return elapsed_ms;
  }

  pub fn get_millis_total(
    &self,
    name: &str,
  ) -> f64 {
    let elapsed = self.get_nanos_total(name);
    let elapsed_ns = elapsed as f64;
    let elapsed_ms = elapsed_ns as f64 / 1_000_000 as f64;
    return elapsed_ms;
  }

  pub fn get_millis_max(
    &self,
    name: &str,
  ) -> f64 {
    let elapsed = self.get_nanos_max(name);
    let elapsed_ns = elapsed as f64;
    let elapsed_ms = elapsed_ns as f64 / 1_000_000 as f64;
    return elapsed_ms;
  }

  pub fn get_millis_min(
    &self,
    name: &str,
  ) -> f64 {
    let elapsed = self.get_nanos_min(name);
    let elapsed_ns = elapsed as f64;
    let elapsed_ms = elapsed_ns as f64 / 1_000_000 as f64;
    return elapsed_ms;
  }

  pub fn get_seconds_average(
    &self,
    name: &str,
  ) -> f64 {
    let elapsed_ms = self.get_millis_average(name);
    return elapsed_ms / 1_000 as f64;
  }

  pub fn get_seconds_median(
    &self,
    name: &str,
  ) -> f64 {
    let elapsed_ms = self.get_millis_median(name);
    return elapsed_ms / 1_000 as f64;
  }

  pub fn get_seconds_total(
    &self,
    name: &str,
  ) -> f64 {
    let elapsed_ms = self.get_millis_total(name);
    return elapsed_ms / 1_000 as f64;
  }

  pub fn get_seconds_max(
    &self,
    name: &str,
  ) -> f64 {
    let elapsed_ms = self.get_millis_max(name);
    return elapsed_ms / 1_000 as f64;
  }

  pub fn get_seconds_min(
    &self,
    name: &str,
  ) -> f64 {
    let elapsed_ms = self.get_millis_min(name);
    return elapsed_ms / 1_000 as f64;
  }

  pub fn log_nanos_average(
    &self,
    name: &str,
  ) {
    let count = self.get_end_times(name).len();
    let elapsed = self.get_nanos_average(name);
    self.write_log(name, "average", &count, &format!("{}", elapsed), "ns");
  }

  pub fn log_nanos_median(
    &self,
    name: &str,
  ) {
    let count = self.get_end_times(name).len();
    let elapsed = self.get_nanos_median(name);
    self.write_log(name, "median", &count, &format!("{}", elapsed), "ns");
  }

  pub fn log_nanos_total(
    &self,
    name: &str,
  ) {
    let count = self.get_end_times(name).len();
    let elapsed = self.get_nanos_total(name);
    self.write_log(name, "total", &count, &format!("{}", elapsed), "ns");
  }

  pub fn log_nanos_max(
    &self,
    name: &str,
  ) {
    let count = self.get_end_times(name).len();
    let elapsed = self.get_nanos_max(name);
    self.write_log(name, "max", &count, &format!("{}", elapsed), "ns");
  }

  pub fn log_nanos_min(
    &self,
    name: &str,
  ) {
    let count = self.get_end_times(name).len();
    let elapsed = self.get_nanos_min(name);
    self.write_log(name, "min", &count, &format!("{}", elapsed), "ns");
  }

  pub fn log_millis_average(
    &self,
    name: &str,
  ) {
    let count = self.get_end_times(name).len();
    let elapsed_ms = self.get_millis_average(name);
    self.write_log(name, "average", &count, &format!("{}", elapsed_ms), "ms");
  }

  pub fn log_millis_median(
    &self,
    name: &str,
  ) {
    let count = self.get_end_times(name).len();
    let elapsed_ms = self.get_millis_median(name);
    self.write_log(name, "median", &count, &format!("{}", elapsed_ms), "ms");
  }

  pub fn log_millis_total(
    &self,
    name: &str,
  ) {
    let count = self.get_end_times(name).len();
    let elapsed_ms = self.get_millis_total(name);
    self.write_log(name, "total", &count, &format!("{}", elapsed_ms), "ms");
  }

  pub fn log_millis_max(
    &self,
    name: &str,
  ) {
    let count = self.get_end_times(name).len();
    let elapsed_ms = self.get_millis_max(name);
    self.write_log(name, "max", &count, &format!("{}", elapsed_ms), "ms");
  }

  pub fn log_millis_min(
    &self,
    name: &str,
  ) {
    let count = self.get_end_times(name).len();
    let elapsed_ms = self.get_millis_min(name);
    self.write_log(name, "min", &count, &format!("{}", elapsed_ms), "ms");
  }

  pub fn log_seconds_average(
    &self,
    name: &str,
  ) {
    let count = self.get_end_times(name).len();
    let elapsed_s = self.get_seconds_average(name);
    self.write_log(name, "average", &count, &format!("{:.3}", elapsed_s), "s");
  }

  pub fn log_seconds_median(
    &self,
    name: &str,
  ) {
    let count = self.get_end_times(name).len();
    let elapsed_s = self.get_seconds_median(name);
    self.write_log(name, "median", &count, &format!("{:.3}", elapsed_s), "s");
  }

  pub fn log_seconds_total(
    &self,
    name: &str,
  ) {
    let count = self.get_end_times(name).len();
    let elapsed_s = self.get_seconds_total(name);
    self.write_log(name, "total", &count, &format!("{:.3}", elapsed_s), "s");
  }

  pub fn log_seconds_max(
    &self,
    name: &str,
  ) {
    let count = self.get_end_times(name).len();
    let elapsed_s = self.get_seconds_max(name);
    self.write_log(name, "max", &count, &format!("{:.3}", elapsed_s), "s");
  }

  pub fn log_seconds_min(
    &self,
    name: &str,
  ) {
    let count = self.get_end_times(name).len();
    let elapsed_s = self.get_seconds_min(name);
    self.write_log(name, "min", &count, &format!("{:.3}", elapsed_s), "s");
  }

  fn write_log(
    &self,
    name: &str,
    kind: &str,
    count: &usize,
    elapsed: &str,
    unit: &str,
  ) {
    let key = format!("{}_{}:", name, kind);
    let count = format!("({})", count);
    println!("{:<10}{:<20} {} {}", count, key, elapsed, unit);

    if let Some(log_file) = self.log_file.lock().unwrap().as_mut() {
      writeln!(log_file, "{},{},{},{}", name, count, elapsed, unit).unwrap();
    }
  }

  fn get_end_times(
    &self,
    name: &str,
  ) -> Ref<'_, String, Vec<Duration>> {
    if !self.end_times.contains_key(name) {
      self.lap(name);
    }
    let times = self
      .end_times
      .get(name)
      .expect(&format!("could not find end performance mark \"{}\"", name));
    return times;
  }
}

impl std::fmt::Debug for Profiler {
  fn fmt(
    &self,
    _: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    std::fmt::Result::Ok(())
  }
}

impl Serialize for Profiler {
  fn serialize<S>(
    &self,
    serializer: S,
  ) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_str("Profiler")
  }
}
