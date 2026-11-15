# 01 - SDK AppContext Design & Translation

## Overview

This document outlines the architectural translation of `appcontext` from the Go SDK (`go-sdk/appcontext/`) into the Rust SDK (`rust-sdk/src/appcontext/`).

## Architectural Comparison & Translation Strategy

- **Go (`go-sdk/appcontext`)**:
  - Leverages Go's built-in `context.Context` interface (contract).
  - Uses strongly-typed package functions taking and returning `context.Context` (`SetAcceptLanguage`, `GetAcceptLanguage`, etc.) with `contextKey` types and default fallbacks via ternary operations.
- **Rust (`rust-sdk/src/appcontext`)**:
  - Implements the equivalent of Go's interface contract using a Rust trait (`pub trait Interface`).
  - Defines request-scoped getters and setters for:
    - Accept Language (`Language`)
    - Request ID (`String`)
    - Service Version (`String`)
    - User Agent (`String`)
    - Request Start Time (`SystemTime`)
  - Provides a concrete default implementation (`AppContext`) conforming to the trait.
  - Includes comprehensive co-located unit tests (`appcontext_test.rs`) verifying default fallbacks and setter/getter workflows.
