//! `equations` provides a registry-driven engineering equation system.
//!
//! # Quick Usage
//!
//! ```no_run
//! use equations::{eq, run_registry_tests};
//! use eng_core::units::{qty, typed::{length, pressure}};
//!
//! eq.validate_with_tests().expect("validate");
//!
//! let sigma_h = eq
//!     .solve(equations::structures::hoop_stress::equation())
//!     .target_sigma_h()
//!     .given_p(2.5e6)
//!     .given_r(0.2)
//!     .given_t(0.008)
//!     .value()
//!     .expect("solve");
//! assert!(sigma_h > 0.0);
//!
//! // Typed constructor path (explicit units, no runtime string parsing needed).
//! let sigma_h_typed = eq
//!     .solve(equations::structures::hoop_stress::equation())
//!     .target_sigma_h()
//!     .given_p(pressure::mpa(2.5))
//!     .given_r(length::m(0.2))
//!     .given_t(length::mm(8.0))
//!     .value()
//!     .expect("solve with typed quantities");
//! assert!(sigma_h_typed > 0.0);
//!
//! // Compile-time quantity literal path.
//! let sigma_h_qty = eq
//!     .solve(equations::structures::hoop_stress::equation())
//!     .target_sigma_h()
//!     .given_p(qty!("2.5 MPa"))
//!     .given_r(qty!("0.2 m"))
//!     .given_t(qty!("8 mm"))
//!     .value_in("MPa")
//!     .expect("solve with qty! literals");
//! assert!(sigma_h_qty > 0.0);
//!
//! // Runtime string path (boundary convenience).
//! let sigma_h_mpa = eq
//!     .solve(equations::structures::hoop_stress::equation())
//!     .target_sigma_h()
//!     .given_p("2.5 MPa")
//!     .given_r("0.2 m")
//!     .given_t("8 mm")
//!     .value_in("MPa")
//!     .expect("solve in unit");
//! assert!(sigma_h_mpa > 0.0);
//!
//! let result = eq
//!     .solve(equations::compressible::area_mach::equation())
//!     .target_m()
//!     .branch_supersonic()
//!     .given_area_ratio(2.0049745454545462)
//!     .given_gamma(1.4)
//!     .result()
//!     .expect("branch solve");
//! assert!(result.value_si > 1.0);
//!
//! // Constants like g0/pi are auto-resolved from equations::constants by default.
//! let isp = eq
//!     .solve(equations::rockets::specific_impulse_ideal::equation())
//!     .target_i_sp()
//!     .given_c_f(1.7684408756881704)
//!     .given_c_star(1718.7683350153386)
//!     .value()
//!     .expect("solve with auto constant");
//! assert!(isp > 0.0);
//!
//! // Advanced path: explicitly override a constant for one solve.
//! let isp_override = eq
//!     .solve(equations::rockets::specific_impulse_ideal::equation())
//!     .target_i_sp()
//!     .given_c_f(1.7684408756881704)
//!     .given_c_star(1718.7683350153386)
//!     .override_constant("g0", 9.81)
//!     .value()
//!     .expect("solve with overridden constant");
//! assert!(isp_override > 0.0);
//!
//! let sigma_h_quick = eq.solve_value(
//!     "structures.hoop_stress",
//!     "sigma_h",
//!     [("P", 2.5e6), ("r", 0.2), ("t", 0.008)],
//! ).expect("solve");
//! assert!(sigma_h_quick > 0.0);
//!
//! let registry = eq.registry().expect("registry");
//! let summary = run_registry_tests(&registry).expect("tests");
//! assert_eq!(summary.failed, 0);
//! ```

pub mod api;
pub mod api_context;
pub mod constants;
pub mod defaults;
pub mod docs;
pub mod equation_families;
pub mod error;
pub mod expr;
pub mod facade;
pub mod model;
pub mod normalize;
pub mod registry;
pub mod schema;
pub mod solve_engine;
pub mod testing;

pub use api::{
    IntoEquationId, IntoSolveInput, SolveBuilder, SolveInputValue, SolveResult, SolveStart,
};
pub use api_context::{ContextBinding, ContextSolveBuilder};
pub use constants::{EngineeringConstant, all as all_constants, get as get_constant};
pub use docs::export_docs_artifacts;
pub use error::{EquationError, Result};
pub use facade::EqFacade;
pub use registry::Registry;
pub use registry::lint::LintWarning;
pub use schema::generate_schema_to_path;
pub use solve_engine::{SolveMethod, SolveRequest, SolveResponse, solve_equation};
pub use testing::{RegistryTestSummary, run_registry_tests};

pub static EQ: EqFacade = EqFacade;
pub use EQ as eq;

#[doc(hidden)]
mod generated_typed {
    include!(concat!(env!("OUT_DIR"), "/typed_equations.rs"));
}
pub use generated_typed::*;

#[doc(hidden)]
pub mod generated_families {
    include!(concat!(env!("OUT_DIR"), "/typed_families.rs"));
}
pub use generated_families as families;

#[cfg(test)]
mod tests;
