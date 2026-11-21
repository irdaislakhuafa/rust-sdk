# 06 - SDK Storage Design & Porting Notes

## Overview

This document outlines the architectural translation and porting of the `storage` module from the Go SDK (`go-sdk/storage/`) into the Rust SDK (`rust-sdk/src/storage/`).

The Go module provides object-storage backends (Minio, dummy) behind a common interface. The Rust port adapts that design to idiomatic Rust naming, a synchronous `Storage` trait, and the SDK's `codes`/`errors` error model. MinIO uses `rust-s3` (sync rustls) to avoid Tokio/OpenSSL bridging.

## Source (Go SDK)

- `go-sdk/storage/` — package `storage` with:
  - Shared types/params/results (`Config`, `PutParams`, `PutResult`, `DelParams`, `UrlParams`, `UrlResult`) and `Storage` interface (`Put`, `Del`, `Url`).
  - Backends: `dummy` (no-op/deterministic), `minio` (S3-compatible), plus disk/mock variants.
  - Helpers: `NormalizeDir`, `FormatBytes`, `PercentBytes`.
- Observed Go usage: `InitDummy(cfg)`, `InitMinio(cfg)` factory functions; method calls `storage.Put(...)`, `storage.Del(...)`, `storage.Url(...)`.

## Rust Translation Strategy

### Naming

- All public functions/methods/variables use Rust conventions (snake_case methods/fields, CamelCase types). Go-style capitalized aliases (`InitDummy`, `Put`, `Del`, `Url`, `NormalizeDir`, etc.) were removed; only snake_case APIs remain.
- Factory functions: `init_dummy(cfg)`, `init_minio(cfg)`, and dispatching `init(cfg)`.
- Trait methods: `put`, `del`, `url`. Helpers: `normalize_dir`, `format_bytes`, `percent_bytes`.

### Files & Layout

- `src/storage/mod.rs` — module wiring, re-exports (`types::*`, `helper::*`, `DummyImpl`, `init_dummy`, `init_minio`), and `init(cfg)` factory.
- `src/storage/types.rs` — shared types and `Storage` trait.
- `src/storage/dummy.rs` — in-memory/no-op dummy backend.
- `src/storage/minio.rs` — MinIO/S3-compatible backend using `rust-s3`.
- `src/storage/helper.rs` — path/size helpers.
- `src/storage/dummy_test.rs` — table-driven deterministic unit tests for Dummy.
- `src/storage/minio_test.rs` — deterministic init test + env-gated table-driven CRUD integration test for MinIO.

### Abstraction

- `Storage` trait (`Send + Sync`) with methods `put`, `del`, `url` (first `_ctx: ()` param retained for parity with Go context usage; currently unused).
- `StorageType` enum (`Minio`, `Disk`, `Dummy`) with `as_str()`.
- Config and param/result structs are `#[derive(...)]` with sensible defaults (`PutParams`, `DelParams`, `UrlParams`, `UrlResult` derive `Default`).

### Backend Details

#### Dummy (`dummy.rs`)

- `DummyImpl { cfg }`, `init_dummy(cfg)` returns `Box<dyn Storage>`.
- `put`: normalizes `dir_name` via `normalize_dir`, returns deterministic result with `region="dummy"`, `tag="dummy"`, `size=params.file_size`, `full_path = dir_name + file_name`.
- `del`: no-op, returns `Ok(())`.
- `url`: returns deterministic URL parts: `scheme="https"`, `host="dummy.com"`, `full_path` preserved, `full_url=format!("dummy.com/{}", params.full_path)`, `raw_query=""`.

#### MinIO (`minio.rs`)

- Uses `rust-s3` with `default-features = false, features = ["sync-rustls-tls", "fail-on-err"]` to match the synchronous `Storage` trait (no Tokio runtime bridging).
- `init_minio(cfg)`:
  - normalizes config via `parse_default()`.
  - builds endpoint from `ssl/host/port`.
  - creates `s3::creds::Credentials` and `Region::Custom`.
  - checks bucket existence and attempts best-effort create if missing; returns `Code::StorageNoClient` on client/init failures.
  - uses path-style bucket access (`with_path_style()`).
- `put`: normalizes dir, reads `file_buffer` into bytes, defaults content type to `application/octet-stream`, uploads via `put_object_with_content_type`, extracts ETag header as `tag`, returns `PutResult`.
- `del`: deletes object by `full_path`; maps failures to `Code::StorageDelFailure`.
- `url`:
  - `parse_default()` for expiry.
  - empty `full_path` returns empty/default result.
  - `http(s)` path is parsed and returned as URL components.
  - otherwise generates presigned GET URL (`presign_get`) and parses it; maps failures to `Code::StorageGenerateUrlFailure` / parsing to appropriate code.

### Helpers (`helper.rs`)

- `normalize_dir(dir: &str) -> String`: ensures trailing `/`; empty dir becomes `"/"`.
- `format_bytes(bytes: u64) -> String`: human-readable size formatting.
- `percent_bytes(used: u64, total: u64) -> f64`: usage percent; guards zero total.

## Error Handling

- Uses `crate::errors::{new_with_code, SdkError}` and `crate::codes::Code`.
- Typical codes: `StorageNoClient`, `StorageNoFile`, `StorageDelFailure`, `StorageGenerateUrlFailure`, `BadRequest`, `NotImplemented` (factory/unsupported cases).

## Tests

### Dummy (`dummy_test.rs`)

Table-driven deterministic tests:
- `test_dummy_put`: cases for dir normalization and result fields.
- `test_dummy_del`: `DelParams` cases, all `Ok`.
- `test_dummy_url`: `full_path` cases, including empty path; validates fixed dummy URL shape.

### MinIO (`minio_test.rs`)

Table-driven + env-gated integration:
- `test_minio_init_invalid_endpoints`: deterministic invalid endpoint/init cases.
- `test_minio_crud_table_driven`: gated by `MINIO_TEST_ACCESS_KEY` and `MINIO_TEST_SECRET_KEY`; optional `MINIO_TEST_HOST`, `MINIO_TEST_PORT`, `MINIO_TEST_BUCKET`, `MINIO_TEST_REGION`, `MINIO_TEST_SSL`, `MINIO_TEST_TOKEN`. For each case: `put` -> verify exists via `head_object` -> `url` presign parseable -> `del` -> verify removed. Uses unique UUID keys per case. Skips with `eprintln!` when env vars absent.

## Verification

- `cargo test` passes by default (MinIO CRUD integration skipped unless env vars are set).
- Targeted: `cargo test --lib storage::dummy_test` / `cargo test --lib storage::minio_test`.
- `todo.md` storage items marked complete.
EOF
echo done