use std::collections::HashMap;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use dashmap::DashMap;
use serde::Serialize;

#[derive(Default, Clone)]
pub struct Profiler {
  start_times: Arc<DashMap<String, SystemTime>>,
  end_times: Arc<DashMap<String, (u128, Duration)>>,
}

impl Profiler {
  pub fn new() -> Self {
    Self::default()
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
    let end_time = self.start_times.get(name).unwrap().elapsed().unwrap();
    if let Some(mut end_times) = self.end_times.get_mut(name) {
      end_times.0 += 1;
      end_times.1 += end_time;
    } else {
      self.end_times.insert(name.to_string(), (1, end_time));
    }
  }

  pub fn get_nanos(
    &self,
    name: &str,
  ) -> u128 {
    if !self.end_times.contains_key(name) {
      self.lap(name);
    }
    let end_times = self.end_times.get(name).unwrap().clone();
    let average_end_time = end_times.1.as_nanos() / end_times.0;
    return average_end_time;
  }

  pub fn get_millis(
    &self,
    name: &str,
  ) -> f64 {
    let elapsed = self.get_nanos(name);
    let elapsed_ns = elapsed as f64;
    let elapsed_ms = elapsed_ns as f64 / 1_000_000 as f64;
    return elapsed_ms;
  }

  pub fn get_seconds(
    &self,
    name: &str,
  ) -> f64 {
    let elapsed_ms = self.get_millis(name);
    return elapsed_ms / 1_000 as f64;
  }

  pub fn log_nanos(
    &self,
    name: &str,
  ) {
    let elapsed = self.get_nanos(name);
    println!("{}: {}ns", name, elapsed);
  }

  pub fn log_millis(
    &self,
    name: &str,
  ) {
    let elapsed_ms = self.get_millis(name);
    println!("{}: {:.3}ms", name, elapsed_ms);
  }

  pub fn log_seconds(
    &self,
    name: &str,
  ) {
    let elapsed_s = self.get_seconds(name);
    println!("{}: {:.3}s", name, elapsed_s);
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
