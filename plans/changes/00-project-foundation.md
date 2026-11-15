# Overview Rust SDK (`rust-sdk`)

This is a modular Rust SDK project inspired by the architectural design and module structure of [go-sdk](https://github.com/irdaislakhuafa/go-sdk.git). It provides a comprehensive, reusable suite of core modules and utilities for Rust applications.

## Tech Stacks & Development

- **Language**: Rust (Edition 2021+)
- **Build & Test Tool**: Cargo (`cargo test`, `cargo build`)
- **Architecture Style**: Modular SDK / Clean Package Structure (matching `go-sdk`)

## Project Directory Structure

```text
rust-sdk/
├── Cargo.toml / Cargo.lock            # Rust workspace & dependencies
├── plans/
│   └── changes/
│       └── 00-project-foundation.md    # Project foundation & migration notes
└── src/
    ├── lib.rs                         # Entry point & module declarations
    ├── appcontext/                    # Application context management
    ├── auth/                          # Authentication utilities (JWT, etc.)
    ├── caches/                        # Caching mechanisms (memory, sqlite, etc.)
    ├── codes/                         # Centralized error/status codes & messages
    ├── collections/                   # Collection helpers (map, reduce, slice)
    ├── concurrency/                   # Concurrency primitives (semaphore, etc.)
    ├── convert/                       # Type conversion utilities & pointers
    ├── cryptography/                  # Cryptographic tools (bcrypt, argon2, sha256, aes256gcm)
    ├── datastructure/                 # Data structures (queue, set)
    ├── db/                            # Database connections (postgresql, sqlite, mysql)
    ├── errors/                        # Error handling & stacktrace utilities
    ├── files/                         # File system helpers
    ├── header/                        # HTTP headers management
    ├── http/                          # HTTP client/server utilities & debugging
    ├── language/                      # Internationalization / localization (en, id)
    ├── log/                           # Logging subsystem & file/console storage
    ├── operator/                      # Utility operators (ternary, etc.)
    ├── querybuilder/                  # Query building & type-safe SQL helpers (sqlc-inspired)
    ├── random/                        # Random data generation
    ├── smtp/                          # Email sending utilities (SMTP / gomail equivalent)
    ├── sorter/                        # Sorting algorithms & helpers
    ├── storage/                       # Object storage integrations (Minio, dummy, mock)
    └── strformat/                     # String formatting utilities
```

## Coding Style & Architecture

Inspired by clean architecture principles and the modular design of [go-sdk](https://github.com/irdaislakhuafa/go-sdk.git):

- **Modular Package Structure**: Each subfolder in `src/` represents an independent, cohesive domain/utility module (mirroring `go-sdk` packages like `appcontext`, `auth`, `caches`, `cryptography`, `db`, `errors`, etc.).
- **Module Tree Management**: Explicit module declarations in `src/lib.rs` (`pub mod ...`), following standard idiomatic Rust module hierarchy.
- **Explicit Types & Context**: Strong typing, explicit error handling with stack traces, and safe memory management without hidden side-effects.
- **Testing**: Co-located unit tests (`*_test.rs` or inline tests) for every utility and module, mirroring the robust test coverage pattern in `go-sdk`.
- **Commit Style**: Conventional Commits format with concise subject lines (≤50 chars).

## Repository References & Templates

- **Rust SDK (`rust-sdk`)**: Current Repository
- **Go SDK Template (`go-sdk`)**: [https://github.com/irdaislakhuafa/go-sdk.git](https://github.com/irdaislakhuafa/go-sdk.git)
- **NusaTrade API Reference**: `/home/i/Projects/SelfInitiates/nusatrade/nusatrade-api/plans/changes/00-project-foundation.md`


## Error handling (`errors` + `codes`)

- **Alias:** `crate::errors::SdkError = Box<dyn std::error::Error + Send + Sync>` is the standard return type for fallible SDK operations.
- **Codes:** use `crate::codes::Code` (and `CodeExt`) to represent stable error/status categories (e.g. `Code::CacheKeyNotFound`, `Code::SqlInit`).
- **Creating errors:** prefer `crate::errors::new_with_code(Code::..., "message") -> SdkError` (captures caller file/line via `#[track_caller]`).
- **Inspecting codes:** use `crate::errors::get_code(&*err)`; it returns `Code::NoCode` if the error is not a stacktrace carrying a code.
- **Structured app errors:** `crate::errors::AppError` carries `code`, `title`, `body`, optional wrapped `sys: Option<SdkError>`. Build with `AppError::new(...)`.
- **Localization/HTTP mapping:** `crate::errors::compile(err, lang)` returns `(u16, AppError)` using `crate::codes::get_code_messages(...)` and language mappings.
- **Guideline:** construct errors with `new_with_code` and attach semantic `Code` values; avoid ad-hoc string-only errors in SDK APIs when a `Code` exists.
