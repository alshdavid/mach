#![allow(dead_code)]

use std::sync::Arc;
use std::sync::Mutex;

pub struct Container<T> {
  value: Option<T>,
}

impl<T> Container<T> {
  pub fn new(initial_value: T) -> Self {
    return Container {
      value: Some(initial_value),
    };
  }

  pub fn take(&mut self) -> T {
    return self.value.take().unwrap();
  }

  pub fn take_arc(&mut self) -> Arc<T> {
    return Arc::new(self.value.take().unwrap());
  }

  pub fn take_mutex(&mut self) -> Arc<Mutex<T>> {
    return Arc::new(Mutex::new(self.value.take().unwrap()));
  }

  pub fn insert(&mut self, value: T) {
    self.value.replace(value);
  }

  pub fn insert_arc(&mut self, value: Arc<T>) {
    let Ok(value) = Arc::try_unwrap(value) else {
      panic!()
    };
    self.insert(value);
  }

  pub fn insert_mutex(&mut self, value: Arc<Mutex<T>>) {
    let Ok(value) = Arc::try_unwrap(value) else {
      panic!()
    };
    self.insert(value.into_inner().unwrap());
  }
}
