pub mod cipher;
pub mod adapters;
mod encryption;
mod key;
mod models;

use models::*;
use encryption::Encrypted;
use key::KeyGenerator;
use crate::storage::adapters::StorageAdapter;
use cipher::{Bytes, Cipher};

use std::result::Result;
use std::error::Error;
use std::time::Duration;
use rand_core::{OsRng, RngCore};
use base64::{encode_config as encode, URL_SAFE_NO_PAD};
use serde::{Serialize, Deserialize};

pub struct Storage<A: StorageAdapter + ?Sized> {
    adapter: Encrypted<A>
}

impl<A: StorageAdapter> Storage<A> {
    pub fn new(adapter: A) -> Storage<A> {
        return Storage { adapter: Encrypted::new(adapter) }
    }
}

impl<A: StorageAdapter + ?Sized + Send> Storage<A> {
    pub async fn save(&mut self, value: String, allowed_attempts: Option<u32>, ttl: Duration) -> Result<String, Box<dyn Error>> {
        let key = self.new_key().await?;
        let data = self.adapter.prepare(&key, value, ttl).await?;
        self.adapter.save_encrypted(&key, &StorageItem::new(data, allowed_attempts)).await?;
        Ok(key)
    }

    pub async fn get(&mut self, key: &String) -> Result<String, Box<dyn Error>> {
        let mut item = self.adapter.get_encrypted(key).await?;
        self.update_access(key, &mut item).await?;
        self.adapter.extract(item.data).await
    }

    async fn update_access(&mut self, key: &str, item: &mut StorageItem) -> Result<(), Box<dyn Error>> {
        if let Some(max_access) = item.max_access {
            item.access_count += 1;
            if item.access_count > max_access {
                self.adapter.delete(key).await?;
                return Err("Access limit exceeded!".into())
            } else {
                self.adapter.save_encrypted(key, item).await?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct MockAdapter {
        saves: Vec<(String, Bytes)>,
        deletes: Vec<String>
    }

    #[async_trait::async_trait]
    impl StorageAdapter for MockAdapter {
        async fn prepare(&mut self, _: &str, value: String, _: Duration) -> Result<Bytes, Box<dyn Error>> { Ok(value.into_bytes()) }
    
        async fn save(&mut self, key: &str, value: Bytes) -> Result<(), Box<dyn Error>> { self.saves.push((key.to_owned(), value)); Ok(()) }
        
        async fn get(&mut self, _: &str) -> Result<Bytes, Box<dyn Error>> { panic!() }
    
        async fn extract(&mut self, value: Bytes) -> Result<String, Box<dyn Error>> { Ok(String::from_utf8(value)?) }

        async fn delete(&mut self, key: &str) -> Result<(), Box<dyn Error>> { self.deletes.push(key.to_owned()); Ok(())}
    }

    #[actix_web::test]
    async fn update_item_after_access() {
        let key = String::from("my_key");
        let mut item = StorageItem { data: String::from("some_value").into_bytes(), access_count: 0, max_access: Some(1) };
        let mut storage = Storage::new(MockAdapter { saves: vec![], deletes: vec![] });
        
        let res = storage.update_access(&key, &mut item).await;
        assert_eq!(true, res.is_ok());

        let saves = &storage.adapter.saves;
        assert_eq!(1, saves.len());
        assert_eq!(&key, &saves.get(0).unwrap().0);
    }

    #[actix_web::test]
    async fn reject_access_after_max_reached() {
        let key = String::from("my_key");
        let mut item = StorageItem { data: String::from("some_value").into_bytes(), access_count: 1, max_access: Some(1) };
        let mut storage = Storage::new(MockAdapter { saves: vec![], deletes: vec![] });
        
        let res = storage.update_access(&key, &mut item).await;
        assert_eq!(false, res.is_ok());

        let deletes = &storage.adapter.deletes;
        assert_eq!(1, deletes.len());
        assert_eq!(Some(&key), deletes.get(0));
    }
}