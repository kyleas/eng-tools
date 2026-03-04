//! Unified top-level engineering facade.
//!
//! Preferred import surface:
//! `use eng::{eq, equations, fluids, materials, constants, docs, qty};`
//!
//! ```no_run
//! use eng::{docs, eq, equations, fluids, materials};
//!
//! let re = eq
//!     .solve_with_context(equations::fluids::reynolds_number::equation())
//!     .fluid(fluids::water().state_tp("300 K", "1 bar").expect("state"))
//!     .for_target("Re")
//!     .given("V", "3 m/s")
//!     .given("D", "0.1 m")
//!     .value()
//!     .expect("solve");
//! assert!(re > 0.0);
//!
//! let p_cr = eq
//!     .solve_with_context(equations::structures::euler_buckling_load::equation())
//!     .material(materials::stainless_304().temperature("350 K").expect("state"))
//!     .for_target("P_cr")
//!     .given("I", "8e-6 m4")
//!     .given("K", 1.0)
//!     .given("L", "2 m")
//!     .value()
//!     .expect("solve");
//! assert!(p_cr > 0.0);
//!
//! let out = docs::export_unified_docs().expect("export unified artifacts");
//! assert!(out.join("catalog.json").exists());
//! ```

pub mod architecture;
pub mod docs;

pub use eng_core as core;
pub use eng_fluids as fluids;
pub use eng_materials as materials;
pub use eng_qty_macros::qty;
pub use equations::constants;
pub use equations::{self, eq};

pub mod units {
    pub use eng_core::units::typed;
}

pub mod prelude {
    pub use crate::constants;
    pub use crate::docs;
    pub use crate::eq;
    pub use crate::equations;
    pub use crate::fluids;
    pub use crate::materials;
    pub use crate::qty;
    pub use crate::units;
}
