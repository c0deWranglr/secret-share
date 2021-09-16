pub mod in_memory;

pub use in_memory::InMemoryHash;

use std::time::Duration;

pub enum KeyExpiration {
    Never,
    AfterUse(u32),
    AfterTime(Duration)
}

pub trait StorageAdapter {
    fn get(&mut self, key: &str) -> Option<String>;

    fn set(&mut self, key: &str, value: String, expiration: KeyExpiration);
}