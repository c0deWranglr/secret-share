pub mod in_memory;
pub mod google_cloud_storage;

pub use in_memory::InMemoryHash;
pub use google_cloud_storage::CloudStorage;

use std::time::Duration;
use std::result::Result;
use std::error::Error;
use serde::{Serialize, Deserialize};
use super::cipher::Bytes;

#[async_trait::async_trait]
pub trait StorageAdapter {
    async fn prepare(&mut self, key: &str, value: String, ttl: Duration) -> Result<Bytes, Box<dyn Error>>;
    
    async fn save(&mut self, key: &str, value: Bytes) -> Result<(), Box<dyn Error>>;

    async fn prepare_and_save(&mut self, key: &str, value: String, ttl: Duration) -> Result<Bytes, Box<dyn Error>> {
        let bytes = self.prepare(key, value, ttl).await?;
        self.save(key, bytes.clone()).await?;
        Ok(bytes)
    }
    
    async fn get(&mut self, key: &str) -> Result<Bytes, Box<dyn Error>>;

    async fn extract(&mut self, value: Bytes) -> Result<String, Box<dyn Error>>;

    async fn get_and_extract(&mut self, key: &str) -> Result<String, Box<dyn Error>> {
        let bytes = self.get(key).await?;
        self.extract(bytes).await
    }

    async fn delete(&mut self, key: &str) -> Result<(), Box<dyn Error>>;
}