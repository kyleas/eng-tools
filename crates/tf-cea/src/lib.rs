//! Thermoflow CEA integration crate.
//!
//! This crate owns Thermoflow's domain model for thermochemistry and rocket
//! performance analyses while delegating physics to an external CEA backend.

pub mod adapter;
pub mod backend;
pub mod config;
pub mod error;
pub mod model;
pub mod native;

pub use adapter::CeaProcessAdapter;
pub use backend::CeaBackend;
pub use config::{BackendExecutable, CeaBackendConfig};
pub use error::CeaError;
pub use model::*;
pub use native::NativeCeaBackend;
