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

impl<A: StorageAdapter + ?Sized> Storage<A> {
    pub fn save(&mut self, value: String, allowed_attempts: Option<u32>, ttl: Duration) -> Result<String, Box<dyn Error>> {
        let key = self.new_key()?;
        let data = self.adapter.prepare(&key, value, ttl)?;
        self.adapter.save_encrypted(&key, &StorageItem::new(data, allowed_attempts))?;
        Ok(key)
    }

    pub fn get(&mut self, key: &String) -> Result<String, Box<dyn Error>> {
        let mut item = self.adapter.get_encrypted(key)?;
        self.update_access(key, &mut item)?;
        self.adapter.extract(item.data)
    }

    fn update_access(&mut self, key: &str, item: &mut StorageItem) -> Result<(), Box<dyn Error>> {
        if let Some(max_access) = item.max_access {
            item.access_count += 1;
            if item.access_count > max_access {
                self.adapter.delete(key)?;
                return Err("Access limit exceeded!".into())
            } else {
                self.adapter.save_encrypted(key, item)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockAdapter {
        ret: Vec<Result<Bytes, Box<dyn Error>>>,
        saves: Vec<(String, Bytes)>,
        deletes: Vec<String>
    }

    impl StorageAdapter for MockAdapter {
        fn prepare(&mut self, _: &str, value: String, _: Duration) -> Result<Bytes, Box<dyn Error>> { Ok(value.into_bytes()) }
    
        fn save(&mut self, key: &str, value: Bytes) -> Result<(), Box<dyn Error>> { self.saves.push((key.to_owned(), value)); Ok(()) }
        
        fn get(&mut self, _: &str) -> Result<Bytes, Box<dyn Error>> { self.ret.remove(0) }
    
        fn extract(&mut self, value: Bytes) -> Result<String, Box<dyn Error>> { Ok(String::from_utf8(value)?) }

        fn delete(&mut self, key: &str) -> Result<(), Box<dyn Error>> { self.deletes.push(key.to_owned()); Ok(()) }
    }

    #[test]
    fn update_item_after_access() {
        let key = String::from("my_key");
        let mut item = StorageItem { data: String::from("some_value").into_bytes(), access_count: 0, max_access: Some(1) };
        let mut storage = Storage::new(MockAdapter { ret: vec![], saves: vec![], deletes: vec![] });
        
        let res = storage.update_access(&key, &mut item);
        assert_eq!(true, res.is_ok());

        let saves = &storage.adapter.saves;
        assert_eq!(1, saves.len());
        assert_eq!(&key, &saves.get(0).unwrap().0);
    }

    #[test]
    fn reject_access_after_max_reached() {
        let key = String::from("my_key");
        let mut item = StorageItem { data: String::from("some_value").into_bytes(), access_count: 1, max_access: Some(1) };
        let mut storage = Storage::new(MockAdapter { ret: vec![], saves: vec![], deletes: vec![] });
        
        let res = storage.update_access(&key, &mut item);
        assert_eq!(false, res.is_ok());

        let deletes = &storage.adapter.deletes;
        assert_eq!(1, deletes.len());
        assert_eq!(Some(&key), deletes.get(0));
    }
}