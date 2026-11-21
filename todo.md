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

- [x] Implement functional helpers for slices/vectors: `map`, `reduce`, `filter`, `find`.
- [x] Add thorough unit tests for all collection operations.

## 6. Concurrency (`src/concurrency/`)

- [x] Implement concurrency utilities such as semaphores, rate limiters, or worker pools.
- [x] Add unit tests for concurrency primitives under load.

## 7. Type Conversion (`src/convert/`)

- [x] Implement pointer helper functions (`to_ptr`, `from_ptr`, etc.).
- [x] Add unit tests.

## 8. Cryptography (`src/cryptography/`)

- [x] Implement SHA-256 hashing.
- [x] Implement Bcrypt password hashing and verification (`bcrypt` crate).
- [x] Implement Argon2 hashing and verification (`argon2` crate).
- [x] Implement AES-256-GCM authenticated encryption/decryption (`aes-gcm` crate).
- [x] Add unit tests for all cryptographic algorithms.

## 9. Data Structures (`src/datastructure/`)

- [x] Implement Thread-safe Queue.
- [x] Implement Set (unique collection).
- [x] Add unit tests for queue and set operations.

## 10. Database Connections (`src/db/`)

- [x] Implement PostgreSQL connection factory/pool (`sqlx` or `tokio-postgres`).
- [x] Implement SQLite connection factory.
- [x] Implement MySQL connection factory.
- [x] Add unit tests / connection verification.

## 11. Errors & Stack Traces (`src/errors/`)

- [x] Implement custom application error type (`AppError`).
- [x] Implement stack trace capture and formatting.
- [x] Add unit tests for error wrapping and stack traces.

## 12. File System Helpers (`src/files/`)

- [x] Implement file I/O helper routines (read, write, exists, remove).
- [x] Add unit tests.

## 13. HTTP Headers (`src/header/`)

- [x] Define standard HTTP header constants and helper utilities.
- [x] Add unit tests.

## 14. HTTP Utilities & Debugging (`src/http/`)

- [x] Implement HTTP client/server request debugging helpers.
- [x] Implement HTTP status code constants and text mappings (`http/status/`).
- [x] Add unit tests.

## 15. Internationalization / Language (`src/language/`)

- [x] Define `Language` enum (`English`, `Indonesian`, etc.).
- [x] Implement localized text lookup (`text_en.rs`, `text_id.rs`).
- [x] Add unit tests.

## 16. Logging (`src/log/`)

- [x] Implement log levels (`DEBUG`, `INFO`, `WARN`, `ERROR`).
- [x] Implement Console logger.
- [x] Implement File logger with rotation/storage.
- [x] Add unit tests.

## 17. Operators (`src/operator/`)

- [x] Implement ternary operator helper function (`ternary`).
- [x] Add unit tests.

## 18. Query Builder & SQL Helpers (`src/querybuilder/`)

- [x] Implement SQLC-inspired typed query builders and wrappers.
- [x] Add example schemas and code generation documentation.

## 19. Random Utilities (`src/random/`)

- [x] Implement random integer/string generation functions.
- [x] Add unit tests.

## 20. SMTP / Email (`src/smtp/`)

- [x] Implement SMTP email sender (gomail equivalent using `lettre` crate).
- [x] Add unit tests (with mock SMTP / credentials).

## 21. Sorter (`src/sorter/`)

- [x] Implement generic sorting helpers and algorithms.
- [x] Add unit tests.

## 22. Object Storage (`src/storage/`)

- [x] Define storage interface/trait.
- [x] Implement MinIO / S3 storage client.
- [x] Implement Dummy / Mock storage for testing.
- [x] Add unit tests.

## 23. String Formatting (`src/strformat/`)

- [x] Implement string manipulation and formatting utilities.
- [x] Add unit tests.
