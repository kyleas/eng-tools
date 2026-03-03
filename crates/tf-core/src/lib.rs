//! Compatibility shim for legacy `tf-core` imports.
//!
//! Canonical shared engineering core now lives in the `eng-core` package
//! (`eng_core` crate path). This crate re-exports `eng_core` to keep
//! existing downstream paths stable.

pub use eng_core::*;
