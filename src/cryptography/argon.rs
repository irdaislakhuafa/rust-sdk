use crate::codes::Code;
use crate::cryptography::OneWay;
use crate::errors::{new_with_code, SdkError};
use argon2 as argon2_crate;
use base64::Engine;
use rand::RngCore;

#[derive(Debug, Clone, Copy)]
pub struct Argon2Config {
    pub memory: u32,
    pub iterations: u32,
    pub parallelism: u8,
    pub key_len: u32,
}

impl Default for Argon2Config {
    fn default() -> Self {
        Self {
            memory: 4 * 1024,
            iterations: 3,
            parallelism: 1,
            key_len: 32,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Argon2Impl {
    memory: u32,
    iterations: u32,
    parallelism: u8,
    key_len: u32,
}

pub fn new_argon2() -> Box<dyn OneWay> {
    new_argon2_with_config(Argon2Config::default())
}

pub fn new_argon2_with_config(config: Argon2Config) -> Box<dyn OneWay> {
    Box::new(Argon2Impl {
        memory: config.memory,
        iterations: config.iterations,
        parallelism: config.parallelism,
        key_len: config.key_len,
    })
}

impl OneWay for Argon2Impl {
    fn hash(&self, text: &[u8]) -> Result<Vec<u8>, SdkError> {
        use argon2_crate::{Algorithm, Argon2, Params, Version};

        let params = Params::new(
            self.memory,
            self.iterations,
            self.parallelism as u32,
            Some(self.key_len as usize),
        )
        .map_err(|e| new_with_code(Code::Argon2, e.to_string()))?;
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
        let mut salt = vec![0u8; 16];
        rand::thread_rng().fill_bytes(&mut salt);
        let mut out = vec![0u8; self.key_len as usize];
        argon2
            .hash_password_into(text, &salt, &mut out)
            .map_err(|e: argon2_crate::Error| new_with_code(Code::Argon2, e.to_string()))?;

        let b64 = base64::engine::general_purpose::STANDARD_NO_PAD;
        let encoded = format!(
            "$argon2id$v=19$m={},t={},p={}${}${}",
            self.memory,
            self.iterations,
            self.parallelism,
            b64.encode(&salt),
            b64.encode(&out)
        );
        Ok(encoded.into_bytes())
    }

    fn verify(&self, text: &[u8], hashed_text: &[u8]) -> Result<(), SdkError> {
        use argon2_crate::{Algorithm, Argon2, Params, Version};

        let hashed = std::str::from_utf8(hashed_text)
            .map_err(|e| new_with_code(Code::Argon2, e.to_string()))?;
        let parts: Vec<&str> = hashed.split('$').collect();
        if parts.len() != 6 {
            return Err(new_with_code(
                Code::Argon2,
                format!("invalid length of encoded hash, expected 6 but got {}", parts.len()),
            ));
        }
        if parts[1] != "argon2id" {
            return Err(new_with_code(Code::Argon2, "invalid argon2 variant"));
        }
        let version: i32 = parts[2]
            .trim_start_matches("v=")
            .parse::<i32>()
            .map_err(|e| new_with_code(Code::Argon2, e.to_string()))?;
        if version != 19 {
            return Err(new_with_code(
                Code::Argon2IncompatibleVersion,
                format!("current argon2 version is 19 but encoded hash using version {}", version),
            ));
        }

        let mut memory = 0u32;
        let mut iterations = 0u32;
        let mut parallelism = 0u8;
        for item in parts[3].split(',') {
            if let Some(v) = item.strip_prefix("m=") {
                memory = v.parse::<u32>().map_err(|e| new_with_code(Code::Argon2, e.to_string()))?;
            } else if let Some(v) = item.strip_prefix("t=") {
                iterations = v.parse::<u32>().map_err(|e| new_with_code(Code::Argon2, e.to_string()))?;
            } else if let Some(v) = item.strip_prefix("p=") {
                parallelism = v.parse::<u8>().map_err(|e| new_with_code(Code::Argon2, e.to_string()))?;
            }
        }

        let b64 = base64::engine::general_purpose::STANDARD_NO_PAD;
        let salt = b64
            .decode(parts[4])
            .map_err(|e| new_with_code(Code::Argon2, e.to_string()))?;
        let key = b64
            .decode(parts[5])
            .map_err(|e| new_with_code(Code::Argon2, e.to_string()))?;

        let params = Params::new(memory, iterations, parallelism as u32, Some(key.len()))
            .map_err(|e| new_with_code(Code::Argon2, e.to_string()))?;
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
        let mut out = vec![0u8; key.len()];
        argon2
            .hash_password_into(text, &salt, &mut out)
            .map_err(|e: argon2_crate::Error| new_with_code(Code::Argon2, e.to_string()))?;

        if out == key {
            Ok(())
        } else {
            Err(new_with_code(
                Code::Argon2NotMatch,
                "argon2 for plain text with hashed text is not match",
            ))
        }
    }
}