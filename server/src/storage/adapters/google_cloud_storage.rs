use super::*;
use crate::props::GCloudProps;
use cloud_storage::Object;
use serde::{Serialize, Deserialize};
use chrono::{Utc};

pub struct CloudStorage {
    bucket: String,
}

#[derive(Serialize, Deserialize, std::fmt::Debug)]
struct StoredData {
    key: String,
    value: String,
    ttl: Duration,
    created_at: i64
}

impl CloudStorage {
    pub fn new(props: &GCloudProps) -> Self {
        std::env::set_var("SERVICE_ACCOUNT", &props.storage_sa);
        CloudStorage {
            bucket: props.storage_bucket.to_owned()
        }
    }

    fn expire(val: &StoredData) -> bool {
        Utc::now().timestamp() - val.created_at >= val.ttl.as_secs() as i64
    }
}

impl StorageAdapter for CloudStorage {
    fn prepare(&mut self, key: &str, value: String, ttl: Duration) -> Result<Bytes, Box<dyn Error>> {
        let value = StoredData { key: key.to_owned(), value, ttl, created_at: Utc::now().timestamp() };
        Ok(serde_json::to_vec(&value)?)
    }

    fn save(&mut self, key: &str, value: Bytes) -> Result<(), Box<dyn Error>>{ 
        Object::create_sync(&self.bucket, value, key, "text/plain")?;
        Ok(())
    }

    fn get(&mut self, key: &str) -> Result<Bytes, Box<dyn Error>> {
        let data = Object::download_sync(&self.bucket, key)?;
        Ok(data)
    }

    fn extract(&mut self, value: Bytes) -> Result<String, Box<dyn Error>> {
        let val: StoredData = serde_json::from_slice(&value[..])?;
        let ret_value = val.value.to_owned();
        
        if Self::expire(&val) { self.delete(&val.key)?; Err("Value for key has expired".into()) }
        else { Ok(ret_value) }
    }

    fn delete(&mut self, key: &str) -> Result<(), Box<dyn Error>> {
        Object::delete_sync(&self.bucket, key)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn can_set_get_and_delete() {
        let mut adapter = CloudStorage::new(&crate::props::GCloudProps::load());

        let data = adapter.get_and_extract("test_key").ok();
        assert_eq!(None, data);

        adapter.prepare_and_save("test_key", String::from("test_data"), Duration::from_secs(60)).unwrap();
        
        let data = adapter.get_and_extract("test_key").unwrap();
        assert_eq!("test_data", &data);

        adapter.delete("test_key").unwrap();
        
        let data = adapter.get_and_extract("test_key").ok();
        assert_eq!(None, data);
    }

    #[test]
    #[ignore]
    fn can_expire_key() {
        let mut adapter = CloudStorage::new(&crate::props::GCloudProps::load());
        let key = "hello".to_owned();
        let value = "world".to_owned();
        let ttl = Duration::from_secs(500);
        let created_at = Utc::now().timestamp();

        let stored = StoredData { key: key.clone(), value: value.clone(), ttl: ttl.clone(), created_at };
        adapter.save(&key, serde_json::to_vec(&stored).unwrap()).unwrap();
        assert_eq!(Some("world".to_owned()), adapter.get_and_extract("hello").ok());                         //Value exists

        
        let stored = StoredData { key: key.clone(), value: value.clone(), ttl: ttl.clone(), created_at: created_at - 500 };
        adapter.save(&key, serde_json::to_vec(&stored).unwrap()).unwrap();
        assert_eq!(true, adapter.get_and_extract("hello").is_err());                             //Value doesn't exist anymore
    }
}