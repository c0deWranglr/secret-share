use super::*;
use std::collections::HashMap;
use chrono::{Utc};

#[derive(Serialize, Deserialize)]
struct StoredValue {
    key: String,
    value: String,
    ttl: Duration,
    created_at: i64
}

pub struct InMemoryHash {
    data: HashMap<String, Bytes>
}

impl InMemoryHash {
    pub fn new() -> InMemoryHash {
        return InMemoryHash { data: HashMap::new() }
    }

    fn expire(val: &StoredValue) -> bool {
        Utc::now().timestamp() - val.created_at >= val.ttl.as_secs() as i64
    }
}

#[async_trait::async_trait]
impl StorageAdapter for InMemoryHash {

    async fn prepare(&mut self, key: &str, value: String, ttl: Duration) -> Result<Bytes, Box<dyn Error>> {
        let value = StoredValue { key: key.to_owned(), value, ttl, created_at: Utc::now().timestamp() };
        Ok(serde_json::to_vec(&value)?)
    }

    async fn save(&mut self, key: &str, value: Bytes) -> Result<(), Box<dyn Error>>{ 
        self.data.insert(key.to_owned(), value);
        Ok(())
    }

    async fn get(&mut self, key: &str) -> Result<Bytes, Box<dyn Error>> { 
        self.data.get(key).map(|value| value.to_owned()).ok_or("Key not found".into())
    }

    async fn extract(&mut self, value: Bytes) -> Result<String, Box<dyn Error>> {
        let val: StoredValue = serde_json::from_slice(&value[..])?;
        let ret_value = val.value.to_owned();
        
        if Self::expire(&val) { self.delete(&val.key).await?; Err("Value for key has expired".into()) }
        else { Ok(ret_value) }
    }

    async fn delete(&mut self, key: &str) -> Result<(), Box<dyn Error>> {
        self.data.remove(key);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn can_set_get_and_delete() {
        let mut adapter = InMemoryHash::new();

        // Get (pre save)
        assert_eq!(true, adapter.get("hello").await.is_err());

        // Save
        adapter.prepare_and_save("hello", "world".to_owned(), Duration::from_secs(10)).await.unwrap();
        
        // Get (post save)
        let data = adapter.get_and_extract("hello").await;
        assert_eq!(Some("world".to_owned()), data.ok());

        // Delete
        adapter.delete("hello").await.unwrap();

        // Get (post delete)
        let data = adapter.get_and_extract("hello").await;
        assert_eq!(None, data.ok());
    }

    #[actix_web::test]
    async fn can_expire_key() {
        let mut adapter = InMemoryHash::new();
        let key = "hello".to_owned();
        let value = "world".to_owned();
        let ttl = Duration::from_secs(500);
        let created_at = Utc::now().timestamp();

        let stored = StoredValue { key: key.clone(), value: value.clone(), ttl: ttl.clone(), created_at };
        adapter.save(&key, serde_json::to_vec(&stored).unwrap()).await.unwrap();
        assert_eq!(Some("world".to_owned()), adapter.get_and_extract("hello").await.ok());                         //Value exists

        
        let stored = StoredValue { key: key.clone(), value: value.clone(), ttl: ttl.clone(), created_at: created_at - 500 };
        adapter.save(&key, serde_json::to_vec(&stored).unwrap()).await.unwrap();
        assert_eq!(true, adapter.get_and_extract("hello").await.is_err());                             //Value doesn't exist anymore`
    }
}