use super::*;

pub(super) trait KeyGenerator {
    fn new_key(&mut self) -> Result<String, Box<dyn Error>>;
}

impl<A: StorageAdapter + ?Sized> KeyGenerator for Storage<A> {
    fn new_key(&mut self) -> Result<String, Box<dyn Error>> {
        for _ in 0..100 {
            let len = ((OsRng.next_u64() % 3) + 2) as usize;
            let mut bytes = vec![0u8; len];
            OsRng.fill_bytes(&mut bytes);
            let key = encode(bytes, URL_SAFE_NO_PAD);

            if let Ok(_) = self.adapter.get(&key) {
                continue;
            } else {
                return Ok(key)
            }
        }
        return Err("Unable to find key".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockKeyAdapter {
        collisions: Vec<Result<Bytes, Box<dyn Error>>>
    }

    impl StorageAdapter for MockKeyAdapter {
        fn prepare(&mut self, _: &str, _: String, _: Duration) -> Result<Bytes, Box<dyn Error>> { panic!() }
    
        fn save(&mut self, _: &str, _: Bytes) -> Result<(), Box<dyn Error>> { panic!() }
        
        fn get(&mut self, _: &str) -> Result<Bytes, Box<dyn Error>> { self.collisions.remove(0) }
    
        fn extract(&mut self, _: Bytes) -> Result<String, Box<dyn Error>> { panic!() }
    
        fn delete(&mut self, _: &str) -> Result<(), Box<dyn Error>> { panic!() }
    }

    #[test]
    fn can_generate_keys() {
        for i in 0..99 {
            let mut collisions: Vec<Result<Bytes, Box<dyn Error>>> = (0..i).map(|_| Ok(vec![])).collect();
            collisions.push(Err("Key not found".into()));
            let mut storage = Storage::new(MockKeyAdapter { collisions });
            let key = storage.new_key();
            println!("Key generated: {:?}", &key);
            assert_eq!(true, key.is_ok())
        } 
    }

    #[test]
    fn error_if_all_collisions() {
        let mut storage = Storage::new(MockKeyAdapter { collisions: (0..100).map(|_| Ok(vec![])).collect() });
        assert_eq!(false, storage.new_key().is_ok())
    }
}