pub mod memory;

use crate::Result;
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

pub trait Cache: Send + Sync {
    fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>>;
    fn set<T: Serialize>(&self, key: &str, value: &T, ttl: Duration) -> Result<()>;
    fn clear(&self) -> Result<()>;
}

pub use memory::MemoryCache;