# TODO: Complete Porting of `go-sdk` to `rust-sdk`

This document outlines the step-by-step porting tasks for each module from `go-sdk` to `rust-sdk`.

## 1. AppContext (`src/appcontext/`)

- [x] Define abstract trait contract (`Interface`) for request-scoped metadata (Accept Language, Request ID, Service Version, User Agent, Request Start Time).
- [x] Implement concrete `AppContext` struct implementing `Interface`.
- [x] Add comprehensive unit tests in `appcontext_test.rs`.

## 2. Authentication (`src/auth/`)

- [x] Port JWT token generation and validation.
- [x] Implement claims struct and signing/verification methods (using `jsonwebtoken` crate).
- [x] Add unit tests for JWT success/failure cases.

## 3. Caching (`src/caches/`)

- [x] Implement abstract cache interface/trait (`Cache`).
- [x] Implement Memory Cache (`sync::RwLock<HashMap>`).
- [x] Implement SQLite Cache (using `rusqlite`).
- [x] Add unit tests for memory and SQLite caching backends.

## 4. Error Codes & Messages (`src/codes/`)

- [x] Define standard error/status codes and messages (mirroring Go SDK codes package).
- [x] Implement mapping between code numbers and localized messages.
- [x] Add unit tests.

## 5. Collections (`src/collections/`)

- [ ] Implement functional helpers for slices/vectors: `map`, `reduce`, `filter`, `find`.
- [ ] Add thorough unit tests for all collection operations.

## 6. Concurrency (`src/concurrency/`)

- [ ] Implement concurrency utilities such as semaphores, rate limiters, or worker pools.
- [ ] Add unit tests for concurrency primitives under load.

## 7. Type Conversion (`src/convert/`)

- [ ] Implement pointer helper functions (`to_ptr`, `from_ptr`, etc.).
- [ ] Add unit tests.

## 8. Cryptography (`src/cryptography/`)

- [x] Implement SHA-256 hashing.
- [x] Implement Bcrypt password hashing and verification (`bcrypt` crate).
- [x] Implement Argon2 hashing and verification (`argon2` crate).
- [x] Implement AES-256-GCM authenticated encryption/decryption (`aes-gcm` crate).
- [x] Add unit tests for all cryptographic algorithms.

## 9. Data Structures (`src/datastructure/`)

- [ ] Implement Thread-safe Queue.
- [ ] Implement Set (unique collection).
- [ ] Add unit tests for queue and set operations.

## 10. Database Connections (`src/db/`)

- [x] Implement PostgreSQL connection factory/pool (`sqlx`).
- [x] Implement SQLite connection factory.
- [x] Implement MySQL connection factory.
- [x] Add unit tests / connection verification.

## 11. Errors & Stack Traces (`src/errors/`)

- [x] Implement custom application error type (`AppError`).
- [x] Implement stack trace capture and formatting.
- [x] Add unit tests for error wrapping and stack traces.

## 12. File System Helpers (`src/files/`)

- [ ] Implement file I/O helper routines (read, write, exists, remove).
- [ ] Add unit tests.

## 13. HTTP Headers (`src/header/`)

- [x] Define standard HTTP header constants and helper utilities.
- [x] Add unit tests.

## 14. HTTP Utilities & Debugging (`src/http/`)

- [ ] Implement HTTP client/server request debugging helpers.
- [ ] Implement HTTP status code constants and text mappings (`http/status/`).
- [ ] Add unit tests.

## 15. Internationalization / Language (`src/language/`)

- [ ] Define `Language` enum (`English`, `Indonesian`, etc.).
- [ ] Implement localized text lookup (`text_en.rs`, `text_id.rs`).
- [ ] Add unit tests.

## 16. Logging (`src/log/`)

- [ ] Implement log levels (`DEBUG`, `INFO`, `WARN`, `ERROR`).
- [ ] Implement Console logger.
- [ ] Implement File logger with rotation/storage.
- [ ] Add unit tests.

## 17. Operators (`src/operator/`)

- [ ] Implement ternary operator helper function (`ternary`).
- [ ] Add unit tests.

## 18. Query Builder & SQL Helpers (`src/querybuilder/`)

- [ ] Implement SQLC-inspired typed query builders and wrappers.
- [ ] Add example schemas and code generation documentation.

## 19. Random Utilities (`src/random/`)

- [ ] Implement random integer/string generation functions.
- [ ] Add unit tests.

## 20. SMTP / Email (`src/smtp/`)

- [ ] Implement SMTP email sender (gomail equivalent using `lettre` crate).
- [ ] Add unit tests (with mock SMTP / credentials).

## 21. Sorter (`src/sorter/`)

- [ ] Implement generic sorting helpers and algorithms.
- [ ] Add unit tests.

## 22. Object Storage (`src/storage/`)

- [x] Define storage interface/trait.
- [x] Implement MinIO / S3 storage client.
- [x] Implement Dummy / Mock storage for testing.
- [x] Add unit tests.

## 23. String Formatting (`src/strformat/`)

- [ ] Implement string manipulation and formatting utilities.
- [ ] Add unit tests.
