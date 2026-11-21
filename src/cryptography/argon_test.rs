#[cfg(test)]
mod tests {
    use crate::codes::Code;
    use crate::cryptography::{Argon2Config, OneWay, new_argon2, new_argon2_with_config};
    use crate::errors;

    #[test]
    fn test_argon_hash_and_verify() {
        struct Case {
            name: &'static str,
            plain: &'static [u8],
            wrong: &'static [u8],
        }
        let cases = [
            Case {
                name: "basic",
                plain: b"password",
                wrong: b"wrong-password",
            },
            Case {
                name: "empty",
                plain: b"",
                wrong: b"x",
            },
        ];
        for case in cases {
            let h = new_argon2_with_config(Argon2Config {
                memory: 4 * 1024,
                iterations: 3,
                parallelism: 1,
                key_len: 32,
            });
            let hashed = h
                .hash(case.plain)
                .unwrap_or_else(|e| panic!("hash failed for {}: {}", case.name, e));
            h.verify(case.plain, &hashed)
                .unwrap_or_else(|e| panic!("verify failed for {}: {}", case.name, e));
            let err = h.verify(case.wrong, &hashed).unwrap_err();
            assert_eq!(
                errors::get_code(err.as_ref()),
                Code::Argon2NotMatch,
                "case: {}",
                case.name
            );
        }
    }

    #[test]
    fn test_argon_default() {
        let h = new_argon2();
        let hashed = h.hash(b"password").unwrap();
        h.verify(b"password", &hashed).unwrap();
    }

    #[test]
    fn test_argon_invalid_hash() {
        struct Case {
            name: &'static str,
            hashed: &'static [u8],
        }
        let cases = [
            Case {
                name: "wrong length",
                hashed: b"$argon2id$v=19$m=4096,t=3,p=1$abc",
            },
            Case {
                name: "wrong variant",
                hashed: b"$argon2i$v=19$m=4096,t=3,p=1$AAAAAAAAAAAAAAAAAAAAAA$AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
            },
        ];
        let h = new_argon2();
        for case in cases {
            let err = h.verify(b"password", case.hashed).unwrap_err();
            // any argon error code acceptable
            let code = errors::get_code(err.as_ref());
            assert!(
                code == Code::Argon2 || code == Code::Argon2IncompatibleVersion,
                "case: {} got {:?}",
                case.name,
                code
            );
        }
    }

    #[test]
    fn test_argon_trait_object() {
        fn run(h: &dyn OneWay) {
            let hashed = h.hash(b"abc").unwrap();
            h.verify(b"abc", &hashed).unwrap();
            assert!(h.verify(b"xyz", &hashed).is_err());
        }
        run(new_argon2().as_ref());
    }
}