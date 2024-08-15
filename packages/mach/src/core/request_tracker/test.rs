use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;

use super::*;

#[test]
fn should_run_request() {
  let mut rt = RequestTracker::<Vec<String>>::new();

  let request_c = TestRequest::new("C", &[]);
  let request_b = TestRequest::new("B", &[&request_c]);
  let request_a = TestRequest::new("A", &[&request_b]);

  let result = rt.run_request(&request_a).unwrap();

  assert_eq!(result[0], "A");
  assert_eq!(result[1], "B");
  assert_eq!(result[2], "C");
}

#[test]
fn should_reuse_previously_run_request() {
  let mut rt = RequestTracker::<Vec<String>>::new();

  let request_c = TestRequest::new("C", &[]);
  let request_b = TestRequest::new("B", &[&request_c]);
  let request_a = TestRequest::new("A", &[&request_b]);

  let result = rt.run_request(&request_a).unwrap();

  assert_eq!(result[0], "A");
  assert_eq!(result[1], "B");
  assert_eq!(result[2], "C");

  let result = rt.run_request(&request_a).unwrap();
  assert_eq!(result[0], "A");
  assert_eq!(result[1], "B");
  assert_eq!(result[2], "C");
}

#[test]
fn should_run_request_once() {
  let mut rt = RequestTracker::<Vec<String>>::new();

  let request_a = TestRequest::new("A", &[]);

  let result = rt.run_request(&request_a).unwrap();

  assert_eq!(result[0], "A");
  assert_eq!(request_a.run_count(), 1);

  let result = rt.run_request(&request_a).unwrap();
  assert_eq!(result[0], "A");
  assert_eq!(request_a.run_count(), 1);
}

#[test]
fn should_run_request_once_2() {
  let mut rt = RequestTracker::<Vec<String>>::new();

  let request_b = TestRequest::new("B", &[]);
  let request_a = TestRequest::new("A", &[&request_b]);

  let result = rt.run_request(&request_a).unwrap();

  assert_eq!(result[0], "A");
  assert_eq!(result[1], "B");
  assert_eq!(request_a.run_count(), 1);
  assert_eq!(request_b.run_count(), 1);

  let result = rt.run_request(&request_a).unwrap();
  assert_eq!(result[0], "A");
  assert_eq!(result[1], "B");
  assert_eq!(request_a.run_count(), 1);
  assert_eq!(request_b.run_count(), 1);
}

/// This is a universal "Request" that can be instructed
/// to run sub_requests via the constructor
#[derive(Clone, Default)]
pub struct TestRequest<'a> {
  pub runs: Arc<AtomicUsize>,
  pub name: String,
  pub sub_requests: Arc<Mutex<Vec<&'a TestRequest<'a>>>>,
}

impl<'a> std::fmt::Debug for TestRequest<'a> {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    f.debug_struct(&format!("TestRequest({})", self.name))
      .finish()
  }
}

impl<'a> TestRequest<'a> {
  pub fn new<T: AsRef<str>>(
    name: T,
    sub_requests: &[&'a TestRequest<'a>],
  ) -> Self {
    Self {
      runs: Default::default(),
      name: name.as_ref().to_string(),
      sub_requests: Arc::new(Mutex::new(sub_requests.to_owned())),
    }
  }

  pub fn run_count(&self) -> usize {
    self.runs.load(Ordering::Relaxed)
  }
}

impl<'a> std::hash::Hash for TestRequest<'a> {
  fn hash<H: std::hash::Hasher>(
    &self,
    state: &mut H,
  ) {
    self.name.hash(state);
  }
}

impl<'a> Request<Vec<String>> for TestRequest<'a> {
  fn run(
    &self,
    mut context: RunRequestContext<Vec<String>>,
  ) -> Result<RequestResult<Vec<String>>, RunRequestError> {
    self.runs.fetch_add(1, Ordering::Relaxed);

    let name = self.name.clone();
    let mut sub_requests = self.sub_requests.lock().unwrap().clone();

    let mut result = vec![name];

    while let Some(sub_request) = sub_requests.pop() {
      let req = sub_request.clone();
      let sub_request_result = context.run_request(&req)?;
      result.extend(sub_request_result);
    }

    Ok(RequestResult {
      result,
      invalidations: vec![],
    })
  }
}
