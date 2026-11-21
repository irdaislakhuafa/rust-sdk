#[cfg(test)]
mod tests {
    use crate::cryptography::{OneWay, hex_encode, hex_decode, new_sha256, new_sha256_with_key};

    #[test]
    fn test_sha256_digest() {
        struct Case {
            name: &'static str,
            data: &'static [u8],
            key: Option<&'static [u8]>,
            expected_hex: &'static str,
        }
        let cases = [
            Case {
                name: "plain sha256",
                data: b"password",
                key: None,
                expected_hex: "5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8",
            },
            Case {
                name: "hmac sha256",
                data: b"password",
                key: Some(b"key"),
                expected_hex: "4d42fb9ffc8d7d0a245429438b4bc73db1007a167026a0a0c6a74fa58e8e86ca",
            },
        ];
        for case in cases {
            let h = match case.key {
                Some(k) => new_sha256_with_key(k),
                None => new_sha256(),
            };
            let digest = h
                .hash(case.data)
                .unwrap_or_else(|e| panic!("hash failed for {}: {}", case.name, e));
            assert_eq!(hex_encode(&digest), case.expected_hex, "case: {}", case.name);
            h.verify(case.data, &digest)
                .unwrap_or_else(|e| panic!("verify failed for {}: {}", case.name, e));
        }
    }

    #[test]
    fn test_sha256_verify_mismatch() {
        let h = new_sha256();
        let digest = h.hash(b"password").unwrap();
        let err = h.verify(b"other", &digest).unwrap_err();
        assert_eq!(
            crate::errors::get_code(err.as_ref()),
            crate::codes::Code::InvalidValue,
        );
    }

    #[test]
    fn test_sha256_trait_object() {
        fn run(h: &dyn OneWay) {
            let d = h.hash(b"abc").unwrap();
            assert_eq!(d.len(), 32);
            h.verify(b"abc", &d).unwrap();
        }
        run(new_sha256().as_ref());
        run(new_sha256_with_key(b"k").as_ref());
    }
}