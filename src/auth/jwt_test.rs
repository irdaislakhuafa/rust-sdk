#[cfg(test)]
mod tests {
    use crate::auth::InitJWT;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct MyClaims {
        sub: String,
        exp: usize,
    }

    #[test]
    fn test_jwt_roundtrip() {
        let claims = MyClaims {
            sub: "user".into(),
            exp: 9999999999,
        };
        let jwt = InitJWT(b"secret", claims);
        let token = jwt.generate().unwrap();
        let decoded = jwt.validate(&token).unwrap();
        let c = jwt.extract_claims(&decoded).unwrap();
        assert_eq!(c.sub, "user");
    }
}
