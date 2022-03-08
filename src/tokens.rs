use aes_gcm_siv::Aes256GcmSiv;
use aes_gcm_siv::aead::{Aead, NewAead};
use aes_gcm_siv::aead::generic_array::GenericArray;

use uuid::Uuid;
use std::convert::TryInto;


pub trait Tokens {
    fn new(key: &str, iv: &str) -> Self;

    fn new_random() -> Self;

    fn encrypt(&self, plaintext: &str) -> String;

    fn decrypt(&self, ciphertext: &str) -> String;
}


pub struct TokenService {
    key: [u8; 32],
    nonce: [u8; 12],
}

impl TokenService {
    pub fn new(key_input: & str, nonce_input: &str) -> Self {
        let key = TokenService::generate_hex(key_input, 32);
        let nonce = TokenService::generate_hex(nonce_input, 12);

        TokenService {
            key: key.try_into().unwrap(),
            nonce: nonce.try_into().unwrap()
        }
    }

    pub fn new_random() -> Self {
        let key_input = Uuid::new_v4().to_simple().to_string();
        let nonce_input = Uuid::new_v4().to_simple().to_string();

        let key = TokenService::generate_hex(&key_input, 32);
        let nonce = TokenService::generate_hex(&nonce_input, 12);

        TokenService {
            key: key.try_into().unwrap(),
            nonce: nonce.try_into().unwrap()
        }
    }

    pub fn encrypt(&self, plaintext: &str) -> String {
        let cipher = Aes256GcmSiv::new(GenericArray::from_slice(&self.key));
        let ciphertext = cipher.encrypt(GenericArray::from_slice(&self.nonce), plaintext.as_bytes())
            .expect("encryption failure!");
        hex::encode(&ciphertext)
    }

    pub fn decrypt(&self, ciphertext: &str) -> String {
        let cipher = Aes256GcmSiv::new(GenericArray::from_slice(&self.key));
        let encrypted_data: Vec<u8> = hex::decode(ciphertext).unwrap();
        let plaintext = cipher.decrypt(GenericArray::from_slice(&self.nonce), encrypted_data.as_slice())
            .expect("decryption failure!");
        String::from_utf8(plaintext).unwrap()
    }
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



#[cfg(test)]
mod token_service_tests {
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
        let key = Key::from_slice(b"000102030405060708090a0b0c0d0e0f");
        let nonce = Nonce::from_slice(b"f9fafbfcfdfe");

        let message = b"Hello world!";

        let cipher = Aes256GcmSiv::new(key);
        let ciphertext = cipher.encrypt(nonce, message.as_ref())
            .expect("encryption failure!");

        let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())
            .expect("decryption failure!");

        assert_eq!(plaintext, message);
    }

}
