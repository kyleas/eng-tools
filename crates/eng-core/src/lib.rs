//! eng-core (`eng_core`): shared engineering foundation crate.
//!
//! Contains:
//! - units (typed SI units + quantity parsing/conversion)
//! - numeric (Real + tolerances + float helpers)
//! - ids (stable compact IDs for graph/model objects)
//! - error (shared error types)
//! - timing (lightweight performance measurement)

pub mod error;
pub mod ids;
pub mod numeric;
pub mod timing;
pub mod units;

pub use error::{TfError, TfResult};
pub use ids::*;
pub use numeric::*;
pub use timing::*;
pub use units::*;
