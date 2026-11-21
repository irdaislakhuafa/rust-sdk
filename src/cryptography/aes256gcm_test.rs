#[cfg(test)]
mod tests {
    use crate::codes::Code;
    use crate::cryptography::{TwoWay, new_aes256gcm};
    use crate::errors;

    #[test]
    fn test_aes_roundtrip() {
        struct Case {
            name: &'static str,
            text: &'static [u8],
            key: &'static [u8],
        }
        let cases = [
            Case {
                name: "password",
                text: b"password",
                key: b"7d5b44298bf959af149a0086d79334e6",
            },
            Case {
                name: "empty plaintext",
                text: b"",
                key: b"7d5b44298bf959af149a0086d79334e6",
            },
            Case {
                name: "longer text",
                text: b"the quick brown fox jumps over the lazy dog",
                key: b"7d5b44298bf959af149a0086d79334e6",
            },
        ];
        for case in cases {
            let cipher = new_aes256gcm(case.key).unwrap_or_else(|e| panic!("init failed for {}: {}", case.name, e));
            let enc = cipher
                .encrypt(case.text)
                .unwrap_or_else(|e| panic!("encrypt failed for {}: {}", case.name, e));
            let dec = cipher
                .decrypt(&enc)
                .unwrap_or_else(|e| panic!("decrypt failed for {}: {}", case.name, e));
            assert_eq!(dec, case.text, "case: {}", case.name);
        }
    }

    #[test]
    fn test_aes_invalid_key_length() {
        struct Case {
            name: &'static str,
            key: &'static [u8],
        }
        let cases = [
            Case {
                name: "too short",
                key: b"short",
            },
            Case {
                name: "too long",
                key: b"7d5b44298bf959af149a0086d79334e6-extra",
            },
        ];
        for case in cases {
            let err = new_aes256gcm(case.key).unwrap_err();
            assert_eq!(
                errors::get_code(err.as_ref()),
                Code::InvalidValue,
                "case: {}",
                case.name
            );
        }
    }

    #[test]
    fn test_aes_invalid_ciphertext() {
        let key = b"7d5b44298bf959af149a0086d79334e6";
        let cipher = new_aes256gcm(key).unwrap();
        let err = cipher.decrypt(b"not-valid-ciphertext").unwrap_err();
        assert_eq!(errors::get_code(err.as_ref()), Code::InvalidValue);
    }

    #[test]
    fn test_aes_trait_object() {
        fn run(c: &dyn TwoWay) {
            let enc = c.encrypt(b"abc").unwrap();
            assert_eq!(c.decrypt(&enc).unwrap(), b"abc");
        }
        let c = new_aes256gcm(b"7d5b44298bf959af149a0086d79334e6").unwrap();
        run(c.as_ref());
    }
}