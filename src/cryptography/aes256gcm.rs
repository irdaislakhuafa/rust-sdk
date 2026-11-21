use crate::codes::Code;
use crate::cryptography::{TwoWay, hex_decode, hex_encode};
use crate::errors::{new_with_code, SdkError};
use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit, Nonce};
use base64::Engine;
use rand::RngCore;

#[derive(Debug)]
pub(crate) struct Aes256GcmImpl {
    key: Vec<u8>,
}

pub fn new_aes256gcm(key: &[u8]) -> Result<Box<dyn TwoWay>, SdkError> {
    if key.len() != 32 {
        return Err(new_with_code(Code::InvalidValue, "invalid key length"));
    }
    Ok(Box::new(Aes256GcmImpl {
        key: key.to_vec(),
    }))
}

impl TwoWay for Aes256GcmImpl {
    fn encrypt(&self, text: &[u8]) -> Result<Vec<u8>, SdkError> {
        let cipher = Aes256Gcm::new_from_slice(&self.key)
            .map_err(|e| new_with_code(Code::InvalidValue, e.to_string()))?;
        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = cipher
            .encrypt(nonce, text)
            .map_err(|e| new_with_code(Code::InvalidValue, e.to_string()))?;
        let mut raw = nonce_bytes.to_vec();
        raw.extend_from_slice(&ciphertext);
        let hexed = hex_encode(&raw);
        let b64 = base64::engine::general_purpose::STANDARD.encode(hexed.as_bytes());
        Ok(b64.into_bytes())
    }

    fn decrypt(&self, text: &[u8]) -> Result<Vec<u8>, SdkError> {
        let b64 = std::str::from_utf8(text).map_err(|e| new_with_code(Code::InvalidValue, e.to_string()))?;
        let hexed_bytes = base64::engine::general_purpose::STANDARD
            .decode(b64)
            .map_err(|e| new_with_code(Code::InvalidValue, e.to_string()))?;
        let hexed = std::str::from_utf8(&hexed_bytes).map_err(|e| new_with_code(Code::InvalidValue, e.to_string()))?;
        let raw = hex_decode(hexed).map_err(|e| new_with_code(Code::InvalidValue, e.to_string()))?;

        let cipher = Aes256Gcm::new_from_slice(&self.key)
            .map_err(|e| new_with_code(Code::InvalidValue, e.to_string()))?;
        if raw.len() < 12 {
            return Err(new_with_code(Code::InvalidValue, "invalid ciphertext"));
        }
        let (nonce_bytes, ct) = raw.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        let plaintext = cipher
            .decrypt(nonce, ct)
            .map_err(|e| new_with_code(Code::Aes256GcmOpenError, e.to_string()))?;
        Ok(plaintext)
    }
}