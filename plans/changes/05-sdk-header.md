# 05 - SDK Header Design & Porting Notes

## Overview

This document outlines the architectural translation and porting of the `header` module from the Go SDK (`go-sdk/header/`) into the Rust SDK (`rust-sdk/src/header/`).

The Go module is a constants-only package. The Rust port preserves the exact string values and semantic grouping while using idiomatic Rust constant naming.

## Source (Go SDK)

- `go-sdk/header/header.go` — package `header` defining string constants grouped into:
  - Header keys (`KeyRequestID`, `KeyAuthorization`, `KeyUserAgent`, `KeyContentType`, `KeyContentAccept`, `KeyAcceptLanguage`, `KeyCacheControl`)
  - Content types for request payloads (`ContentTypeJSON`, `ContentTypeXML`, `ContentTypeForm`, `ContentTypeHTML`, `ContentTypePDF`)
  - Accept/media types for content negotiation (`MediaTextPlain`, `MediaTextHTML`, `MediaTextCSV`, `MediaTextXML`, `MediaImageGIF`, `MediaImageJPEG`, `MediaImagePNG`, `MediaImageWEBP`)
  - Cache control directives (`CacheControlNoCache`, `CacheControlNoStore`)
  - Content encoding (`ContentEncodingUTF8`)

Observed Go usage accesses constants directly (e.g. `header.KeyContentType`, `header.ContentTypeJSON`) in HTTP debug helpers and storage helpers.

## Rust Translation Strategy

### Abstraction

- No trait/struct needed: this is a pure constants module, mirroring Go's package-level `const`.
- Constants are declared as `pub const NAME: &str = "...";` in `src/header/mod.rs`.
- Naming uses idiomatic Rust `SCREAMING_SNAKE_CASE` (e.g. `KEY_CONTENT_TYPE`, `CONTENT_TYPE_JSON`) while keeping values byte-for-byte identical to Go.
- Grouping and comments mirror Go's section headers (header keys, content types, media/accept, cache control, encoding).

### Files & Layout

- `src/header/mod.rs` — all public constants plus module-level docs.
- `src/header/header_test.rs` — co-located unit tests wired via `#[cfg(test)] mod header_test;` in `mod.rs`, following the same split-file pattern used by `codes`/`caches`.

### Tests

- `header_test.rs` verifies each constant equals its Go-defined string value.
- Includes an invariant check ensuring all header keys remain lowercase (consistent with HTTP canonical form).
- Tests use `use super::super::*;` to reach constants from the nested `tests` module, matching the pattern in `codes_test.rs`.

## API Notes (Rust vs Go)

- Go identifiers are CamelCase (`KeyRequestID`); Rust identifiers are `SCREAMING_SNAKE_CASE` (`KEY_REQUEST_ID`). Values are unchanged, so behavior/interop is identical.
- No runtime logic, dependencies, or traits are required; the module is zero-cost and compile-time.

## Verification

- `cargo test --lib header` passes (6 tests).
- `cargo build` and `cargo test --lib` pass with no regressions.
- `todo.md` item 13 (HTTP Headers) marked complete.