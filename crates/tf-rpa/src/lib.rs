//! Backend-first RPA-style rocket performance orchestration for Thermoflow.
//!
//! This crate does not implement CEA physics. It orchestrates an existing
//! `tf-cea` backend, validates analysis inputs, and maps backend outputs into
//! an RPA-style result model suitable for future GUI/CLI workflows.

pub mod error;
pub mod geometry;
pub mod model;
pub mod solver;
pub mod study;
pub mod thermal;

pub use error::RpaError;
pub use geometry::*;
pub use model::*;
pub use solver::RocketAnalysisSolver;

pub use study::*;
pub use thermal::*;
