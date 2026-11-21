#[cfg(test)]
mod tests {
    use crate::codes::Code;
    use crate::cryptography::{OneWay, bcrypt_cost_from_hash, new_bcrypt, new_bcrypt_with_config};
    use crate::cryptography::BcryptConfig;
    use crate::errors;

    #[test]
    fn test_bcrypt_hash_and_verify() {
        struct Case {
            name: &'static str,
            plain: &'static [u8],
            cost: u32,
        }
        let cases = [
            Case {
                name: "default cost",
                plain: b"password",
                cost: crate::cryptography::BCRYPT_DEFAULT_COST,
            },
            Case {
                name: "min cost",
                plain: b"password",
                cost: crate::cryptography::BCRYPT_MIN_COST,
            },
        ];
        for case in cases {
            let h = new_bcrypt_with_config(BcryptConfig { cost: case.cost });
            let hashed = h
                .hash(case.plain)
                .unwrap_or_else(|e| panic!("hash failed for {}: {}", case.name, e));
            h.verify(case.plain, &hashed)
                .unwrap_or_else(|e| panic!("verify failed for {}: {}", case.name, e));
            // mismatch
            let err = h.verify(b"wrong-password", &hashed).unwrap_err();
            assert_eq!(
                errors::get_code(err.as_ref()),
                Code::Bcrypt,
                "case: {}",
                case.name
            );
            // cost round-trips
            let cost = bcrypt_cost_from_hash(&hashed)
                .unwrap_or_else(|e| panic!("cost_from_hash failed for {}: {}", case.name, e));
            assert_eq!(cost, case.cost, "case: {}", case.name);
        }
    }

    #[test]
    fn test_bcrypt_default() {
        let h = new_bcrypt();
        let hashed = h.hash(b"password").unwrap();
        h.verify(b"password", &hashed).unwrap();
    }

    #[test]
    fn test_bcrypt_invalid_hash() {
        let err = bcrypt_cost_from_hash(b"not-a-bcrypt-hash").unwrap_err();
        assert_eq!(errors::get_code(err.as_ref()), Code::Bcrypt);
    }

    #[test]
    fn test_bcrypt_trait_object() {
        fn run(h: &dyn OneWay) {
            let hashed = h.hash(b"abc").unwrap();
            h.verify(b"abc", &hashed).unwrap();
            assert!(h.verify(b"xyz", &hashed).is_err());
        }
        run(new_bcrypt().as_ref());
    }
}