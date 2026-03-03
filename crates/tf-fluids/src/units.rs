//! Compatibility re-export for fluid-facing unit parsing APIs.
//!
//! Canonical implementation now lives in `eng_core::units::quantity`.

pub use eng_core::units::{Quantity, UnitError, UnitValue, parse_quantity};
