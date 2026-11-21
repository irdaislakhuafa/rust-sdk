// auth module (port of go-sdk/auth)
//
// Contract (trait) lives here; concrete implementations live in separate files
// (e.g. `jwt.rs`) so additional backends can be added without touching the contract.

use jsonwebtoken::Algorithm;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub mod jwt;

#[cfg(test)]
mod jwt_test;

pub use jwt::Jwt;
pub use jwt::InitJWT;

/// Trait bound for types usable as JWT claims.
pub trait Claims: Clone + Serialize + DeserializeOwned + Send + Sync + 'static {}

impl<T> Claims for T
where
    T: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
{
}

/// Contract for JWT token generation/validation.
pub trait JWTInterface<C> {
    fn generate(&self) -> Result<String, crate::errors::SdkError>;
    fn validate(&self, token_string: &str) -> Result<jsonwebtoken::TokenData<C>, crate::errors::SdkError>;
    fn extract_claims(&self, token: &jsonwebtoken::TokenData<C>) -> Result<C, crate::errors::SdkError>;
    fn with_signing_method(&mut self, signing_method: Algorithm) -> &mut Self;
}
