use super::*;
use std::{ time::Instant, collections::HashMap};

struct StoredValue {
    value: String,
    access_count: u32,
    expiration: KeyExpiration,
    created_at: Instant
}

pub struct InMemoryHash {
    data: HashMap<String, StoredValue>
}

impl InMemoryHash {
    pub fn new() -> InMemoryHash {
        return InMemoryHash { data: HashMap::new() }
    }

    fn expire(val: &StoredValue) -> bool {
        match val.expiration {
            KeyExpiration::Never => false,
            KeyExpiration::AfterUse(times) => val.access_count > times,
            KeyExpiration::AfterTime(duration) => val.created_at.elapsed() >= duration
        }
    }
}

impl StorageAdapter for InMemoryHash {
    
    fn get(&mut self, key: &str) -> std::option::Option<String> { 
        let val = { self.data.get_mut(key) };
        if let Some(v) = val {
            v.access_count += 1;
            let ret_value = v.value.to_owned();
            
            if Self::expire(v) { self.data.remove(key); None }
            else { Some(ret_value) }
        } else {
            None
        }
    }

    fn set(&mut self, key: &str, value: String, expiration: KeyExpiration) { 
        self.data.insert(key.to_owned(), StoredValue { value, access_count: 0, expiration, created_at: Instant::now() }); 
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_core::{OsRng, RngCore};

    #[test]
    fn can_set_and_get() {
        let mut adapter = InMemoryHash::new();

        assert_eq!(None, adapter.get("hello"));
        adapter.set("hello", "world".to_owned(), KeyExpiration::Never);
        assert_eq!(Some("world".to_owned()), adapter.get("hello"));
    }

    #[test]
    fn can_expire_key_after_use() {
        let mut adapter = InMemoryHash::new();
        let times = (OsRng.next_u32() % 250) + 1;

        adapter.set("hello", "world".to_owned(), KeyExpiration::AfterUse(times));

        for _ in 0..times {
            assert_eq!(Some("world".to_owned()), adapter.get("hello")); //Value exists for x # of times
        }
        assert_eq!(None, adapter.get("hello"));                         //Value doesn't exist anymore
    }

    #[test]
    fn can_expire_key_after_time() {
        let mut adapter = InMemoryHash::new();

        adapter.set("hello", "world".to_owned(), KeyExpiration::AfterTime(Duration::from_secs(500)));
        
        assert_eq!(Some("world".to_owned()), adapter.get("hello"));                         //Value exists
        adapter.data.get_mut("hello").map(|v| v.created_at -= Duration::from_secs(500));    //Adjust time
        assert_eq!(None, adapter.get("hello"));                                             //Value doesn't exist anymore
    }
}