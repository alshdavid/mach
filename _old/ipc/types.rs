use serde::{de::DeserializeOwned, Serialize};

pub trait DataTo: Clone + Send + Sync + Serialize + DeserializeOwned {}
pub trait DataFrom: Clone + Send + Sync + Serialize + DeserializeOwned {}