use crate::codes::Code;
use crate::cryptography::{OneWay, hex_encode};
use crate::errors::{new_with_code, SdkError};
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub(crate) struct Sha256Impl {
    key: Option<Vec<u8>>,
}

pub fn new_sha256() -> Box<dyn OneWay> {
    Box::new(Sha256Impl { key: None })
}

pub fn new_sha256_with_key(key: &[u8]) -> Box<dyn OneWay> {
    Box::new(Sha256Impl {
        key: Some(key.to_vec()),
    })
}

impl OneWay for Sha256Impl {
    fn hash(&self, data: &[u8]) -> Result<Vec<u8>, SdkError> {
        if let Some(key) = &self.key {
            type HmacSha256 = Hmac<Sha256>;
            let mut mac = HmacSha256::new_from_slice(key)
                .map_err(|e| new_with_code(Code::InvalidValue, e.to_string()))?;
            mac.update(data);
            Ok(mac.finalize().into_bytes().as_slice().to_vec())
        } else {
            let mut hasher = Sha256::new();
            hasher.update(data);
            Ok(hasher.finalize().as_slice().to_vec())
        }
    }

    fn verify(&self, data: &[u8], hashed: &[u8]) -> Result<(), SdkError> {
        let computed = self.hash(data)?;
        if computed == hashed {
            Ok(())
        } else {
            Err(new_with_code(Code::InvalidValue, "sha256 digest mismatch"))
        }
    }
}