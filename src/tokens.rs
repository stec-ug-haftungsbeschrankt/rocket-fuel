use uuid::Uuid;
use std::convert::TryInto;

use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;


pub trait Tokens {
    fn new(key: &str, iv: &str) -> Self;

    fn new_random() -> Self;

    fn encrypt(&self, plaintext: &str) -> String;

    fn decrypt(&self, ciphertext: &str) -> String;
}



pub struct TokenService {
    key: [u8; 16], 
    iv: [u8; 16]
}

impl TokenService {
    fn generate_hex(input: &str, count: usize) -> Vec<u8> {
        let mut result = Vec::new();
        let raw = input.as_bytes();

        for x in 0..count {
            let index = x % raw.len(); 
            let c = raw[index];
            result.push(c);
        }
        result
    }
}

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

impl Tokens for TokenService {
    
    fn new(key_input: &str, iv_input: &str) -> Self { 
        let key = TokenService::generate_hex(&key_input, 16);
        let iv = TokenService::generate_hex(&iv_input, 16);

        TokenService { 
            key: key.try_into().unwrap(), 
            iv: iv.try_into().unwrap(), 
        } 
    }

    fn new_random() -> Self { 
        let key_input = Uuid::new_v4().to_simple().to_string();
        let iv_input = Uuid::new_v4().to_simple().to_string();

        let key = TokenService::generate_hex(&key_input, 16);
        let iv = TokenService::generate_hex(&iv_input, 16);
 
        TokenService { 
            key: key.try_into().unwrap(), 
            iv: iv.try_into().unwrap(), 
        } 
    }
    
    fn encrypt(&self, plaintext: &str) -> String { 
        let cipher = Aes128Cbc::new_var(&self.key, &self.iv).unwrap();
        let ciphertext = cipher.encrypt_vec(plaintext.as_bytes());

        hex::encode(&ciphertext)
    }
    
    fn decrypt(&self, ciphertext: &str) -> String { 
        let cipher = Aes128Cbc::new_var(&self.key, &self.iv).unwrap();
        let encrypted_data: Box<[u8]> = hex::decode(ciphertext).unwrap().try_into().unwrap();
        let plaintext = cipher.decrypt_vec(&encrypted_data).unwrap();
        
        String::from_utf8(plaintext).unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_decrypt_new_random() {
        let plaintext = "Hello world!";
        let token_service = TokenService::new_random();

        let encrypted = token_service.encrypt(plaintext);
        let decrypted = token_service.decrypt(&encrypted);

        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn encrypt_decrypt_new() {
        let plaintext = "Hello world!";
        let token_service = TokenService::new("some key", "some iv");

        let encrypted = token_service.encrypt(plaintext);
        let decrypted = token_service.decrypt(&encrypted);

        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_raw() {
        let key = hex::decode("000102030405060708090a0b0c0d0e0f").unwrap();
        let iv = hex::decode("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff").unwrap();

        let plaintext = b"Hello world!";
 
        let cipher = Aes128Cbc::new_var(&key, &iv).unwrap();
        let ciphertext = cipher.encrypt_vec(plaintext);

        assert_eq!(ciphertext, hex::decode("1b7a4c403124ae2fb52bedc534d82fa8").unwrap());

        let cipher = Aes128Cbc::new_var(&key, &iv).unwrap();
        let decrypted_ciphertext = cipher.decrypt_vec(&ciphertext).unwrap();

        assert_eq!(decrypted_ciphertext, plaintext);
    }

}