# 04 - SDK Caches Design & Porting Notes

## Overview

This document outlines the architectural translation and porting of the `caches` module from the Go SDK (`go-sdk/caches/`) into the Rust SDK (`rust-sdk/src/caches/`).

The goal is to mirror Go’s behavior and API surface while using idiomatic Rust: trait-based abstraction, explicit error handling via `crate::errors`/`crate::codes`, and split files per backend with colocated tests.

## Source (Go SDK)

- `go-sdk/caches/caches.go` — `Interface[T]`, `item[T]`, `StorageType`, `Config`, `Init[T]`
- `go-sdk/caches/memory.go` — in-memory implementation
- `go-sdk/caches/sqlite.go` — SQLite-backed implementation (stores JSON of `item[T]`)

Key behaviors (parity targets):

- `Remember(key, ttlS, callback)`: return cached value if present and not expired; on expiration delete and re-evaluate like Go; on miss call callback, store with `TTL = now_unix + ttlS`.
- `Forget`/`Get` return “key not found” style errors mapped to cache codes in Go.
- SQLite stores JSON text for the cached `item` payload (TTL + data).
- Factory `Init(cfg)` selects backend; unknown type returns a not-implemented style error.

## Rust Translation Strategy

### Abstraction

- Go `Interface[T]` → Rust trait `pub trait Cache<T>: Send + Sync + ...` in `src/caches/mod.rs` (object-safe methods using `&mut dyn FnMut` for closures).
- Backends:
  - `MemoryCache<T>` in `memory.rs`
  - `SqliteCache<T>` in `sqlite.rs`
- Unified construction via `pub fn init<T>(cfg: Config) -> Result<Box<dyn Cache<T>>, crate::errors::SdkError>`.

### Files & Layout (mirrors Go naming)

- `src/caches/mod.rs`
  - `StorageType`, `Config` (`memory()`, `sqlite(dir)`), `Item<T>` (internal), `now_secs()`
  - `pub trait Cache<T: Clone + Serialize + DeserializeOwned + Send + Sync + 'static>`
  - `pub fn init<T>(...) -> Result<Box<dyn Cache<T>>, SdkError>`
- `src/caches/memory.rs`
  - `MemoryCache<T>` with `RwLock<HashMap<String, Item<T>>>`
  - tests in `memory_test.rs` wired via `#[cfg(test)] mod memory_test;`
- `src/caches/sqlite.rs`
  - `SqliteCache<T>` with `Mutex<rusqlite::Connection>`, table `caches(id, key TEXT UNIQUE, data TEXT JSON)`
  - uses bundled SQLite feature in `Cargo.toml`
  - tests in `sqlite_test.rs` wired via `#[cfg(test)] mod sqlite_test;`
- Tests are split per backend (`*_test.rs`) rather than a single combined file, matching Go’s `memory_test.go` / `sqlite_test.go` style.

### Dependencies (added)

- `serde`, `serde_json` (serialize/deserialize cached `Item<T>` and values)
- `rusqlite` with `bundled` feature

### Error Handling

- All cache operations return `Result<..., crate::errors::SdkError>` (alias of `Box<dyn std::error::Error + Send + Sync>` defined in `crate::errors`).
- Errors are created with `crate::errors::new_with_code(crate::codes::Code, message)` (e.g. `CacheKeyNotFound`, `CacheAddErr`, `CacheReadErr`, `CacheDelErr`, `SqlInit`, `NotImplemented` context as appropriate).
- Callers can inspect codes via `crate::errors::get_code(&*err)` when needed (tests do this for `CacheKeyNotFound`).

### API Notes (Rust vs Go)

- `Remember` takes `&mut dyn FnMut() -> Result<T, SdkError>` (object-safe) instead of a generic `FnOnce` in the trait; call sites use `&mut || ...`.
- `ForgetFn` takes `&mut dyn FnMut(&str) -> Result<T, SdkError>`.
- `Config.dir` is `Option<String>`; SQLite backend uses it as the path/prefix consistent with Go’s `Config.Dir` usage in tests (`"/tmp/"`).

## Verification

- `cargo test` passes with unit tests for memory and SQLite caches (split test files).
- `cargo clippy` run with remaining warnings only where unrelated (e.g. pre-existing `AppContext` Default suggestion noted during development).
