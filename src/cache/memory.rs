use crate::{cache::Cache, Result};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::{Duration, SystemTime},
};

#[derive(Clone)]
struct CacheEntry {
    data: String,
    expires_at: SystemTime,
}

pub struct MemoryCache {
    store: Arc<RwLock<HashMap<String, CacheEntry>>>,
}

impl Default for MemoryCache {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryCache {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    fn cleanup_expired(&self) {
        if let Ok(mut store) = self.store.write() {
            let now = SystemTime::now();
            store.retain(|_, entry| entry.expires_at > now);
        }
    }
}

impl Cache for MemoryCache {
    fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>> {
        self.cleanup_expired();
        
        let store = self.store.read().map_err(|_| {
            crate::PokeCliError::Cache("Failed to acquire read lock".to_string())
        })?;
        
        if let Some(entry) = store.get(key) {
            if entry.expires_at > SystemTime::now() {
                let value: T = serde_json::from_str(&entry.data)?;
                return Ok(Some(value));
            }
        }
        
        Ok(None)
    }
    
    fn set<T: Serialize>(&self, key: &str, value: &T, ttl: Duration) -> Result<()> {
        let data = serde_json::to_string(value)?;
        let expires_at = SystemTime::now() + ttl;
        
        let mut store = self.store.write().map_err(|_| {
            crate::PokeCliError::Cache("Failed to acquire write lock".to_string())
        })?;
        
        store.insert(key.to_string(), CacheEntry { data, expires_at });
        Ok(())
    }
    
    fn clear(&self) -> Result<()> {
        let mut store = self.store.write().map_err(|_| {
            crate::PokeCliError::Cache("Failed to acquire write lock".to_string())
        })?;
        
        store.clear();
        Ok(())
    }
}