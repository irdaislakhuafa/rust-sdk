use crate::codes::Code;
use crate::cryptography::OneWay;
use crate::errors::{new_with_code, SdkError};
use bcrypt as bcrypt_crate;

pub const BCRYPT_DEFAULT_COST: u32 = bcrypt_crate::DEFAULT_COST;
pub const BCRYPT_MAX_COST: u32 = 31;
pub const BCRYPT_MIN_COST: u32 = 4;

#[derive(Debug, Clone, Copy)]
pub struct BcryptConfig {
    pub cost: u32,
}

impl Default for BcryptConfig {
    fn default() -> Self {
        Self {
            cost: BCRYPT_DEFAULT_COST,
        }
    }
}

#[derive(Debug)]
pub(crate) struct BcryptImpl {
    cost: u32,
}

pub fn new_bcrypt() -> Box<dyn OneWay> {
    new_bcrypt_with_config(BcryptConfig::default())
}

pub fn new_bcrypt_with_config(config: BcryptConfig) -> Box<dyn OneWay> {
    Box::new(BcryptImpl { cost: config.cost })
}

pub fn bcrypt_cost_from_hash(hashed_text: &[u8]) -> Result<u32, SdkError> {
    let hashed = std::str::from_utf8(hashed_text)
        .map_err(|e| new_with_code(Code::Bcrypt, e.to_string()))?;
    let parts: Vec<&str> = hashed.split('$').collect();
    if parts.len() < 3 {
        return Err(new_with_code(Code::Bcrypt, "invalid hash"));
    }
    parts[2]
        .parse::<u32>()
        .map_err(|e| new_with_code(Code::Bcrypt, e.to_string()))
}

impl OneWay for BcryptImpl {
    fn hash(&self, plain_text: &[u8]) -> Result<Vec<u8>, SdkError> {
        bcrypt_crate::hash(plain_text, self.cost)
            .map(|s| s.into_bytes())
            .map_err(|e| new_with_code(Code::Bcrypt, e.to_string()))
    }

    fn verify(&self, plain_text: &[u8], hashed_text: &[u8]) -> Result<(), SdkError> {
        let hashed = std::str::from_utf8(hashed_text)
            .map_err(|e| new_with_code(Code::Bcrypt, e.to_string()))?;
        let ok = bcrypt_crate::verify(plain_text, hashed)
            .map_err(|e| new_with_code(Code::Bcrypt, e.to_string()))?;
        if !ok {
            return Err(new_with_code(Code::Bcrypt, "plain text with hashed text not match"));
        }
        Ok(())
    }
}