use aes_gcm_siv::{Aes256GcmSiv, KeyInit};
use aes_gcm_siv::aead::Aead;
use aes_gcm_siv::aead::generic_array::GenericArray;

use uuid::Uuid;
use std::convert::TryInto;


pub trait Tokens {
    fn new(key: &str, iv: &str) -> Self;

    fn new_random() -> Self;

    fn encrypt(&self, plaintext: &str) -> String;

    fn decrypt(&self, ciphertext: &str) -> Result<String, String>;
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
        let key_input = Uuid::new_v4().as_simple().to_string();
        let nonce_input = Uuid::new_v4().as_simple().to_string();

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

    /// Decrypts a hex-encoded ciphertext produced by [`encrypt`](Self::encrypt).
    ///
    /// Returns `Err` instead of panicking for malformed/tampered input, since
    /// `ciphertext` is typically attacker-controlled (e.g. a token read from
    /// a request query parameter) and any of hex-decoding, AEAD
    /// authentication, or UTF-8 validation can fail on garbage input.
    pub fn decrypt(&self, ciphertext: &str) -> Result<String, String> {
        let cipher = Aes256GcmSiv::new(GenericArray::from_slice(&self.key));
        let encrypted_data: Vec<u8> = hex::decode(ciphertext)
            .map_err(|e| format!("Invalid token encoding: {}", e))?;
        let plaintext = cipher.decrypt(GenericArray::from_slice(&self.nonce), encrypted_data.as_slice())
            .map_err(|_| String::from("Failed to decrypt token"))?;
        String::from_utf8(plaintext).map_err(|e| format!("Decrypted token is not valid UTF-8: {}", e))
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
    use aes_gcm_siv::{Key, Nonce};

    #[test]
    fn encrypt_decrypt_new_random() {
        let plaintext = "Hello world!";
        let token_service = TokenService::new_random();

        let encrypted = token_service.encrypt(plaintext);
        let decrypted = token_service.decrypt(&encrypted).expect("test fixture: decrypt should succeed");

        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn encrypt_decrypt_new() {
        let plaintext = "Hello world!";
        let token_service = TokenService::new("some key", "some iv");

        let encrypted = token_service.encrypt(plaintext);
        let decrypted = token_service.decrypt(&encrypted).expect("test fixture: decrypt should succeed");

        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn decrypt_rejects_non_hex_input_instead_of_panicking() {
        let token_service = TokenService::new_random();

        assert!(token_service.decrypt("not-valid-hex!!").is_err());
    }

    #[test]
    fn decrypt_rejects_tampered_ciphertext_instead_of_panicking() {
        let token_service = TokenService::new_random();
        let encrypted = token_service.encrypt("Hello world!");

        let mut tampered = hex::decode(&encrypted).expect("test fixture: should be valid hex");
        if let Some(first_byte) = tampered.first_mut() {
            *first_byte ^= 0xFF;
        }
        let tampered_hex = hex::encode(tampered);

        assert!(token_service.decrypt(&tampered_hex).is_err());
    }

    #[test]
    fn decrypt_rejects_arbitrary_hex_garbage_instead_of_panicking() {
        let token_service = TokenService::new_random();

        assert!(token_service.decrypt("deadbeef").is_err());
    }

    #[test]
    fn test_raw() {
        let key = GenericArray::from_slice(b"000102030405060708090a0b0c0d0e0f");
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
