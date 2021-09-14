use super::*;
use std::collections::HashMap;

pub struct InMemoryHash {
    data: HashMap<String, String>
}

impl InMemoryHash {
    pub fn new() -> InMemoryHash {
        return InMemoryHash { data: HashMap::new() }
    }
}

impl StorageAdapter for InMemoryHash {
    
    fn get(&mut self, key: &str) -> std::option::Option<String> { self.data.get(key).map(|v| v.to_owned()) }
    fn set(&mut self, key: &str, value: String) { self.data.insert(key.to_owned(), value); }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_set_and_get() {
        let mut adapater = InMemoryHash::new();

        assert_eq!(None, adapater.get("hello"));
        adapater.set("hello", "world".to_owned());
        assert_eq!(Some("world".to_owned()), adapater.get("hello"));
    }
}