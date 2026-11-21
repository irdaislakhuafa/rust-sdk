# 07 - SDK Cryptography Design & Porting Notes

## Overview

This document outlines the architectural translation and porting of the `cryptography` module from the Go SDK (`go-sdk/cryptography/`) into the Rust SDK (`rust-sdk/src/cryptography/`).

The Rust port provides one-way (non-reversible) and two-way (reversible) cryptographic primitives behind trait-based abstractions, using idiomatic Rust naming, the SDK's `codes`/`errors` error model, and split files per algorithm with colocated table-driven tests.

## Source (Go SDK)

- `go-sdk/cryptography/` — package `cryptography` providing:
  - SHA-256 hashing (plain digest + HMAC via `WithKey`)
  - Bcrypt password hashing/verification with configurable cost
  - Argon2 (Argon2id) hashing/verification with configurable memory/iterations/parallelism
  - AES-256-GCM authenticated encryption/decryption
- Observed Go usage: builder-style constructors (`NewSHA256(text).WithKey(key).Build()`), cost helpers, encoded hash strings.

## Rust Translation Strategy

### Naming

- All public functions/methods/variables use Rust conventions (snake_case); types use CamelCase; constants use SCREAMING_SNAKE_CASE.
- Go-style CamelCase aliases (`NewSHA256`, `EncryptAES256GCM`, etc.) were removed; only snake_case APIs remain (consistent with the `storage` module).
- Empty marker traits (`SHA256<T>`, `BCRYPT`, `AES256GCM`) were removed as unused.

### Files & Layout

```text
src/cryptography/
├── mod.rs          # wiring, OneWay/TwoWay traits, shared hex helpers, re-exports, test wiring
├── sha256.rs       # Sha256Impl (OneWay) - plain digest / HMAC
├── bcrypt.rs       # BcryptImpl (OneWay) + BcryptConfig + bcrypt_cost_from_hash
├── argon.rs        # Argon2Impl (OneWay) + Argon2Config
├── aes256gcm.rs    # Aes256GcmImpl (TwoWay)
├── sha256_test.rs
├── bcrypt_test.rs
├── argon_test.rs
└── aes256gcm_test.rs
```

### Abstraction

- Two capability traits (instead of one unified interface, since hashing and encryption have different contracts):
  - `pub trait OneWay: Send + Sync + Debug` — `hash(&self, data) -> Result<Vec<u8>, SdkError>` and `verify(&self, data, hashed) -> Result<(), SdkError>`. Implementors: SHA-256, Bcrypt, Argon2.
  - `pub trait TwoWay: Send + Sync + Debug` — `encrypt`/`decrypt`. Implementor: AES-256-GCM.
- Factories return the trait (`Box<dyn OneWay>` / `Box<dyn TwoWay>`), not concrete types, so callers depend on the capability and algorithms are swappable. Concrete impls are `pub(crate)`.
- Configuration is supplied before construction (args or config struct), since config cannot be mutated on a `dyn` trait object:
  - `new_sha256() -> Box<dyn OneWay>` (plain digest)
  - `new_sha256_with_key(key: &[u8]) -> Box<dyn OneWay>` (HMAC)
  - `new_bcrypt() -> Box<dyn OneWay>` (default cost)
  - `new_bcrypt_with_config(BcryptConfig) -> Box<dyn OneWay>`
  - `new_argon2() -> Box<dyn OneWay>` (default params)
  - `new_argon2_with_config(Argon2Config) -> Box<dyn OneWay>`
  - `new_aes256gcm(key: &[u8]) -> Result<Box<dyn TwoWay>, SdkError>` (key validated at init; must be 32 bytes)

### Backend Details

#### SHA-256 (`sha256.rs`)
- `Sha256Impl { key: Option<Vec<u8>> }` implements `OneWay`.
- `hash`: plain SHA-256 digest, or HMAC-SHA256 when a key is configured; returns raw digest bytes (`Vec<u8>`).
- `verify`: recomputes digest and compares; returns `Ok(())` on match, `Err(Code::InvalidValue)` on mismatch.
- `Debug` derived.

#### Bcrypt (`bcrypt.rs`)
- `BcryptConfig { cost: u32 }` with `Default` (cost = `BCRYPT_DEFAULT_COST`).
- `BcryptImpl { cost }` implements `OneWay`.
- `hash`: returns encoded hash as bytes (`Vec<u8>`).
- `verify`: checks plain text against encoded hash; returns `Ok(())` on match, `Err(Code::Bcrypt)` on mismatch/parse failure.
- `bcrypt_cost_from_hash(hashed) -> Result<u32, SdkError>` free helper for extracting cost from an encoded hash.
- Constants: `BCRYPT_DEFAULT_COST`, `BCRYPT_MAX_COST` (31), `BCRYPT_MIN_COST` (4).

#### Argon2 (`argon.rs`)
- `Argon2Config { memory, iterations, parallelism, key_len }` with `Default` (4096 / 3 / 1 / 32).
- `Argon2Impl` implements `OneWay`.
- `hash`: Argon2id, returns the encoded PHC string as bytes.
- `verify`: parses the encoded hash, recomputes, compares; returns `Ok(())` on match, `Err(Code::Argon2NotMatch)` on mismatch, `Err(Code::Argon2)` / `Code::Argon2IncompatibleVersion` on parse/version errors.

#### AES-256-GCM (`aes256gcm.rs`)
- `Aes256GcmImpl { key }` implements `TwoWay`.
- Key must be 32 bytes; validated in `new_aes256gcm` (returns `Err(Code::InvalidValue)` otherwise).
- `encrypt`: generates a random 12-byte nonce, encrypts, encodes as hex then base64; returns ciphertext bytes.
- `decrypt`: reverses the encoding and decrypts; returns `Err(Code::Aes256GcmOpenError)` on authentication failure.

### Shared Helpers (`mod.rs`)
- `hex_encode(&[u8]) -> String` and `hex_decode(&str) -> Result<Vec<u8>, String>` (`pub(crate)`), used by SHA-256 and AES-256-GCM.

### Error Handling
- All operations return `Result<..., crate::errors::SdkError>`.
- Errors created with `crate::errors::new_with_code(crate::codes::Code, message)`.
- Semantic codes: `Code::InvalidValue` (SHA-256 digest mismatch / invalid key / bad ciphertext), `Code::Bcrypt` (bcrypt failures/mismatch), `Code::Argon2` / `Code::Argon2IncompatibleVersion` / `Code::Argon2NotMatch` (argon2), `Code::Aes256GcmOpenError` (AES decryption failure).

### Normalization Notes
- Bcrypt `verify` and Argon2 `verify` both return `Result<(), SdkError>` with mismatch as a semantic error (Argon2 previously returned `Result<bool, SdkError>`; changed for consistency).
- `hash` returns `Vec<u8>` uniformly (raw digest bytes for SHA-256; encoded hash bytes for bcrypt/argon2; ciphertext bytes for AES).
- SHA-256 moved from builder-style `NewSHA256(text).WithKey(key).Build()` to `new_sha256()` / `new_sha256_with_key(key)` + `hash(data)` / `verify(data, digest)`, enabling reuse and trait polymorphism.

### Tests
- Table-driven tests per backend in `*_test.rs`, wired via `#[cfg(test)] mod ...;` in `mod.rs`.
- Each test uses a `Case { name: &'static str, ... }` struct, includes `case.name` in assertions/panics, covers happy path, edge cases, and error cases, and asserts semantic `Code` via `errors::get_code`.
- Trait-object smoke tests (`Box<dyn OneWay>` / `Box<dyn TwoWay>`) verify polymorphic dispatch works.
- Targeted run: `cargo test --lib cryptography` (15 tests).

### Verification
- `cargo build` and `cargo test` pass (81 tests total, 0 failures) after the refactor.
- `todo.md` item 8 (Cryptography) can be marked complete (SHA-256, Bcrypt, Argon2, AES-256-GCM implemented with unit tests).

### API Differences (intentional, idiomatic Rust)
- Go-style CamelCase aliases removed; only snake_case APIs remain.
- Empty marker traits removed.
- Factories return `Box<dyn OneWay>` / `Box<dyn TwoWay>` instead of concrete types.
- `verify` returns `Result<(), SdkError>` instead of `Result<bool, SdkError>` (argon2).
- SHA-256 config (key) is supplied at construction; data is passed to `hash`/`verify` per call.