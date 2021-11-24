pub mod in_memory;

pub use in_memory::InMemoryHash;

use std::time::Duration;
use std::result::Result;
use std::error::Error;
use serde::{Serialize, Deserialize};
use super::cipher::Bytes;

pub trait StorageAdapter {
    fn prepare(&mut self, key: &str, value: String, ttl: Duration) -> Result<Bytes, Box<dyn Error>>;
    
    fn save(&mut self, key: &str, value: Bytes) -> Result<(), Box<dyn Error>>;

    fn prepare_and_save(&mut self, key: &str, value: String, ttl: Duration) -> Result<Bytes, Box<dyn Error>> {
        let bytes = self.prepare(key, value, ttl)?;
        self.save(key, bytes.clone())?;
        Ok(bytes)
    }
    
    fn get(&mut self, key: &str) -> Result<Bytes, Box<dyn Error>>;

    fn extract(&mut self, value: Bytes) -> Result<String, Box<dyn Error>>;

    fn get_and_extract(&mut self, key: &str) -> Result<String, Box<dyn Error>> {
        let bytes = self.get(key)?;
        self.extract(bytes)
    }

    fn delete(&mut self, key: &str) -> Result<(), Box<dyn Error>>;
}