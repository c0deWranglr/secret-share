pub mod in_memory;

pub use in_memory::InMemoryHash;

pub trait StorageAdapter {
    fn get(&mut self, key: &str) -> Option<String>;

    fn set(&mut self, key: &str, value: String);
}