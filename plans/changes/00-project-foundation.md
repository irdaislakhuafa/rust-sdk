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

The SDK uses a two-layer model: **`codes`** define stable, semantic status/error categories; **`errors`** attach those codes to concrete errors, capture caller location, and compile errors into localized HTTP/app responses.

### `codes` module (`src/codes/`)

- **`Code` enum**: `#[repr(u64)]`, `Copy + Eq + Hash`. Stable numeric values via `code.value()` / `*self as u64`. Includes `NoCode = 0`, `Success = 1`, common HTTP/app errors, and module-specific ranges (`Sql*`, `Client*`, `Auth*`, `Json*`, `Storage*`, `Jwt*`, `Gql*`, `Argon2*`, `Bcrypt*`, `Aes256Gcm*`, `Smtp*`, `Identiface*`, `StrTemplate*`, `Queue*`, `Cache*`, `RateLimiter*`, `End`).
- **`CodeExt`**: helper methods on `Code`:
  - `is_between(start_code, end_code) -> bool` - numeric range check (inclusive).
  - `is_one_of(&[Code]) -> bool`
  - `is_not_one_of(&[Code]) -> bool`
- **`Message`**: localized response payload with `status_code: u16`, `title: String`, `body: String`.
- **`get_code_messages(code) -> HashMap<Language, Message>`**: maps a `Code` to localized messages (via `codes::messages` / `MsgCode`). Unmapped codes return an empty map.
- **`codes::compile(code, lang) -> Message`**: resolves a localized message for `code/lang`; falls back to English `Success` message, then an empty `200` message as last resort. Use this when you have a code and want a message without an error object.

### `errors` module (`src/errors/`)

- **`SdkError` alias**: `pub type SdkError = Box<dyn std::error::Error + Send + Sync>;` - standard return type for fallible SDK operations.
- **`Stacktrace`**: SDK error carrying code + caller location.
  - Fields: `message`, `cause: Option<SdkError>`, `code: Code`, `file`, `function`, `line`.
  - `Display` prints `message`; `Error::source()` returns `cause`.
  - `exit_code() -> i64`: returns `1` for `Code::NoCode`, otherwise `code.value() as i64`.
  - `PartialEq/Eq` compare simple fields; boxed causes are compared by pointer identity.
- **`AppError`**: structured application-facing error.
  - Fields: `code: Code`, `title`, `body`, `sys: Option<SdkError>`.
  - `AppError::new(code, title, body, sys)` constructs it.
  - `safe_convert(err) -> Result<&AppError, String>` downcasts a dyn error ref.
  - `Display` prefers `sys` message when present, else `Error [<code>]: <title>`; `Error::source()` returns `sys`.
  - `PartialEq/Eq` compare simple fields; `sys` compared by pointer identity.
- **Creating errors**:
  - `new_with_code(code, message) -> SdkError` is the preferred constructor. It is `#[track_caller]`, captures `file/line` from `std::panic::Location::caller()`, and returns a boxed `Stacktrace`.
  - Internal `create(...)` resolves `Code::NoCode` from an existing cause's stacktrace code when possible.
- **Inspecting errors**:
  - `get_code(err: &(dyn Error + Send + Sync + 'static)) -> Code` downcasts to `Stacktrace`; returns `Code::NoCode` if not a stacktrace.
  - `get_caller(cause: SdkError) -> Result<(String, usize, String), SdkError>` extracts `(file, line, message)` from a `Stacktrace`; otherwise returns an error wrapping the cause.
  - `get_caller_opt(cause: Option<SdkError>) -> Result<(String, usize, String), SdkError>` same as `get_caller`, but `None` becomes a `NoCode` stacktrace error.
- **Compiling to HTTP/app response**:
  - `errors::compile(err: SdkError, lang: Language) -> (u16, AppError)` gets code via `get_code`, looks up localized messages via `codes::get_code_messages`, and returns `(status_code, AppError{..., sys: Some(err)})`.
  - Fallback when no message exists for `lang/code`: `(500, AppError { code, title: "Service Error Not Defined", body: "Unknown error. Please contact admin", sys: Some(err) })`.

### Usage guidelines

- **Return type:** use `Result<T, SdkError>` for fallible SDK APIs.
- **Create semantic errors:** prefer `new_with_code(Code::..., "context")` over plain string errors.
- **Pick specific codes:** choose the closest domain code (`StorageNoFile`, `StorageDelFailure`, `SqlRecordDoesNotExist`, etc.) instead of generic codes when available.
- **Preserve context:** include operation/object context in the message (path, key, provider action) while keeping the code semantic.
- **Map at boundary:** use `errors::compile(err, lang)` at HTTP/API boundaries; use `get_code` for internal branching/retries/metrics.
- **Avoid ad-hoc errors:** do not return `Err("...".into())`/string-only errors in SDK APIs when a `Code` exists.

### Example (storage-style)

```rust
use crate::codes::Code;
use crate::errors::{new_with_code, SdkError};

fn put_object(path: &str) -> Result<(), SdkError> {
    // ... backend call ...
    Err(new_with_code(
        Code::StorageNoFile,
        format!("failed to read object: {}", path),
    ))
}

fn handle(err: SdkError) {
    let code = crate::errors::get_code(err.as_ref());
    if code == Code::StorageNoFile { /* handle */ }

    let (status, app_err) = crate::errors::compile(err, crate::language::ENGLISH);
    // status: u16, app_err: AppError { code, title, body, sys }
}
```

## Testing conventions

- **Unit tests:** co-locate tests per module as `*_test.rs` files (or inline `#[cfg(test)]` modules) and register them from the module's `mod.rs` when using separate files.
- **Verification:** run `cargo test` before considering work complete; use targeted runs during development (`cargo test --lib <module>::<test_name>`).

### Table-driven tests (go-sdk style, adapted to Rust)

The `go-sdk` uses table-driven tests heavily: define cases as a slice of structs, iterate, and run each case as a subtest with `t.Run`. In Rust we keep the same intent - one test body, many cases - with idiomatic Rust assertions.

#### Go pattern (reference)

```go
func TestPut(t *testing.T) {
    tests := []struct {
        name     string
        dirName  string
        fileName string
        wantDir  string
    }{
        {name: "basic dir", dirName: "dir", fileName: "file.txt", wantDir: "dir/"},
        {name: "already normalized", dirName: "dir/", fileName: "file.txt", wantDir: "dir/"},
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            got := NormalizeDir(tt.dirName)
            if got != tt.wantDir {
                t.Fatalf("NormalizeDir() = %v, want %v", got, tt.wantDir)
            }
        })
    }
}
```

#### Rust pattern (this SDK)

```rust
#[test]
fn test_dummy_put() {
    struct Case {
        name: &'static str,
        dir_name: &'static str,
        file_name: &'static str,
        file_size: i64,
        expected_dir: &'static str,
    }

    let cases = [
        Case { name: "basic dir", dir_name: "dir", file_name: "file.txt", file_size: 5, expected_dir: "dir/" },
        Case { name: "already normalized", dir_name: "dir/", file_name: "file.txt", file_size: 5, expected_dir: "dir/" },
    ];

    let storage = init(dummy_config()).unwrap();

    for case in cases {
        let put = storage
            .put((), PutParams { dir_name: case.dir_name.to_string(), file_name: case.file_name.to_string(), file_size: case.file_size, ..Default::default() })
            .unwrap_or_else(|e| panic!("put failed for {}: {}", case.name, e));

        assert_eq!(put.dir_name, case.expected_dir, "case: {}", case.name);
        assert_eq!(put.full_path, format!("{}{}", case.expected_dir, case.file_name), "case: {}", case.name);
    }
}
```

#### Rules

- **Case struct/tuple:** define a small `Case` struct (preferred when >2 fields) or tuple. Include `name: &'static str` for diagnostics. Use clear field names (`input`, `expected_*`, `want_err`, etc.).
- **Case collection:** use an array/`Vec` of cases and `for case in cases { ... }`. Keep case data close to the test.
- **Case naming in failures:** always include `case.name` in assertions/panics: `assert_eq!(got, want, "case: {}", case.name)`, `unwrap_or_else(|e| panic!("... {}: {}", case.name, e))`. This compensates for Rust not having Go's built-in `t.Run` subtest naming.
- **Coverage per table:** include happy path, edge cases, and error cases where applicable. Separate tables/tests by behavior when mixing success and failure makes the loop unclear.
- **Deterministic by default:** unit tests should not depend on network, external services, wall-clock timing, or global mutable state. Use fixed inputs and local setup.
- **Integration/env-gated cases:** when a test needs an external service (e.g. MinIO), gate it with env vars. If required vars are missing, `eprintln!("skipping ...: set ...")` and `return` so default `cargo test` stays green. Use unique keys/resources per case/run (e.g. UUID) to avoid collisions, and clean up created resources.
- **Failure behavior:** a panic in a loop stops the test at the first failing case. This is acceptable for the SDK's default style. Use separate `#[test]` functions or per-case test crates/macros (`rstest`, `test-case`) only when isolated per-case reporting is worth the extra complexity/dependency.
- **Setup/teardown:** share cheap immutable setup outside the loop; create per-case resources inside the loop when cases mutate state. For integration resources, delete/verify cleanup in the same case.
- **Assertions:** assert the full relevant result, not just "it didn't error". For errors, assert the semantic `Code` via `errors::get_code` when possible, not just `is_err()`.
