# 03 - SDK Errors Design & Porting Notes

## Overview

This document outlines the architectural translation and porting of the `errors` module from the Go SDK (`go-sdk/errors/`) into the Rust SDK (`rust-sdk/src/errors/`).

The `errors` module provides a custom application error type (`App` / `AppError`), a stack-trace-carrying error (`Stacktrace`), and helpers to extract caller information, compile localized error responses, and generate exit codes. It mirrors the behavior of `go-sdk/errors` while adapting to Rust's ownership, trait, and error-handling model.

## Architectural Comparison & Translation Strategy

### Go (`go-sdk/errors`)

- Defines two core types:
  - `App`: `{ Code codes.Code, Title string, Body string, sys error }` — a user-facing error payload.
  - `stacktrace`: `{ message, cause error, code codes.Code, file, function, line }` — an internal error carrying caller location.
- Captures caller location via `runtime.Caller(2)` and derives a short function name via `runtime.FuncForPC`.
- Provides:
  - `GetCode(err) codes.Code` — returns the code if `err` is a `*stacktrace`, else `codes.NoCode`.
  - `Compile(err, lang) (int, App)` — maps an error to an HTTP status code and localized `App`.
  - `GetCaller(cause) (file, line, message, err)` — extracts location from a `stacktrace`, or returns a cast failure error.
  - `NewWithCode(code, msg, val...)` — creates a `stacktrace` with caller location.
  - `ExitCode()` method on `*stacktrace` — `1` if `NoCode`, else `int(code)`.
- `App.Error()` returns `sys.Error()`; `stacktrace.Error()` returns `message`.

### Rust (`rust-sdk/src/errors`)

- Implements `App` (aliased as `AppError`) and `Stacktrace` structs mirroring Go.
- Uses `code: codes::Code`, `title/body: String`, and `sys/cause: Option<Box<dyn Error + Send + Sync>>`.
- Captures caller location via `#[track_caller]` + `std::panic::Location::caller()` (Rust equivalent of `runtime.Caller`). Function name is left empty because `Location` does not expose it.
- Provides:
  - `get_code(err)` — extracts `Code` from a `Stacktrace`, else `codes::NoCode`.
  - `compile(err, lang) -> (u16, App)` — maps an error to an HTTP status code and localized `App`.
  - `get_caller(err)` — returns `Result<(String, usize, String), Box<dyn Error + Send + Sync>>`.
  - `get_caller_opt(Option<...>)` — handles the nil case.
  - `new_with_code(code, message)` — `#[track_caller]`, so file/line point at the caller.
  - `exit_code()` — method on `Stacktrace`.
  - `safe_convert` — downcasts `&dyn Error` back to `&App`.
- Errors and tests are split into separate files (`errors_test.rs`, `errors_stacktrace_test.rs`).

## Data Structures

### `Stacktrace`

```rust
pub struct Stacktrace {
    pub message: String,
    pub cause: Option<Box<dyn Error + Send + Sync>>,
    pub code: Code,
    pub file: String,
    pub function: String,
    pub line: usize,
}
```

- `error()` returns `&self.message`.
- `Display` prints `message`.
- `source()` returns the chained `cause`.
- `exit_code()` returns `1` for `NoCode`, otherwise `code.value()`.
- `PartialEq`/`Eq` are implemented manually (see Notes).

### `App` (alias `AppError`)

```rust
pub struct App {
    pub code: Code,
    pub title: String,
    pub body: String,
    pub sys: Option<Box<dyn Error + Send + Sync>>,
}
```

- `Display` prints the wrapped error (`sys`) if present, otherwise `"Error [code]: title"`.
- `source()` returns the wrapped `sys`.
- `safe_convert` downcasts `&dyn Error` to `&App`.
- `PartialEq`/`Eq` are implemented manually (see Notes).

## Core Functions

### `get_code`

```rust
pub fn get_code(err: &(dyn Error + Send + Sync + 'static)) -> Code
```

Extracts `Code` from a `Stacktrace`. Returns `codes::NoCode` for non-stacktrace errors.

### `new_with_code`

```rust
#[track_caller]
pub fn new_with_code(code: Code, message: impl Into<String>) -> Box<dyn Error + Send + Sync>
```

Creates a `Stacktrace` with the caller's file and line captured via `#[track_caller]`.

### `get_caller`

```rust
#[track_caller]
pub fn get_caller(
    cause: Box<dyn Error + Send + Sync>,
) -> Result<(String, usize, String), Box<dyn Error + Send + Sync>>
```

- Returns `Ok((file, line, message))` if `cause` is a `Stacktrace`.
- Otherwise wraps the original error into a new `Stacktrace` and returns it as `Err`.

### `get_caller_opt`

```rust
#[track_caller]
pub fn get_caller_opt(
    cause: Option<Box<dyn Error + Send + Sync>>,
) -> Result<(String, usize, String), Box<dyn Error + Send + Sync>>
```

- `None` → `Err` with message `"failed to cast error to stacktrace"`.
- `Some(c)` → delegates to the same downcast-or-wrap logic.

### `compile`

```rust
pub fn compile(err: Box<dyn Error + Send + Sync>, lang: Language) -> (u16, App)
```

- Extracts the error code via `get_code`.
- Looks up localized messages via `codes::get_code_messages(code)`.
- On a match, returns `(msg.status_code, App { code, title: msg.title, body: msg.body, sys: Some(err) })`.
- Otherwise returns `(500, App { code, title: "Service Error Not Defined", body: "Unknown error. Please contact admin", sys: Some(err) })`.

### `create` (internal)

```rust
fn create(
    cause: Option<Box<dyn Error + Send + Sync>>,
    code: Code,
    message: String,
    loc: &'static std::panic::Location<'static>,
) -> Stacktrace
```

Internal constructor that resolves `NoCode` to the cause's code and captures `file`/`line` from `loc`.

## Rust Idioms & Translation Notes

- **`#[track_caller]`**: Required so that `std::panic::Location::caller()` inside `new_with_code` / `get_caller` points to the application call site rather than the helper itself (Rust equivalent of Go's `runtime.Caller(2)`).
- **`Box<dyn Error + Send + Sync>`**: Used instead of Go's `error` interface to allow storing arbitrary errors in `App.sys` / `Stacktrace.cause` while remaining `Send + Sync`.
- **Manual `PartialEq`/`Eq`**: `Box<dyn Error + Send + Sync>` does not implement `PartialEq`/`Eq`, so `#[derive(PartialEq, Eq)]` cannot be used. The manual impl compares simple fields directly and the boxed error by pointer identity (`std::ptr::eq`).
- **`Result` instead of multiple returns**: Go's `GetCaller` returns `(file, line, message, err)`; Rust returns `Result<(file, line, message), err>`.
- **Variadic formatting**: Go's `NewWithCode(code, msg, val...)` becomes `new_with_code(code, format!(...))`.

## Module Files

- `src/errors/mod.rs`: Defines `Stacktrace`, `App`/`AppError`, and core functions (`get_code`, `new_with_code`, `get_caller`, `get_caller_opt`, `compile`, `exit_code`).
- `src/errors/errors_test.rs`: Unit tests for `App` creation/conversion, `get_caller`, `compile`, and `get_code`.
- `src/errors/errors_stacktrace_test.rs`: Unit tests for `Stacktrace` message and `exit_code`.

## Notes

- No new crate dependencies were required.
- `App.sys` is exposed as `Option<Box<dyn Error + Send + Sync>>`.
- Function name is left empty because Rust's `Location` does not expose it (Go derives it via `runtime.FuncForPC`).

## API Differences (intentional, idiomatic Rust)

- `NewWithCode(code, msg, val...)` → `new_with_code(code, format!(...))`.
- `GetCaller(err) (file, line, message, err)` → `get_caller(err) -> Result<(file, line, message), err>`.
- `Compile(err, lang) (int, App)` → `compile(err, lang) -> (u16, App)`.
- `ExitCode()` method on `*stacktrace` → `exit_code()` method on `Stacktrace`.
