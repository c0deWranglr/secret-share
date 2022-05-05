use super::*;

#[async_trait::async_trait]
pub(super) trait KeyGenerator {
    async fn new_key(&mut self) -> Result<String, Box<dyn Error>>;
}

#[async_trait::async_trait]
impl<A: StorageAdapter + ?Sized + Send> KeyGenerator for Storage<A> {
    async fn new_key(&mut self) -> Result<String, Box<dyn Error>> {
        for _ in 0..100 {
            let len = ((OsRng.next_u64() % 3) + 2) as usize;
            let mut bytes = vec![0u8; len];
            OsRng.fill_bytes(&mut bytes);
            let key = encode(bytes, URL_SAFE_NO_PAD);

            if let Ok(_) = self.adapter.get(&key).await {
                continue;
            } else {
                return Ok(key);
            }
        }
        return Err("Unable to find key".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockKeyAdapter {
        collisions: Vec<bool>
    }

    #[async_trait::async_trait]
    impl StorageAdapter for MockKeyAdapter {
        async fn prepare(&mut self, _: &str, _: String, _: Duration) -> Result<Bytes, Box<dyn Error>> { panic!() }
    
        async fn save(&mut self, _: &str, _: Bytes) -> Result<(), Box<dyn Error>> { panic!() }
        
        async fn get(&mut self, _: &str) -> Result<Bytes, Box<dyn Error>> { 
            if self.collisions.remove(0) {
                Ok(vec![])
            } else {
                Err("Not Found!".into())
            }
        }
    
        async fn extract(&mut self, _: Bytes) -> Result<String, Box<dyn Error>> { panic!() }
    
        async fn delete(&mut self, _: &str) -> Result<(), Box<dyn Error>> { panic!() }
    }

    #[actix_web::test]
    async fn can_generate_keys() {
        for i in 0..99 {
            let mut collisions: Vec<bool> = (0..i).map(|_| true).collect();
            collisions.push(false);
            let mut storage = Storage::new(MockKeyAdapter { collisions });
            let key = storage.new_key().await;
            println!("Key generated: {:?}", &key);
            assert_eq!(true, key.is_ok())
        } 
    }

    #[actix_web::test]
    async fn error_if_all_collisions() {
        let mut storage = Storage::new(MockKeyAdapter { collisions: (0..100).map(|_| true).collect() });
        assert_eq!(false, storage.new_key().await.is_ok())
    }
}