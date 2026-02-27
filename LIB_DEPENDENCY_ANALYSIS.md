# Library Dependency Analysis

## Current State

### Dependency Graph

```
error (base - no internal deps)
  ↑
  ├── common (depends on error for HTTP errors)
  │
  └── infrastructure (depends on error + config)
          ↑
          config (independent - has its own ConfigError using thiserror)
```

### Current Cargo Dependencies

| Library          | Depends On              |
| ---------------- | ----------------------- |
| `error`          | (none - base)           |
| `config`         | (none - uses thiserror) |
| `common`         | `error`                 |
| `infrastructure` | `error`, `config`       |
| `telemetry`      | (none)                  |

---

## Key Points

### 1. Config is Independent

The `config` crate does NOT depend on `error`. It has its own error handling:

```rust
// libs/config/src/core/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to load configuration: {0}")]
    Load(String),
    // ...
}
```

### 2. Infrastructure Depends on Both

```toml
# libs/infrastructure/Cargo.toml
[dependencies]
error = { path = "../error" }
config = { path = "../config" }
```

This allows infrastructure to:

- Use typed errors from the error crate
- Use configuration types from config crate

### 3. Common Depends on Error

```toml
# libs/common/Cargo.toml
[dependencies]
error = { path = "../error", features = ["http"] }
```

---

## Circular Dependency Check

```
error (base)
  ↑
common → error (OK)
  ↑
services

infrastructure → config + error (OK)
  ↑
services
```

No circular dependencies - all libraries form a DAG.

---

## Summary

| Library          | Depends On        | Notes                           |
| ---------------- | ----------------- | ------------------------------- |
| `error`          | (none - base)     | Core error types                |
| `config`         | (none)            | Uses thiserror for ConfigError  |
| `infrastructure` | `error`, `config` | For typed errors + config types |
| `common`         | `error`           | For HTTP error types            |
| `telemetry`      | (none)            | Tracing/logging                 |

All libraries are properly structured with no circular dependencies.
