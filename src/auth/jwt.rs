// Default JWT implementation for the auth module.

use crate::auth::{Claims, JWTInterface};
use crate::codes::Code;
use crate::errors::{new_with_code, SdkError};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};

pub struct Jwt<C> {
    claims: C,
    signing_method: Algorithm,
    secret_key: Vec<u8>,
}

impl<C> Jwt<C>
where
    C: Claims,
{
    pub fn new(secret_key: &[u8], claims: C) -> Self {
        Self {
            claims,
            signing_method: Algorithm::HS256,
            secret_key: secret_key.to_vec(),
        }
    }

    pub fn init_jwt(secret_key: &[u8], claims: C) -> Self {
        Self::new(secret_key, claims)
    }

    pub fn generate(&self) -> Result<String, SdkError> {
        JWTInterface::generate(self)
    }

    pub fn validate(&self, token_string: &str) -> Result<jsonwebtoken::TokenData<C>, SdkError> {
        JWTInterface::validate(self, token_string)
    }

    pub fn extract_claims(&self, token: &jsonwebtoken::TokenData<C>) -> Result<C, SdkError> {
        JWTInterface::extract_claims(self, token)
    }

    pub fn with_signing_method(&mut self, signing_method: Algorithm) -> &mut Self {
        self.signing_method = signing_method;
        self
    }
}

impl<C> JWTInterface<C> for Jwt<C>
where
    C: Claims,
{
    fn generate(&self) -> Result<String, SdkError> {
        let token = jsonwebtoken::encode(
            &Header::new(self.signing_method),
            &self.claims,
            &EncodingKey::from_secret(&self.secret_key),
        )
        .map_err(|e| new_with_code(Code::JwtSignedStringError, e.to_string()))?;
        Ok(token)
    }

    fn validate(&self, token_string: &str) -> Result<jsonwebtoken::TokenData<C>, SdkError> {
        let token = jsonwebtoken::decode::<C>(
            token_string,
            &DecodingKey::from_secret(&self.secret_key),
            &Validation::new(self.signing_method),
        )
        .map_err(|e| new_with_code(Code::JwtParseWithClaimsError, e.to_string()))?;
        Ok(token)
    }

    fn extract_claims(&self, token: &jsonwebtoken::TokenData<C>) -> Result<C, SdkError> {
        Ok(token.claims.clone())
    }

    fn with_signing_method(&mut self, signing_method: Algorithm) -> &mut Self {
        self.signing_method = signing_method;
        self
    }
}

// Go-style function name
pub fn InitJWT<C>(secret_key: &[u8], claims: C) -> Jwt<C>
where
    C: Claims,
{
    Jwt::new(secret_key, claims)
}
