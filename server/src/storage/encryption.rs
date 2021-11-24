use super::*;
use std::ops::{Deref, DerefMut};

pub(super) struct Encrypted<A: StorageAdapter + ?Sized> {
    adapter: Box<A>
}

impl<A: StorageAdapter> Encrypted<A> {
    pub(super) fn new(adapter: A) -> Encrypted<A> {
        Encrypted { adapter: Box::new(adapter) }
    }
}

impl<A: StorageAdapter + ?Sized> Deref for Encrypted<A> {
    type Target = A;

    fn deref(&self) -> &Self::Target {
        &self.adapter
    }
}

impl<A: StorageAdapter + ?Sized> DerefMut for Encrypted<A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.adapter
    }
}

impl<A: StorageAdapter + ?Sized> Encrypted<A> {
    pub fn save_encrypted(&mut self, key: &str, item: &StorageItem) -> Result<(), Box<dyn Error>> {
        let bytes = serde_json::to_vec(item)?;
        let encrypted = Cipher::encrypt(&bytes)?;
        self.adapter.save(&key, encrypted)
    }

    pub fn get_encrypted(&mut self, key: &str) -> Result<StorageItem, Box<dyn Error>> {
        let encrypted = self.adapter.get(key)?;
        let decrypted = Cipher::decrypt(&encrypted)?;
        let item = serde_json::from_slice::<StorageItem>(&decrypted[..])?;
        Ok(item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockCryptoAdapter {
        saves: Vec<(String, Bytes)>,
        gets: Vec<Result<Bytes, Box<dyn Error>>>
    }

    impl StorageAdapter for MockCryptoAdapter {
        fn prepare(&mut self, _: &str, _: String, _: Duration) -> Result<Bytes, Box<dyn Error>> { panic!() }
    
        fn save(&mut self, key: &str, value: Bytes) -> Result<(), Box<dyn Error>> { self.saves.push((key.to_owned(), value)); Ok(()) }
        
        fn get(&mut self, _: &str) -> Result<Bytes, Box<dyn Error>> { self.gets.remove(0) }
    
        fn extract(&mut self, _: Bytes) -> Result<String, Box<dyn Error>> { panic!() }
    
        fn delete(&mut self, _: &str) -> Result<(), Box<dyn Error>> { panic!() }
    }

    #[test]
    fn encrypted_on_save() {
        let key = "my_key".to_owned();
        let item = StorageItem::new("hello, world".as_bytes().to_owned(), None);

        let mut adapter = Encrypted::new(MockCryptoAdapter { saves: vec![], gets: vec![] });
        let res = adapter.save_encrypted(&key, &item);
        assert_eq!(true, res.is_ok());

        let saves = &(adapter.adapter.as_ref() as &MockCryptoAdapter).saves;
        assert_eq!(1, saves.len());
        assert_eq!(key, saves.get(0).unwrap().0);
        assert_eq!(&item, &{
            serde_json::from_slice(&Cipher::decrypt(&saves.get(0).unwrap().1).unwrap()[..]).unwrap()
        });
    }

    #[test]
    fn decrypted_on_get() {
        let key = "my_key".to_owned();
        let item = StorageItem::new("hello, world".as_bytes().to_owned(), None);
        let encrypted = Cipher::encrypt(&serde_json::to_vec(&item).unwrap()).unwrap();

        let mut adapter = Encrypted::new(MockCryptoAdapter { saves: vec![], gets: vec![Ok(encrypted)] });
        let res = adapter.get_encrypted(&key);
        assert_eq!(true, res.is_ok());
        assert_eq!(&item, &res.unwrap());
    }
}