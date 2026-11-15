# 02 - SDK Codes Design & Porting Notes

## Overview

This document outlines the architectural translation and porting of the `codes` module from the Go SDK (`go-sdk/codes/`) into the Rust SDK (`rust-sdk/src/codes/`).

## Architectural Comparison & Translation Strategy

- **Go (`go-sdk/codes`)**:
  - Leverages `type Code uint64` with `iota` for auto-incrementing constants (`CodeStart`, `CodeSuccess`, `CodeInvalidValue`, etc.).
  - Implements a `CodeInterface` trait and pointer methods (`IsBetween`, `IsOneOf`, `IsNotOneOf`).
  - Stores localized error and success messages in `codeMessages` mapped by `Code` and `language.Language`.

- **Rust (`rust-sdk/src/codes`)**:
  - Implements `Code` as an enum with `#[repr(u64)]` and `MsgCode` as an enum with `#[repr(i64)]` to provide automatic sequential values (mimicking Go's `iota`) while ensuring type safety and 64-bit precision.
  - Implements an extension trait `CodeExt` providing helper methods (`is_between`, `is_one_of`, `is_not_one_of`) for `Code`.
  - Organizes localized messages in `src/codes/messages.rs` supporting English (`language::ENGLISH`) and Indonesian (`language::INDONESIAN`) with dynamic HTTP status text mapping (`language::http_status_text`).
  - Includes dedicated unit test files (`codes_test.rs` and `code_func_test.rs`) covering message compilation, localization, fallbacks, and helper functions.

## Module Files

- `src/codes/mod.rs`: Defines the `Code` enum, `Message` struct, `CodeExt` trait, and core compilation functions (`get_code_messages`, `compile`).
- `src/codes/messages.rs`: Houses the `MsgCode` enum (`#[repr(i64)]`) and message dictionaries for 2xx, 4xx, and 5xx error/success codes in English and Indonesian.
- `src/codes/codes_test.rs`: Unit tests for compilation, language localization, and fallbacks.
- `src/codes/code_func_test.rs`: Unit tests for `is_between`, `is_one_of`, and `is_not_one_of`.
