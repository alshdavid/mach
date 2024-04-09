// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

use crate::deno_cli::args::CliOptions;
use crate::deno_cli::args::Flags;
use crate::deno_cli::args::TestFlags;
use crate::deno_cli::args::TestReporterConfig;
use crate::deno_cli::colors;
use crate::deno_cli::factory::CliFactory;
use crate::deno_cli::factory::CliFactoryBuilder;
use crate::deno_cli::file_fetcher::File;
use crate::deno_cli::file_fetcher::FileFetcher;
use crate::deno_cli::graph_util::has_graph_root_local_dependent_changed;
use crate::deno_cli::module_loader::ModuleLoadPreparer;
use crate::deno_cli::ops;
use crate::deno_cli::util::display;
use crate::deno_cli::util::file_watcher;
use crate::deno_cli::util::fs::collect_specifiers;
use crate::deno_cli::util::fs::WalkEntry;
use crate::deno_cli::util::path::get_extension;
use crate::deno_cli::util::path::is_script_ext;
use crate::deno_cli::util::path::mapped_specifier_for_tsc;
use crate::deno_cli::util::path::matches_pattern_or_exact_path;
use crate::deno_cli::worker::CliMainWorkerFactory;

use deno_ast::swc::common::comments::CommentKind;
use deno_ast::MediaType;
use deno_ast::SourceRangedForSpanned;
use deno_config::glob::FilePatterns;
use deno_core::anyhow;
use deno_core::anyhow::bail;
use deno_core::anyhow::Context as _;
use deno_core::error::generic_error;
use deno_core::error::AnyError;
use deno_core::error::JsError;
use deno_core::futures::future;
use deno_core::futures::stream;
use deno_core::futures::FutureExt;
use deno_core::futures::StreamExt;
use deno_core::located_script_name;
use deno_core::serde_v8;
use deno_core::stats::RuntimeActivity;
use deno_core::stats::RuntimeActivityDiff;
use deno_core::stats::RuntimeActivityStats;
use deno_core::stats::RuntimeActivityStatsFactory;
use deno_core::stats::RuntimeActivityStatsFilter;
use deno_core::unsync::spawn;
use deno_core::unsync::spawn_blocking;
use deno_core::url::Url;
use deno_core::v8;
use deno_core::ModuleSpecifier;
use deno_core::PollEventLoopOptions;
use deno_runtime::deno_io::Stdio;
use deno_runtime::deno_io::StdioPipe;
use deno_runtime::fmt_errors::format_js_error;
use deno_runtime::permissions::Permissions;
use deno_runtime::permissions::PermissionsContainer;
use deno_runtime::tokio_util::create_and_run_current_thread;
use deno_runtime::worker::MainWorker;
use indexmap::IndexMap;
use indexmap::IndexSet;
use log::Level;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use regex::Regex;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Write as _;
use std::future::poll_fn;
use std::io::Write;
use std::num::NonZeroUsize;
use std::path::Path;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::task::Poll;
use std::time::Duration;
use std::time::Instant;
use std::time::SystemTime;
use tokio::signal;

mod channel;
pub mod fmt;
pub mod reporters;

pub use channel::create_single_test_event_channel;
pub use channel::create_test_event_channel;
pub use channel::TestEventReceiver;
pub use channel::TestEventSender;
pub use channel::TestEventWorkerSender;
use fmt::format_sanitizer_diff;
pub use fmt::format_test_error;
use reporters::CompoundTestReporter;
use reporters::DotTestReporter;
use reporters::JunitTestReporter;
use reporters::PrettyTestReporter;
use reporters::TapTestReporter;
use reporters::TestReporter;

/// How many times we're allowed to spin the event loop before considering something a leak.
const MAX_SANITIZER_LOOP_SPINS: usize = 16;

/// The test mode is used to determine how a specifier is to be tested.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TestMode {
  /// Test as documentation, type-checking fenced code blocks.
  Documentation,
  /// Test as an executable module, loading the module into the isolate and running each test it
  /// defines.
  Executable,
  /// Test as both documentation and an executable module.
  Both,
}

#[derive(Clone, Debug, Default)]
pub struct TestFilter {
  pub substring: Option<String>,
  pub regex: Option<Regex>,
  pub include: Option<Vec<String>>,
  pub exclude: Vec<String>,
}

impl TestFilter {
  pub fn includes(
    &self,
    name: &String,
  ) -> bool {
    if let Some(substring) = &self.substring {
      if !name.contains(substring) {
        return false;
      }
    }
    if let Some(regex) = &self.regex {
      if !regex.is_match(name) {
        return false;
      }
    }
    if let Some(include) = &self.include {
      if !include.contains(name) {
        return false;
      }
    }
    if self.exclude.contains(name) {
      return false;
    }
    true
  }

  pub fn from_flag(flag: &Option<String>) -> Self {
    let mut substring = None;
    let mut regex = None;
    if let Some(flag) = flag {
      if flag.starts_with('/') && flag.ends_with('/') {
        let rs = flag.trim_start_matches('/').trim_end_matches('/');
        regex = Some(Regex::new(rs).unwrap_or_else(|_| Regex::new("$^").unwrap()));
      } else {
        substring = Some(flag.clone());
      }
    }
    Self {
      substring,
      regex,
      ..Default::default()
    }
  }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct TestLocation {
  pub file_name: String,
  pub line_number: u32,
  pub column_number: u32,
}

#[derive(Default)]
pub(crate) struct TestContainer(TestDescriptions, Vec<v8::Global<v8::Function>>);

impl TestContainer {
  pub fn register(
    &mut self,
    description: TestDescription,
    function: v8::Global<v8::Function>,
  ) {
    self.0.tests.insert(description.id, description);
    self.1.push(function)
  }

  pub fn is_empty(&self) -> bool {
    self.1.is_empty()
  }
}

#[derive(Default, Debug)]
pub struct TestDescriptions {
  tests: IndexMap<usize, TestDescription>,
}

impl TestDescriptions {
  pub fn len(&self) -> usize {
    self.tests.len()
  }

  pub fn is_empty(&self) -> bool {
    self.tests.is_empty()
  }
}

impl<'a> IntoIterator for &'a TestDescriptions {
  type Item = <&'a IndexMap<usize, TestDescription> as IntoIterator>::Item;
  type IntoIter = <&'a IndexMap<usize, TestDescription> as IntoIterator>::IntoIter;
  fn into_iter(self) -> Self::IntoIter {
    (&self.tests).into_iter()
  }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct TestDescription {
  pub id: usize,
  pub name: String,
  pub ignore: bool,
  pub only: bool,
  pub origin: String,
  pub location: TestLocation,
  pub sanitize_ops: bool,
  pub sanitize_resources: bool,
}

/// May represent a failure of a test or test step.
#[derive(Debug, Clone, PartialEq, Deserialize, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct TestFailureDescription {
  pub id: usize,
  pub name: String,
  pub origin: String,
  pub location: TestLocation,
}

impl From<&TestDescription> for TestFailureDescription {
  fn from(value: &TestDescription) -> Self {
    Self {
      id: value.id,
      name: value.name.clone(),
      origin: value.origin.clone(),
      location: value.location.clone(),
    }
  }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TestFailure {
  JsError(Box<JsError>),
  FailedSteps(usize),
  IncompleteSteps,
  Leaked(Vec<String>, Vec<String>), // Details, trailer notes
  // The rest are for steps only.
  Incomplete,
  OverlapsWithSanitizers(IndexSet<String>), // Long names of overlapped tests
  HasSanitizersAndOverlaps(IndexSet<String>), // Long names of overlapped tests
}

impl ToString for TestFailure {
  fn to_string(&self) -> String {
    match self {
      TestFailure::JsError(js_error) => format_test_error(js_error),
      TestFailure::FailedSteps(1) => "1 test step failed.".to_string(),
      TestFailure::FailedSteps(n) => format!("{} test steps failed.", n),
      TestFailure::IncompleteSteps => "Completed while steps were still running. Ensure all steps are awaited with `await t.step(...)`.".to_string(),
      TestFailure::Incomplete => "Didn't complete before parent. Await step with `await t.step(...)`.".to_string(),
      TestFailure::Leaked(details, trailer_notes) => {
        let mut string = "Leaks detected:".to_string();
        for detail in details {
          string.push_str(&format!("\n  - {detail}"));
        }
        for trailer in trailer_notes {
          string.push_str(&format!("\n{trailer}"));
        }
        string
      }
      TestFailure::OverlapsWithSanitizers(long_names) => {
        let mut string = "Started test step while another test step with sanitizers was running:".to_string();
        for long_name in long_names {
          string.push_str(&format!("\n  * {}", long_name));
        }
        string
      }
      TestFailure::HasSanitizersAndOverlaps(long_names) => {
        let mut string = "Started test step with sanitizers while another test step was running:".to_string();
        for long_name in long_names {
          string.push_str(&format!("\n  * {}", long_name));
        }
        string
      }
    }
  }
}

impl TestFailure {
  fn format_label(&self) -> String {
    match self {
      TestFailure::Incomplete => colors::gray("INCOMPLETE").to_string(),
      _ => colors::red("FAILED").to_string(),
    }
  }

  fn format_inline_summary(&self) -> Option<String> {
    match self {
      TestFailure::FailedSteps(1) => Some("due to 1 failed step".to_string()),
      TestFailure::FailedSteps(n) => Some(format!("due to {} failed steps", n)),
      TestFailure::IncompleteSteps => Some("due to incomplete steps".to_string()),
      _ => None,
    }
  }

  fn hide_in_summary(&self) -> bool {
    // These failure variants are hidden in summaries because they are caused
    // by child errors that will be summarized separately.
    matches!(
      self,
      TestFailure::FailedSteps(_) | TestFailure::IncompleteSteps
    )
  }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TestResult {
  Ok,
  Ignored,
  Failed(TestFailure),
  Cancelled,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestStepDescription {
  pub id: usize,
  pub name: String,
  pub origin: String,
  pub location: TestLocation,
  pub level: usize,
  pub parent_id: usize,
  pub root_id: usize,
  pub root_name: String,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TestStepResult {
  Ok,
  Ignored,
  Failed(TestFailure),
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestPlan {
  pub origin: String,
  pub total: usize,
  pub filtered_out: usize,
  pub used_only: bool,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Deserialize)]
pub enum TestStdioStream {
  Stdout,
  Stderr,
}

#[derive(Debug)]
pub enum TestEvent {
  Register(Arc<TestDescriptions>),
  Plan(TestPlan),
  Wait(usize),
  Output(TestStdioStream, Vec<u8>),
  Result(usize, TestResult, u64),
  UncaughtError(String, Box<JsError>),
  StepRegister(TestStepDescription),
  StepWait(usize),
  StepResult(usize, TestStepResult, u64),
  /// Indicates that this worker has completed running tests.
  Completed,
  /// Indicates that the user has cancelled the test run with Ctrl+C and
  /// the run should be aborted.
  Sigint,
  /// Used by the REPL to force a report to end without closing the worker
  /// or receiver.
  ForceEndReport,
}

impl TestEvent {
  // Certain messages require us to ensure that all output has been drained to ensure proper
  // interleaving of output messages.
  pub fn requires_stdio_sync(&self) -> bool {
    matches!(
      self,
      TestEvent::Plan(..)
        | TestEvent::Result(..)
        | TestEvent::StepWait(..)
        | TestEvent::StepResult(..)
        | TestEvent::UncaughtError(..)
        | TestEvent::ForceEndReport
        | TestEvent::Completed
    )
  }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TestSummary {
  pub total: usize,
  pub passed: usize,
  pub failed: usize,
  pub ignored: usize,
  pub passed_steps: usize,
  pub failed_steps: usize,
  pub ignored_steps: usize,
  pub filtered_out: usize,
  pub measured: usize,
  pub failures: Vec<(TestFailureDescription, TestFailure)>,
  pub uncaught_errors: Vec<(String, Box<JsError>)>,
}

#[derive(Debug, Clone)]
struct TestSpecifiersOptions {
  concurrent_jobs: NonZeroUsize,
  fail_fast: Option<NonZeroUsize>,
  log_level: Option<log::Level>,
  filter: bool,
  specifier: TestSpecifierOptions,
  reporter: TestReporterConfig,
  junit_path: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct TestSpecifierOptions {
  pub shuffle: Option<u64>,
  pub filter: TestFilter,
  pub trace_leaks: bool,
}

impl TestSummary {
  pub fn new() -> TestSummary {
    TestSummary {
      total: 0,
      passed: 0,
      failed: 0,
      ignored: 0,
      passed_steps: 0,
      failed_steps: 0,
      ignored_steps: 0,
      filtered_out: 0,
      measured: 0,
      failures: Vec::new(),
      uncaught_errors: Vec::new(),
    }
  }

  fn has_failed(&self) -> bool {
    self.failed > 0 || !self.failures.is_empty()
  }
}

fn get_test_reporter(options: &TestSpecifiersOptions) -> Box<dyn TestReporter> {
  let parallel = options.concurrent_jobs.get() > 1;
  let reporter: Box<dyn TestReporter> = match &options.reporter {
    TestReporterConfig::Dot => Box::new(DotTestReporter::new()),
    TestReporterConfig::Pretty => Box::new(PrettyTestReporter::new(
      parallel,
      options.log_level != Some(Level::Error),
      options.filter,
      false,
    )),
    TestReporterConfig::Junit => Box::new(JunitTestReporter::new("-".to_string())),
    TestReporterConfig::Tap => Box::new(TapTestReporter::new(
      options.concurrent_jobs > NonZeroUsize::new(1).unwrap(),
    )),
  };

  if let Some(junit_path) = &options.junit_path {
    let junit = Box::new(JunitTestReporter::new(junit_path.to_string()));
    return Box::new(CompoundTestReporter::new(vec![reporter, junit]));
  }

  reporter
}

/// Test a single specifier as documentation containing test programs, an executable test module or
/// both.
pub async fn test_specifier(
  worker_factory: Arc<CliMainWorkerFactory>,
  permissions: Permissions,
  specifier: ModuleSpecifier,
  mut worker_sender: TestEventWorkerSender,
  fail_fast_tracker: FailFastTracker,
  options: TestSpecifierOptions,
) -> Result<(), AnyError> {
  match test_specifier_inner(
    worker_factory,
    permissions,
    specifier.clone(),
    &mut worker_sender.sender,
    StdioPipe::file(worker_sender.stdout),
    StdioPipe::file(worker_sender.stderr),
    fail_fast_tracker,
    options,
  )
  .await
  {
    Ok(()) => Ok(()),
    Err(error) => {
      if error.is::<JsError>() {
        worker_sender.sender.send(TestEvent::UncaughtError(
          specifier.to_string(),
          Box::new(error.downcast::<JsError>().unwrap()),
        ))?;
        Ok(())
      } else {
        Err(error)
      }
    }
  }
}

/// Test a single specifier as documentation containing test programs, an executable test module or
/// both.
#[allow(clippy::too_many_arguments)]
async fn test_specifier_inner(
  worker_factory: Arc<CliMainWorkerFactory>,
  permissions: Permissions,
  specifier: ModuleSpecifier,
  sender: &mut TestEventSender,
  stdout: StdioPipe,
  stderr: StdioPipe,
  fail_fast_tracker: FailFastTracker,
  options: TestSpecifierOptions,
) -> Result<(), AnyError> {
  if fail_fast_tracker.should_stop() {
    return Ok(());
  }
  let mut worker = worker_factory
    .create_custom_worker(
      specifier.clone(),
      PermissionsContainer::new(permissions),
      vec![ops::testing::deno_test::init_ops(sender.clone())],
      Stdio {
        stdin: StdioPipe::inherit(),
        stdout,
        stderr,
      },
    )
    .await?;

  let mut coverage_collector = worker.maybe_setup_coverage_collector().await?;

  if options.trace_leaks {
    worker.execute_script_static(
      located_script_name!(),
      "Deno[Deno.internal].core.setLeakTracingEnabled(true);",
    )?;
  }

  // We execute the main module as a side module so that import.meta.main is not set.
  worker.execute_side_module_possibly_with_npm().await?;

  let mut worker = worker.into_main_worker();

  // Ensure that there are no pending exceptions before we start running tests
  worker.run_up_to_duration(Duration::from_millis(0)).await?;

  worker.dispatch_load_event(located_script_name!())?;

  run_tests_for_worker(&mut worker, &specifier, &options, &fail_fast_tracker).await?;

  // Ignore `defaultPrevented` of the `beforeunload` event. We don't allow the
  // event loop to continue beyond what's needed to await results.
  worker.dispatch_beforeunload_event(located_script_name!())?;
  worker.dispatch_unload_event(located_script_name!())?;

  // Ensure all output has been flushed
  _ = sender.flush();

  // Ensure the worker has settled so we can catch any remaining unhandled rejections. We don't
  // want to wait forever here.
  worker.run_up_to_duration(Duration::from_millis(0)).await?;

  if let Some(coverage_collector) = coverage_collector.as_mut() {
    worker
      .js_runtime
      .with_event_loop_future(
        coverage_collector.stop_collecting().boxed_local(),
        PollEventLoopOptions::default(),
      )
      .await?;
  }
  Ok(())
}

pub fn worker_has_tests(worker: &mut MainWorker) -> bool {
  let state_rc = worker.js_runtime.op_state();
  let state = state_rc.borrow();
  !state.borrow::<TestContainer>().is_empty()
}

/// Yields to tokio to allow async work to process, and then polls
/// the event loop once.
#[must_use = "The event loop result should be checked"]
pub async fn poll_event_loop(worker: &mut MainWorker) -> Result<(), AnyError> {
  // Allow any ops that to do work in the tokio event loop to do so
  tokio::task::yield_now().await;
  // Spin the event loop once
  poll_fn(|cx| {
    if let Poll::Ready(Err(err)) = worker
      .js_runtime
      .poll_event_loop(cx, PollEventLoopOptions::default())
    {
      return Poll::Ready(Err(err));
    }
    Poll::Ready(Ok(()))
  })
  .await
}

pub async fn run_tests_for_worker(
  worker: &mut MainWorker,
  specifier: &ModuleSpecifier,
  options: &TestSpecifierOptions,
  fail_fast_tracker: &FailFastTracker,
) -> Result<(), AnyError> {
  let (TestContainer(tests, test_functions), mut sender) = {
    let state_rc = worker.js_runtime.op_state();
    let mut state = state_rc.borrow_mut();
    (
      std::mem::take(&mut *state.borrow_mut::<TestContainer>()),
      state.borrow::<TestEventSender>().clone(),
    )
  };
  let tests: Arc<TestDescriptions> = tests.into();
  sender.send(TestEvent::Register(tests.clone()))?;
  let res = run_tests_for_worker_inner(
    worker,
    specifier,
    tests,
    test_functions,
    &mut sender,
    options,
    fail_fast_tracker,
  )
  .await;
  _ = sender.send(TestEvent::Completed);
  res
}

async fn run_tests_for_worker_inner(
  worker: &mut MainWorker,
  specifier: &ModuleSpecifier,
  tests: Arc<TestDescriptions>,
  test_functions: Vec<v8::Global<v8::Function>>,
  sender: &mut TestEventSender,
  options: &TestSpecifierOptions,
  fail_fast_tracker: &FailFastTracker,
) -> Result<(), AnyError> {
  let unfiltered = tests.len();

  // Build the test plan in a single pass
  let mut tests_to_run = Vec::with_capacity(tests.len());
  let mut used_only = false;
  for ((_, d), f) in tests.tests.iter().zip(test_functions) {
    if !options.filter.includes(&d.name) {
      continue;
    }

    // If we've seen an "only: true" test, the remaining tests must be "only: true" to be added
    if used_only && !d.only {
      continue;
    }

    // If this is the first "only: true" test we've seen, clear the other tests since they were
    // only: false.
    if d.only && !used_only {
      used_only = true;
      tests_to_run.clear();
    }
    tests_to_run.push((d, f));
  }

  if let Some(seed) = options.shuffle {
    tests_to_run.shuffle(&mut SmallRng::seed_from_u64(seed));
  }

  sender.send(TestEvent::Plan(TestPlan {
    origin: specifier.to_string(),
    total: tests_to_run.len(),
    filtered_out: unfiltered - tests_to_run.len(),
    used_only,
  }))?;

  let mut had_uncaught_error = false;
  let stats = worker.js_runtime.runtime_activity_stats_factory();
  let ops = worker.js_runtime.op_names();

  // These particular ops may start and stop independently of tests, so we just filter them out
  // completely.
  let op_id_host_recv_message = ops
    .iter()
    .position(|op| *op == "op_host_recv_message")
    .unwrap();
  let op_id_host_recv_ctrl = ops
    .iter()
    .position(|op| *op == "op_host_recv_ctrl")
    .unwrap();

  // For consistency between tests with and without sanitizers, we _always_ include
  // the actual sanitizer capture before and after a test, but a test that ignores resource
  // or op sanitization simply doesn't throw if one of these constraints is violated.
  let mut filter = RuntimeActivityStatsFilter::default();
  filter = filter.with_resources();
  filter = filter.with_ops();
  filter = filter.with_timers();
  filter = filter.omit_op(op_id_host_recv_ctrl as _);
  filter = filter.omit_op(op_id_host_recv_message as _);

  for (desc, function) in tests_to_run.into_iter() {
    if fail_fast_tracker.should_stop() {
      break;
    }

    // Each test needs a fresh reqwest connection pool to avoid inter-test weirdness with connections
    // failing. If we don't do this, a connection to a test server we just tore down might be re-used in
    // the next test.
    // TODO(mmastrac): this should be some sort of callback that we can implement for any subsystem
    worker
      .js_runtime
      .op_state()
      .borrow_mut()
      .try_take::<deno_runtime::deno_fetch::reqwest::Client>();

    if desc.ignore {
      sender.send(TestEvent::Result(desc.id, TestResult::Ignored, 0))?;
      continue;
    }
    if had_uncaught_error {
      sender.send(TestEvent::Result(desc.id, TestResult::Cancelled, 0))?;
      continue;
    }
    sender.send(TestEvent::Wait(desc.id))?;

    // Poll event loop once, to allow all ops that are already resolved, but haven't
    // responded to settle.
    // TODO(mmastrac): we should provide an API to poll the event loop until no further
    // progress is made.
    poll_event_loop(worker).await?;

    // We always capture stats, regardless of sanitization state
    let before = stats.clone().capture(&filter);

    let earlier = SystemTime::now();
    let call = worker.js_runtime.call(&function);
    let result = match worker
      .js_runtime
      .with_event_loop_promise(call, PollEventLoopOptions::default())
      .await
    {
      Ok(r) => r,
      Err(error) => {
        if error.is::<JsError>() {
          sender.send(TestEvent::UncaughtError(
            specifier.to_string(),
            Box::new(error.downcast::<JsError>().unwrap()),
          ))?;
          fail_fast_tracker.add_failure();
          sender.send(TestEvent::Result(desc.id, TestResult::Cancelled, 0))?;
          had_uncaught_error = true;
          continue;
        } else {
          return Err(error);
        }
      }
    };

    // Await activity stabilization
    if let Some(diff) = wait_for_activity_to_stabilize(
      worker,
      &stats,
      &filter,
      before,
      desc.sanitize_ops,
      desc.sanitize_resources,
    )
    .await?
    {
      let (formatted, trailer_notes) = format_sanitizer_diff(diff);
      if !formatted.is_empty() {
        let failure = TestFailure::Leaked(formatted, trailer_notes);
        let elapsed = SystemTime::now().duration_since(earlier)?.as_millis();
        sender.send(TestEvent::Result(
          desc.id,
          TestResult::Failed(failure),
          elapsed as u64,
        ))?;
        continue;
      }
    }

    let scope = &mut worker.js_runtime.handle_scope();
    let result = v8::Local::new(scope, result);
    let result = serde_v8::from_v8::<TestResult>(scope, result)?;
    if matches!(result, TestResult::Failed(_)) {
      fail_fast_tracker.add_failure();
    }
    let elapsed = SystemTime::now().duration_since(earlier)?.as_millis();
    sender.send(TestEvent::Result(desc.id, result, elapsed as u64))?;
  }
  Ok(())
}

async fn wait_for_activity_to_stabilize(
  worker: &mut MainWorker,
  stats: &RuntimeActivityStatsFactory,
  filter: &RuntimeActivityStatsFilter,
  before: RuntimeActivityStats,
  sanitize_ops: bool,
  sanitize_resources: bool,
) -> Result<Option<RuntimeActivityDiff>, AnyError> {
  // First, check to see if there's any diff at all. If not, just continue.
  let after = stats.clone().capture(filter);
  let mut diff = RuntimeActivityStats::diff(&before, &after);
  if diff.is_empty() {
    // No activity, so we return early
    return Ok(None);
  }

  // We allow for up to MAX_SANITIZER_LOOP_SPINS to get to a point where there is no difference.
  // TODO(mmastrac): We could be much smarter about this if we had the concept of "progress" in
  // an event loop tick. Ideally we'd be able to tell if we were spinning and doing nothing, or
  // spinning and resolving ops.
  for _ in 0..MAX_SANITIZER_LOOP_SPINS {
    // There was a diff, so let the event loop run once
    poll_event_loop(worker).await?;

    let after = stats.clone().capture(filter);
    diff = RuntimeActivityStats::diff(&before, &after);
    if diff.is_empty() {
      return Ok(None);
    }
  }

  if !sanitize_ops {
    diff
      .appeared
      .retain(|activity| !matches!(activity, RuntimeActivity::AsyncOp(..)));
    diff
      .disappeared
      .retain(|activity| !matches!(activity, RuntimeActivity::AsyncOp(..)));
  }
  if !sanitize_resources {
    diff
      .appeared
      .retain(|activity| !matches!(activity, RuntimeActivity::Resource(..)));
    diff
      .disappeared
      .retain(|activity| !matches!(activity, RuntimeActivity::Resource(..)));
  }

  // Since we don't have an option to disable timer sanitization, we use sanitize_ops == false &&
  // sanitize_resources == false to disable those.
  if !sanitize_ops && !sanitize_resources {
    diff.appeared.retain(|activity| {
      !matches!(
        activity,
        RuntimeActivity::Timer(..) | RuntimeActivity::Interval(..)
      )
    });
    diff.disappeared.retain(|activity| {
      !matches!(
        activity,
        RuntimeActivity::Timer(..) | RuntimeActivity::Interval(..)
      )
    });
  }

  Ok(if diff.is_empty() { None } else { Some(diff) })
}

fn extract_files_from_regex_blocks(
  specifier: &ModuleSpecifier,
  source: &str,
  media_type: MediaType,
  file_line_index: usize,
  blocks_regex: &Regex,
  lines_regex: &Regex,
) -> Result<Vec<File>, AnyError> {
  let files = blocks_regex
    .captures_iter(source)
    .filter_map(|block| {
      block.get(1)?;

      let maybe_attributes: Option<Vec<_>> = block
        .get(1)
        .map(|attributes| attributes.as_str().split(' ').collect());

      let file_media_type = if let Some(attributes) = maybe_attributes {
        if attributes.contains(&"ignore") {
          return None;
        }

        match attributes.first() {
          Some(&"js") => MediaType::JavaScript,
          Some(&"javascript") => MediaType::JavaScript,
          Some(&"mjs") => MediaType::Mjs,
          Some(&"cjs") => MediaType::Cjs,
          Some(&"jsx") => MediaType::Jsx,
          Some(&"ts") => MediaType::TypeScript,
          Some(&"typescript") => MediaType::TypeScript,
          Some(&"mts") => MediaType::Mts,
          Some(&"cts") => MediaType::Cts,
          Some(&"tsx") => MediaType::Tsx,
          _ => MediaType::Unknown,
        }
      } else {
        media_type
      };

      if file_media_type == MediaType::Unknown {
        return None;
      }

      let line_offset = source[0..block.get(0).unwrap().start()]
        .chars()
        .filter(|c| *c == '\n')
        .count();

      let line_count = block.get(0).unwrap().as_str().split('\n').count();

      let body = block.get(2).unwrap();
      let text = body.as_str();

      // TODO(caspervonb) generate an inline source map
      let mut file_source = String::new();
      for line in lines_regex.captures_iter(text) {
        let text = line.get(1).unwrap();
        writeln!(file_source, "{}", text.as_str()).unwrap();
      }

      let file_specifier = ModuleSpecifier::parse(&format!(
        "{}${}-{}",
        specifier,
        file_line_index + line_offset + 1,
        file_line_index + line_offset + line_count + 1,
      ))
      .unwrap();
      let file_specifier = mapped_specifier_for_tsc(&file_specifier, file_media_type)
        .map(|s| ModuleSpecifier::parse(&s).unwrap())
        .unwrap_or(file_specifier);

      Some(File {
        specifier: file_specifier,
        maybe_headers: None,
        source: file_source.into_bytes().into(),
      })
    })
    .collect();

  Ok(files)
}

fn extract_files_from_source_comments(
  specifier: &ModuleSpecifier,
  source: Arc<str>,
  media_type: MediaType,
) -> Result<Vec<File>, AnyError> {
  let parsed_source = deno_ast::parse_module(deno_ast::ParseParams {
    specifier: specifier.clone(),
    text_info: deno_ast::SourceTextInfo::new(source),
    media_type,
    capture_tokens: false,
    maybe_syntax: None,
    scope_analysis: false,
  })?;
  let comments = parsed_source.comments().get_vec();
  let blocks_regex = lazy_regex::regex!(r"```([^\r\n]*)\r?\n([\S\s]*?)```");
  let lines_regex = lazy_regex::regex!(r"(?:\* ?)(?:\# ?)?(.*)");

  let files = comments
    .iter()
    .filter(|comment| {
      if comment.kind != CommentKind::Block || !comment.text.starts_with('*') {
        return false;
      }

      true
    })
    .flat_map(|comment| {
      extract_files_from_regex_blocks(
        specifier,
        &comment.text,
        media_type,
        parsed_source.text_info().line_index(comment.start()),
        blocks_regex,
        lines_regex,
      )
    })
    .flatten()
    .collect();

  Ok(files)
}

fn extract_files_from_fenced_blocks(
  specifier: &ModuleSpecifier,
  source: &str,
  media_type: MediaType,
) -> Result<Vec<File>, AnyError> {
  // The pattern matches code blocks as well as anything in HTML comment syntax,
  // but it stores the latter without any capturing groups. This way, a simple
  // check can be done to see if a block is inside a comment (and skip typechecking)
  // or not by checking for the presence of capturing groups in the matches.
  let blocks_regex = lazy_regex::regex!(r"(?s)<!--.*?-->|```([^\r\n]*)\r?\n([\S\s]*?)```");
  let lines_regex = lazy_regex::regex!(r"(?:\# ?)?(.*)");

  extract_files_from_regex_blocks(
    specifier,
    source,
    media_type,
    /* file line index */ 0,
    blocks_regex,
    lines_regex,
  )
}

async fn fetch_inline_files(
  file_fetcher: &FileFetcher,
  specifiers: Vec<ModuleSpecifier>,
) -> Result<Vec<File>, AnyError> {
  let mut files = Vec::new();
  for specifier in specifiers {
    let fetch_permissions = PermissionsContainer::allow_all();
    let file = file_fetcher
      .fetch(&specifier, fetch_permissions)
      .await?
      .into_text_decoded()?;

    let inline_files = if file.media_type == MediaType::Unknown {
      extract_files_from_fenced_blocks(&file.specifier, &file.source, file.media_type)
    } else {
      extract_files_from_source_comments(&file.specifier, file.source, file.media_type)
    };

    files.extend(inline_files?);
  }

  Ok(files)
}

/// Type check a collection of module and document specifiers.
pub async fn check_specifiers(
  cli_options: &CliOptions,
  file_fetcher: &FileFetcher,
  module_load_preparer: &ModuleLoadPreparer,
  specifiers: Vec<(ModuleSpecifier, TestMode)>,
) -> Result<(), AnyError> {
  let lib = cli_options.ts_type_lib_window();
  let inline_files = fetch_inline_files(
    file_fetcher,
    specifiers
      .iter()
      .filter_map(|(specifier, mode)| {
        if *mode != TestMode::Executable {
          Some(specifier.clone())
        } else {
          None
        }
      })
      .collect(),
  )
  .await?;

  if !inline_files.is_empty() {
    let specifiers = inline_files
      .iter()
      .map(|file| file.specifier.clone())
      .collect();

    for file in inline_files {
      file_fetcher.insert_memory_files(file);
    }

    module_load_preparer
      .prepare_module_load(
        specifiers,
        false,
        lib,
        PermissionsContainer::new(Permissions::allow_all()),
      )
      .await?;
  }

  let module_specifiers = specifiers
    .into_iter()
    .filter_map(|(specifier, mode)| {
      if mode != TestMode::Documentation {
        Some(specifier)
      } else {
        None
      }
    })
    .collect();

  module_load_preparer
    .prepare_module_load(
      module_specifiers,
      false,
      lib,
      PermissionsContainer::allow_all(),
    )
    .await?;

  Ok(())
}

static HAS_TEST_RUN_SIGINT_HANDLER: AtomicBool = AtomicBool::new(false);

/// Test a collection of specifiers with test modes concurrently.
async fn test_specifiers(
  worker_factory: Arc<CliMainWorkerFactory>,
  permissions: &Permissions,
  specifiers: Vec<ModuleSpecifier>,
  options: TestSpecifiersOptions,
) -> Result<(), AnyError> {
  let specifiers = if let Some(seed) = options.specifier.shuffle {
    let mut rng = SmallRng::seed_from_u64(seed);
    let mut specifiers = specifiers;
    specifiers.sort();
    specifiers.shuffle(&mut rng);
    specifiers
  } else {
    specifiers
  };

  let (test_event_sender_factory, receiver) = create_test_event_channel();
  let concurrent_jobs = options.concurrent_jobs;

  let mut cancel_sender = test_event_sender_factory.weak_sender();
  let sigint_handler_handle = spawn(async move {
    signal::ctrl_c().await.unwrap();
    cancel_sender.send(TestEvent::Sigint).ok();
  });
  HAS_TEST_RUN_SIGINT_HANDLER.store(true, Ordering::Relaxed);
  let reporter = get_test_reporter(&options);
  let fail_fast_tracker = FailFastTracker::new(options.fail_fast);

  let join_handles = specifiers.into_iter().map(move |specifier| {
    let worker_factory = worker_factory.clone();
    let permissions = permissions.clone();
    let worker_sender = test_event_sender_factory.worker();
    let fail_fast_tracker = fail_fast_tracker.clone();
    let specifier_options = options.specifier.clone();
    spawn_blocking(move || {
      create_and_run_current_thread(test_specifier(
        worker_factory,
        permissions,
        specifier,
        worker_sender,
        fail_fast_tracker,
        specifier_options,
      ))
    })
  });

  // TODO(mmastrac): Temporarily limit concurrency in windows testing to avoid named pipe issue:
  // *** Unexpected server pipe failure '"\\\\.\\pipe\\deno_pipe_e30f45c9df61b1e4.1198.222\\0"': 3
  // This is likely because we're hitting some sort of invisible resource limit
  // This limit is both in cli/lsp/testing/execution.rs and cli/tools/test/mod.rs
  let concurrent = if cfg!(windows) {
    std::cmp::min(concurrent_jobs.get(), 4)
  } else {
    concurrent_jobs.get()
  };

  let join_stream = stream::iter(join_handles)
    .buffer_unordered(concurrent)
    .collect::<Vec<Result<Result<(), AnyError>, tokio::task::JoinError>>>();

  let handler = spawn(async move { report_tests(receiver, reporter).await.0 });

  let (join_results, result) = future::join(join_stream, handler).await;
  sigint_handler_handle.abort();
  HAS_TEST_RUN_SIGINT_HANDLER.store(false, Ordering::Relaxed);
  for join_result in join_results {
    join_result??;
  }
  result??;

  Ok(())
}

/// Gives receiver back in case it was ended with `TestEvent::ForceEndReport`.
pub async fn report_tests(
  mut receiver: TestEventReceiver,
  mut reporter: Box<dyn TestReporter>,
) -> (Result<(), AnyError>, TestEventReceiver) {
  let mut tests = IndexMap::new();
  let mut test_steps = IndexMap::new();
  let mut tests_started = HashSet::new();
  let mut tests_with_result = HashSet::new();
  let mut start_time = None;
  let mut had_plan = false;
  let mut used_only = false;
  let mut failed = false;

  while let Some((_, event)) = receiver.recv().await {
    match event {
      TestEvent::Register(description) => {
        for (_, description) in description.into_iter() {
          reporter.report_register(description);
          // TODO(mmastrac): We shouldn't need to clone here -- we can reuse the descriptions everywhere
          tests.insert(description.id, description.clone());
        }
      }
      TestEvent::Plan(plan) => {
        if !had_plan {
          start_time = Some(Instant::now());
          had_plan = true;
        }
        if plan.used_only {
          used_only = true;
        }
        reporter.report_plan(&plan);
      }
      TestEvent::Wait(id) => {
        if tests_started.insert(id) {
          reporter.report_wait(tests.get(&id).unwrap());
        }
      }
      TestEvent::Output(_, output) => {
        reporter.report_output(&output);
      }
      TestEvent::Result(id, result, elapsed) => {
        if tests_with_result.insert(id) {
          match result {
            TestResult::Failed(_) | TestResult::Cancelled => {
              failed = true;
            }
            _ => (),
          }
          reporter.report_result(tests.get(&id).unwrap(), &result, elapsed);
        }
      }
      TestEvent::UncaughtError(origin, error) => {
        failed = true;
        reporter.report_uncaught_error(&origin, error);
      }
      TestEvent::StepRegister(description) => {
        reporter.report_step_register(&description);
        test_steps.insert(description.id, description);
      }
      TestEvent::StepWait(id) => {
        if tests_started.insert(id) {
          reporter.report_step_wait(test_steps.get(&id).unwrap());
        }
      }
      TestEvent::StepResult(id, result, duration) => {
        if tests_with_result.insert(id) {
          reporter.report_step_result(
            test_steps.get(&id).unwrap(),
            &result,
            duration,
            &tests,
            &test_steps,
          );
        }
      }
      TestEvent::ForceEndReport => {
        break;
      }
      TestEvent::Completed => {
        reporter.report_completed();
      }
      TestEvent::Sigint => {
        let elapsed = start_time
          .map(|t| Instant::now().duration_since(t))
          .unwrap_or_default();
        reporter.report_sigint(
          &tests_started
            .difference(&tests_with_result)
            .copied()
            .collect(),
          &tests,
          &test_steps,
        );
        if let Err(err) = reporter.flush_report(&elapsed, &tests, &test_steps) {
          eprint!("Test reporter failed to flush: {}", err)
        }
        std::process::exit(130);
      }
    }
  }

  let elapsed = start_time
    .map(|t| Instant::now().duration_since(t))
    .unwrap_or_default();
  reporter.report_summary(&elapsed, &tests, &test_steps);
  if let Err(err) = reporter.flush_report(&elapsed, &tests, &test_steps) {
    return (
      Err(generic_error(format!(
        "Test reporter failed to flush: {}",
        err
      ))),
      receiver,
    );
  }

  if used_only {
    return (
      Err(generic_error(
        "Test failed because the \"only\" option was used",
      )),
      receiver,
    );
  }

  if failed {
    return (Err(generic_error("Test failed")), receiver);
  }

  (Ok(()), receiver)
}

fn is_supported_test_path_predicate(entry: WalkEntry) -> bool {
  if !is_script_ext(entry.path) {
    false
  } else if has_supported_test_path_name(entry.path) {
    true
  } else if let Some(include) = &entry.patterns.include {
    // allow someone to explicitly specify a path
    matches_pattern_or_exact_path(include, entry.path)
  } else {
    false
  }
}

/// Checks if the path has a basename and extension Deno supports for tests.
pub(crate) fn is_supported_test_path(path: &Path) -> bool {
  has_supported_test_path_name(path) && is_script_ext(path)
}

fn has_supported_test_path_name(path: &Path) -> bool {
  if let Some(name) = path.file_stem() {
    let basename = name.to_string_lossy();
    basename.ends_with("_test") || basename.ends_with(".test") || basename == "test"
  } else {
    false
  }
}

/// Checks if the path has an extension Deno supports for tests.
fn is_supported_test_ext(path: &Path) -> bool {
  if let Some(ext) = get_extension(path) {
    matches!(
      ext.as_str(),
      "ts"
        | "tsx"
        | "js"
        | "jsx"
        | "mjs"
        | "mts"
        | "cjs"
        | "cts"
        | "md"
        | "mkd"
        | "mkdn"
        | "mdwn"
        | "mdown"
        | "markdown"
    )
  } else {
    false
  }
}

/// Collects specifiers marking them with the appropriate test mode while maintaining the natural
/// input order.
///
/// - Specifiers matching the `is_supported_test_ext` predicate are marked as
/// `TestMode::Documentation`.
/// - Specifiers matching the `is_supported_test_path` are marked as `TestMode::Executable`.
/// - Specifiers matching both predicates are marked as `TestMode::Both`
fn collect_specifiers_with_test_mode(
  files: FilePatterns,
  include_inline: &bool,
) -> Result<Vec<(ModuleSpecifier, TestMode)>, AnyError> {
  // todo(dsherret): there's no need to collect twice as it's slow
  let module_specifiers = collect_specifiers(files.clone(), is_supported_test_path_predicate)?;

  if *include_inline {
    return collect_specifiers(files, |e| is_supported_test_ext(e.path)).map(|specifiers| {
      specifiers
        .into_iter()
        .map(|specifier| {
          let mode = if module_specifiers.contains(&specifier) {
            TestMode::Both
          } else {
            TestMode::Documentation
          };

          (specifier, mode)
        })
        .collect()
    });
  }

  let specifiers_with_mode = module_specifiers
    .into_iter()
    .map(|specifier| (specifier, TestMode::Executable))
    .collect();

  Ok(specifiers_with_mode)
}

/// Collects module and document specifiers with test modes via
/// `collect_specifiers_with_test_mode` which are then pre-fetched and adjusted
/// based on the media type.
///
/// Specifiers that do not have a known media type that can be executed as a
/// module are marked as `TestMode::Documentation`. Type definition files
/// cannot be run, and therefore need to be marked as `TestMode::Documentation`
/// as well.
async fn fetch_specifiers_with_test_mode(
  file_fetcher: &FileFetcher,
  files: FilePatterns,
  doc: &bool,
) -> Result<Vec<(ModuleSpecifier, TestMode)>, AnyError> {
  let mut specifiers_with_mode = collect_specifiers_with_test_mode(files, doc)?;

  for (specifier, mode) in &mut specifiers_with_mode {
    let file = file_fetcher
      .fetch(specifier, PermissionsContainer::allow_all())
      .await?;

    let (media_type, _) = file.resolve_media_type_and_charset();
    if matches!(media_type, MediaType::Unknown | MediaType::Dts) {
      *mode = TestMode::Documentation
    }
  }

  Ok(specifiers_with_mode)
}

pub async fn run_tests(
  flags: Flags,
  test_flags: TestFlags,
) -> Result<(), AnyError> {
  let factory = CliFactory::from_flags(flags).await?;
  let cli_options = factory.cli_options();
  let test_options = cli_options.resolve_test_options(test_flags)?;
  let file_fetcher = factory.file_fetcher()?;
  let module_load_preparer = factory.module_load_preparer().await?;
  // Various test files should not share the same permissions in terms of
  // `PermissionsContainer` - otherwise granting/revoking permissions in one
  // file would have impact on other files, which is undesirable.
  let permissions = Permissions::from_options(&cli_options.permissions_options())?;
  let log_level = cli_options.log_level();

  let specifiers_with_mode =
    fetch_specifiers_with_test_mode(file_fetcher, test_options.files.clone(), &test_options.doc)
      .await?;

  if !test_options.allow_none && specifiers_with_mode.is_empty() {
    return Err(generic_error("No test modules found"));
  }

  check_specifiers(
    cli_options,
    file_fetcher,
    module_load_preparer,
    specifiers_with_mode.clone(),
  )
  .await?;

  if test_options.no_run {
    return Ok(());
  }

  let worker_factory = Arc::new(factory.create_cli_main_worker_factory().await?);

  test_specifiers(
    worker_factory,
    &permissions,
    specifiers_with_mode
      .into_iter()
      .filter_map(|(s, m)| match m {
        TestMode::Documentation => None,
        _ => Some(s),
      })
      .collect(),
    TestSpecifiersOptions {
      concurrent_jobs: test_options.concurrent_jobs,
      fail_fast: test_options.fail_fast,
      log_level,
      filter: test_options.filter.is_some(),
      reporter: test_options.reporter,
      junit_path: test_options.junit_path,
      specifier: TestSpecifierOptions {
        filter: TestFilter::from_flag(&test_options.filter),
        shuffle: test_options.shuffle,
        trace_leaks: test_options.trace_leaks,
      },
    },
  )
  .await?;

  Ok(())
}

pub async fn run_tests_with_watch(
  flags: Flags,
  test_flags: TestFlags,
) -> Result<(), AnyError> {
  // On top of the sigint handlers which are added and unbound for each test
  // run, a process-scoped basic exit handler is required due to a tokio
  // limitation where it doesn't unbind its own handler for the entire process
  // once a user adds one.
  spawn(async move {
    loop {
      signal::ctrl_c().await.unwrap();
      if !HAS_TEST_RUN_SIGINT_HANDLER.load(Ordering::Relaxed) {
        std::process::exit(130);
      }
    }
  });

  file_watcher::watch_func(
    flags,
    file_watcher::PrintConfig::new(
      "Test",
      test_flags
        .watch
        .as_ref()
        .map(|w| !w.no_clear_screen)
        .unwrap_or(true),
    ),
    move |flags, watcher_communicator, changed_paths| {
      let test_flags = test_flags.clone();
      Ok(async move {
        let factory = CliFactoryBuilder::new()
          .build_from_flags_for_watcher(flags, watcher_communicator.clone())
          .await?;
        let cli_options = factory.cli_options();
        let test_options = cli_options.resolve_test_options(test_flags)?;

        let _ = watcher_communicator.watch_paths(cli_options.watch_paths());
        if let Some(set) = &test_options.files.include {
          let watch_paths = set.base_paths();
          if !watch_paths.is_empty() {
            let _ = watcher_communicator.watch_paths(watch_paths);
          }
        }

        let graph_kind = cli_options.type_check_mode().as_graph_kind();
        let log_level = cli_options.log_level();
        let cli_options = cli_options.clone();
        let module_graph_creator = factory.module_graph_creator().await?;
        let file_fetcher = factory.file_fetcher()?;
        let test_modules = if test_options.doc {
          collect_specifiers(test_options.files.clone(), |e| {
            is_supported_test_ext(e.path)
          })
        } else {
          collect_specifiers(test_options.files.clone(), is_supported_test_path_predicate)
        }?;

        let permissions = Permissions::from_options(&cli_options.permissions_options())?;
        let graph = module_graph_creator
          .create_graph(graph_kind, test_modules)
          .await?;
        module_graph_creator.graph_valid(&graph)?;
        let test_modules = &graph.roots;

        let test_modules_to_reload = if let Some(changed_paths) = changed_paths {
          let mut result = Vec::new();
          let changed_paths = changed_paths.into_iter().collect::<HashSet<_>>();
          for test_module_specifier in test_modules {
            if has_graph_root_local_dependent_changed(&graph, test_module_specifier, &changed_paths)
            {
              result.push(test_module_specifier.clone());
            }
          }
          result
        } else {
          test_modules.clone()
        };

        let worker_factory = Arc::new(factory.create_cli_main_worker_factory().await?);
        let module_load_preparer = factory.module_load_preparer().await?;
        let specifiers_with_mode = fetch_specifiers_with_test_mode(
          file_fetcher,
          test_options.files.clone(),
          &test_options.doc,
        )
        .await?
        .into_iter()
        .filter(|(specifier, _)| test_modules_to_reload.contains(specifier))
        .collect::<Vec<(ModuleSpecifier, TestMode)>>();

        check_specifiers(
          &cli_options,
          file_fetcher,
          module_load_preparer,
          specifiers_with_mode.clone(),
        )
        .await?;

        if test_options.no_run {
          return Ok(());
        }

        test_specifiers(
          worker_factory,
          &permissions,
          specifiers_with_mode
            .into_iter()
            .filter_map(|(s, m)| match m {
              TestMode::Documentation => None,
              _ => Some(s),
            })
            .collect(),
          TestSpecifiersOptions {
            concurrent_jobs: test_options.concurrent_jobs,
            fail_fast: test_options.fail_fast,
            log_level,
            filter: test_options.filter.is_some(),
            reporter: test_options.reporter,
            junit_path: test_options.junit_path,
            specifier: TestSpecifierOptions {
              filter: TestFilter::from_flag(&test_options.filter),
              shuffle: test_options.shuffle,
              trace_leaks: test_options.trace_leaks,
            },
          },
        )
        .await?;

        Ok(())
      })
    },
  )
  .await?;

  Ok(())
}

/// Tracks failures for the `--fail-fast` argument in
/// order to tell when to stop running tests.
#[derive(Clone, Default)]
pub struct FailFastTracker {
  max_count: Option<usize>,
  failure_count: Arc<AtomicUsize>,
}

impl FailFastTracker {
  pub fn new(fail_fast: Option<NonZeroUsize>) -> Self {
    Self {
      max_count: fail_fast.map(|v| v.into()),
      failure_count: Default::default(),
    }
  }

  pub fn add_failure(&self) -> bool {
    if let Some(max_count) = &self.max_count {
      self
        .failure_count
        .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        >= *max_count
    } else {
      false
    }
  }

  pub fn should_stop(&self) -> bool {
    if let Some(max_count) = &self.max_count {
      self.failure_count.load(std::sync::atomic::Ordering::SeqCst) >= *max_count
    } else {
      false
    }
  }
}
