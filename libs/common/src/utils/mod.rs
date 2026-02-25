//! Utilities module
//!
//! Common utility functions and helpers used across the application.
//!
//! ## Organization
//!
//! - `strings` - String manipulation and formatting utilities
//! - `encoding` - JSON (basic), Base64, and Hex encoding utilities
//! - `json` - JSON construction and response builders
//! - `serde` - Serialization helpers and rename utilities

pub mod encoding;
pub mod json;
pub mod serde;
pub mod strings;

pub use encoding::{Base64Utils, HexUtils, JsonUtils};
pub use json::{JsonBuilder, JsonResponse};
pub use serde::{OptionalField, compact, rename};
pub use strings::StringUtils;

/// Prelude module for convenient importing
pub mod prelude {
    //! Import common utilities with `use common::utils::prelude::*;`
    pub use super::{
        Base64Utils, HexUtils, JsonBuilder, JsonResponse, JsonUtils, OptionalField, StringUtils,
        compact, rename,
    };
}
