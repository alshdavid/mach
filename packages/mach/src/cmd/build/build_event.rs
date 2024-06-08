use std::time::SystemTime;

#[derive(Clone, Debug)]
pub enum BuildEvent {
  BuildStart {
    timestamp: SystemTime,
  },
  ConfigLoaded {
    timestamp: SystemTime,
  },
  InitializationComplete {
    timestamp: SystemTime,
  },
  TransformationComplete {
    timestamp: SystemTime,
  },
  BundlingComplete {
    timestamp: SystemTime,
  },
  PackagingComplete {
    timestamp: SystemTime,
  },
  BuildComplete {
    timestamp: SystemTime,
  },
}

impl BuildEvent {
  pub fn build_start() -> Self {
    Self::BuildStart { timestamp: SystemTime::now() }
  }
  pub fn config_loaded() -> Self {
    Self::ConfigLoaded { timestamp: SystemTime::now() }
  }
  pub fn initialization_complete() -> Self {
    Self::InitializationComplete { timestamp: SystemTime::now() }
  }
  pub fn transformation_complete() -> Self {
    Self::TransformationComplete { timestamp: SystemTime::now() }
  }
  pub fn bundling_complete() -> Self {
    Self::BundlingComplete { timestamp: SystemTime::now() }
  }
  pub fn packaging_complete() -> Self {
    Self::PackagingComplete { timestamp: SystemTime::now() }
  }
  pub fn build_complete() -> Self {
    Self::BuildComplete { timestamp: SystemTime::now() }
  }
}
