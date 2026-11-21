// cryptography module (port of go-sdk/cryptography)

pub mod aes256gcm;
pub mod argon;
pub mod bcrypt;
pub mod sha256;

use crate::errors::SdkError;

/// One-way (non-reversible) cryptographic operations.
///
/// Implementors: SHA-256 (digest/MAC), Bcrypt, Argon2.
/// Note: these are interchangeable by type only; choose the right
/// algorithm for the job (e.g. do not use a plain digest for passwords).
pub trait OneWay: Send + Sync + std::fmt::Debug {
    fn hash(&self, data: &[u8]) -> Result<Vec<u8>, SdkError>;
    fn verify(&self, data: &[u8], hashed: &[u8]) -> Result<(), SdkError>;
}

/// Reversible (two-way) cryptographic operations.
///
/// Implementor: AES-256-GCM.
pub trait TwoWay: Send + Sync + std::fmt::Debug {
    fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, SdkError>;
    fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, SdkError>;
}

// Shared helpers used by multiple backends.
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        write!(s, "{:02x}", b).unwrap();
    }
    s
}

pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err("invalid hex length".to_string());
    }
    let mut out = Vec::with_capacity(s.len() / 2);
    for i in (0..s.len()).step_by(2) {
        out.push(u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())?);
    }
    Ok(out)
}

// Re-exports.
pub use aes256gcm::new_aes256gcm;
pub use argon::{Argon2Config, new_argon2, new_argon2_with_config};
pub use bcrypt::{BcryptConfig, BCRYPT_DEFAULT_COST, BCRYPT_MAX_COST, BCRYPT_MIN_COST, bcrypt_cost_from_hash, new_bcrypt, new_bcrypt_with_config};
pub use sha256::{new_sha256, new_sha256_with_key};

#[cfg(test)]
mod sha256_test;
#[cfg(test)]
mod bcrypt_test;
#[cfg(test)]
mod argon_test;
#[cfg(test)]
mod aes256gcm_test;