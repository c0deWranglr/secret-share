use crate::props::PROPS;
use std::result::Result;
use std::error::Error;
use rand_core::{RngCore, OsRng};
use aes::{Aes256};
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;

pub type Bytes = Vec<u8>;
const IV_SIZE: usize = 16; //bytes -> 128 bit IV

pub struct Cipher;

impl Cipher {
    fn iv() -> Bytes {
        let mut iv = [0u8; IV_SIZE].to_vec();
        OsRng.fill_bytes(&mut iv);
        iv
    }

    fn extract_iv(data: &Bytes) -> Bytes {
        data[data.len()-IV_SIZE..].to_vec()
    }

    pub fn encrypt(data: &Bytes) -> Result<Bytes, Box<dyn Error>> {
        if data.is_empty() { return Err("No data provided".into()) }

        let mut iv = Self::iv();
        let cipher = Cbc::<Aes256, Pkcs7>::new_from_slices(PROPS.encryption_key.as_bytes(), &iv)?;
        let mut encrypted = cipher.encrypt_vec(data);
        encrypted.append(&mut iv);
        Ok(encrypted)
    }
    
    pub fn decrypt(data: &Bytes) -> Result<Bytes, Box<dyn Error>> {
        if data.len() < IV_SIZE { return Err(format!("Data must be at least {} bytes long", IV_SIZE).into()) }
    
        let iv = Self::extract_iv(data);
        let cipher = Cbc::<Aes256, Pkcs7>::new_from_slices(PROPS.encryption_key.as_bytes(), &iv)?;
        let decrypted = cipher.decrypt_vec(&data[..data.len()-IV_SIZE].to_vec())?;
        Ok(decrypted)
    }
}

pub trait AsBytes {
    fn from_bytes(bytes: Bytes) -> Self;
    fn as_bytes(&self) -> Bytes;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iv_is_probably_random() {
        let mut set = std::collections::HashSet::<String>::new();
        for _ in 0..100 {
            let bytes = Cipher::iv();
            let hexs: Vec<String> = bytes.iter().map(|b| format!("{:02X}", b)).collect();
            let hex = hexs.join(" ");
            println!("{}", &hex);
            set.insert(hex);
        }
        assert_eq!(set.len(), 100);
    }

    #[test]
    fn can_encrypt_and_decrypt() {
        let content = String::from("{ \"key\": \"value\", \"another_key\": 10 }");
        let data = content.as_bytes().to_vec();
        println!("Content: {:?}", content);

        let encrypted = Cipher::encrypt(&data).expect("Failed to encrypt!");
        println!("Encrypted: {:?}", encrypted.clone());
        assert_ne!(&encrypted, &data);

        let decrypted = Cipher::decrypt(&encrypted).expect("Failed to decrypt!");
        println!("Decrypted: {:?}", String::from_utf8(decrypted.clone()).unwrap());
        assert_eq!(&decrypted, &data);
    }

    #[test]
    fn encrypt_empty_vec() {
        let encrypted = Cipher::encrypt(&vec![]).unwrap_err();
        println!("Encrypt: {:?}", encrypted);
    }

    #[test]
    fn decrypt_empty_vec() {
        let decrypted = Cipher::decrypt(&vec![]).unwrap_err();
        println!("Decrypt: {:?}", decrypted);
    }

    #[test]
    fn decrypt_unencrypted_vec() {
        let content = String::from("{ \"key\": \"value\", \"another_key\": 10 }");
        let data = content.as_bytes().to_vec();
        
        let decrypted = Cipher::decrypt(&data).unwrap_err();
        println!("Decrypt: {:?}", decrypted);
    }
}