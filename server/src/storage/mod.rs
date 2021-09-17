pub mod adapters;

use crate::storage::adapters::StorageAdapter;

use rand_core::{OsRng, RngCore};
use base64::{encode_config as encode, URL_SAFE_NO_PAD};


pub struct Storage<A: StorageAdapter + ?Sized> {
    adapter: Box<A>
}

impl<A: StorageAdapter> Storage<A> {
    pub fn new(adapter: A) -> Storage<A> {
        return Storage { adapter: Box::new(adapter) }
    }
}

impl<A: StorageAdapter + ?Sized> Storage<A> {
    pub fn save(&mut self, value: String) -> Option<String> {
        self.save_and_expire(value, adapters::KeyExpiration::Never)
    }

    pub fn save_and_expire(&mut self, value: String, expiration: adapters::KeyExpiration) -> Option<String> {
        let mut bytes = [0u8; 256];
        OsRng.fill_bytes(&mut bytes);
        let long_key = encode(bytes, URL_SAFE_NO_PAD);
        
        let mut len: usize = 4;
        loop {
            if (long_key.len() as isize)-(len as isize) < 0 { return None; }

            println!("Trying length {}", len);
            let key = String::from(&long_key[long_key.len()-len..]);
            if let Some(_) = self.adapter.get(&key) {
                len += 1;
            } else {
                self.adapter.set(&key, value, expiration);
                return Some(key);
            }
        }
    }

    pub fn get(&mut self, key: &String) -> Option<String> {
        self.adapter.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockAdapter {
        ret: Vec<Option<String>>,
        saves: Vec<(String, String)>
    }

    impl StorageAdapter for MockAdapter {
        fn get(&mut self, _: &str) -> std::option::Option<std::string::String> { self.ret.remove(0) }
        fn set(&mut self, key: &str, value: std::string::String, _: adapters::KeyExpiration) { self.saves.push((key.to_owned(), value)) }
    }

    #[test]
    fn four_digit_key_saved_on_no_collision() {
        let mut storage = Storage::new(MockAdapter { ret: vec![None], saves: vec![] });
        let value = "some_value".to_owned();
        
        let key = storage.save(value.clone()).unwrap();
        println!("Key: {}", &key);
        assert_eq!(4, key.len());
        assert_eq!(vec![(key, value)], (storage.adapter.as_ref() as &MockAdapter).saves)
    }

    #[test]
    fn five_digit_key_saved_on_single_collision() {
        let mut storage = Storage::new(MockAdapter { ret: vec![Some("".to_owned()), None], saves: vec![] });
        let value = "some_value".to_owned();

        let key = storage.save(value.clone()).unwrap();
        println!("Key: {}", &key);
        assert_eq!(5, key.len());
        assert_eq!(vec![(key, value)], (storage.adapter.as_ref() as &MockAdapter).saves)
    }

    #[test]
    fn no_key_saved_on_full_collision() {
        let mut storage = Storage::new(MockAdapter { ret: vec![Some("".to_owned()); 512], saves: vec![] });
        let value = "some_value".to_owned();

        let key = storage.save(value);
        println!("Key: {:?}", &key);
        assert_eq!(None, key);
        assert_eq!(Vec::<(String, String)>::new(), (storage.adapter.as_ref() as &MockAdapter).saves)
    }
}