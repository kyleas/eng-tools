//! Raw FFI bindings to NASA CEA (Chemical Equilibrium with Applications)
//!
//! This crate provides low-level unsafe bindings to the official NASA CEA C API,
//! using prebuilt release binaries from `third_party/cea/`.
//!
//! # Architecture
//!
//! - Prebuilt CEA binaries in `third_party/cea/lib/<platform>/`
//! - C API headers in `third_party/cea/include/`
//! - Runtime data in `third_party/cea/data/thermo.lib`
//!
//! # Safety
//!
//! All functions in this crate are `unsafe`. Callers must respect:
//! - CEA global state (initialization once, thread-unsafe)
//! - Memory ownership (which structures are allocated by CEA vs. caller)
//! - Null-terminated strings
//! - Error codes (functions return cea_err)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::{c_char, c_double, c_int};

// ============================================================================
// Type Definitions (from cea.h)
// ============================================================================

/// NASA CEA error code type
pub type cea_err = c_int;

/// NASA CEA error codes
pub const CEA_OK: cea_err = 0;

/// Real number type in CEA (Fortran REAL -> double)
pub type cea_real = c_double;

/// Integer type in CEA (Fortran INTEGER -> int)
pub type cea_int = c_int;

/// String type in CEA (const char*)
pub type cea_string = *const c_char;

/// Array of doubles
pub type cea_array = *const c_double;

// Opaque C types (incomplete in Rust)
pub enum cea_mixture_t {}
pub type cea_mixture = *mut cea_mixture_t;

pub enum cea_eqsolver_t {}
pub type cea_eqsolver = *mut cea_eqsolver_t;

pub enum cea_eqsolution_t {}
pub type cea_eqsolution = *mut cea_eqsolution_t;

pub enum cea_rocket_solver_t {}
pub type cea_rocket_solver = *mut cea_rocket_solver_t;

pub enum cea_rocket_solution_t {}
pub type cea_rocket_solution = *mut cea_rocket_solution_t;

// Enumerations from cea_enum.h
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum cea_equilibrium_type {
    /// Temperature-Pressure (CEA_TP)
    TP = 0,
    /// Enthalpy-Pressure (CEA_HP)
    HP = 1,
    /// Entropy-Pressure (CEA_SP)
    SP = 2,
    /// Temperature-Volume (CEA_TV)
    TV = 3,
    /// Energy-Volume (CEA_UV)
    UV = 4,
    /// Entropy-Volume (CEA_SV)
    SV = 5,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum cea_property_type {
    /// Temperature (K)
    Temperature = 0,
    /// Pressure (Pa)
    Pressure = 1,
    /// Specific volume (m³/kg)
    Volume = 2,
    /// Density (kg/m³)
    Density = 3,
    /// Molar mass (kg/mol)
    M = 4,
    /// Molecular weight (kg/kmol)
    Mw = 5,
    /// Enthalpy (J/kg)
    Enthalpy = 6,
    /// Internal energy (J/kg)
    Energy = 7,
    /// Entropy (J/(kg·K))
    Entropy = 8,
    /// Gibbs energy (J/kg)
    GibbsEnergy = 9,
    /// Ratio of specific heats (gamma)
    GammaS = 10,
    /// Frozen specific heat at constant pressure (J/(kg·K))
    FrozenCp = 11,
    /// Frozen specific heat at constant volume (J/(kg·K))
    FrozenCv = 12,
    /// Equilibrium specific heat at constant pressure (J/(kg·K))
    EquilibriumCp = 13,
    /// Equilibrium specific heat at constant volume (J/(kg·K))
    EquilibriumCv = 14,
    /// Viscosity (Pa·s)
    Viscosity = 15,
    /// Frozen thermal conductivity (W/(m·K))
    FrozenConductivity = 16,
    /// Equilibrium thermal conductivity (W/(m·K))
    EquilibriumConductivity = 17,
    /// Frozen Prandtl number
    FrozenPrandtl = 18,
    /// Equilibrium Prandtl number
    EquilibriumPrandtl = 19,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum cea_rocket_property_type {
    /// Temperature (K)
    Temperature = 0,
    /// Pressure (Pa)
    Pressure = 1,
    /// Specific volume (m³/kg)
    Volume = 2,
    /// Density (kg/m³)
    Density = 3,
    /// Molar mass (kg/mol)
    M = 4,
    /// Molecular weight (kg/kmol)
    Mw = 5,
    /// Enthalpy (J/kg)
    Enthalpy = 6,
    /// Internal energy (J/kg)
    Energy = 7,
    /// Entropy (J/(kg·K))
    Entropy = 8,
    /// Gibbs energy (J/kg)
    GibbsEnergy = 9,
    /// Ratio of specific heats (gamma)
    GammaS = 10,
    /// Frozen Cp (J/(kg·K))
    FrozenCp = 11,
    /// Frozen Cv (J/(kg·K))
    FrozenCv = 12,
    /// Equilibrium Cp (J/(kg·K))
    EquilibriumCp = 13,
    /// Equilibrium Cv (J/(kg·K))
    EquilibriumCv = 14,
    /// Mach number
    Mach = 15,
    /// Sonic velocity (m/s)
    SonicVelocity = 16,
    /// Area ratio (Ae/At)
    AeAt = 17,
    /// Characteristic velocity C* (m/s)
    Cstar = 18,
    /// Coefficient of thrust
    CoefficientOfThrust = 19,
    /// Specific impulse (s)
    Isp = 20,
    /// Vacuum specific impulse (s)
    IspVacuum = 21,
    /// Viscosity (Pa·s)
    Viscosity = 22,
    /// Frozen thermal conductivity (W/(m·K))
    FrozenConductivity = 23,
    /// Equilibrium thermal conductivity (W/(m·K))
    EquilibriumConductivity = 24,
    /// Frozen Prandtl number
    FrozenPrandtl = 25,
    /// Equilibrium Prandtl number
    EquilibriumPrandtl = 26,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum cea_equilibrium_size {
    /// Number of reactants
    NumReactants = 0,
    /// Number of products
    NumProducts = 1,
    /// Number of gas species
    NumGas = 2,
    /// Number of condensed species
    NumCondensed = 3,
    /// Number of elements
    NumElements = 4,
    /// Maximum number of equations
    MaxEquations = 5,
}

// ============================================================================
// External C Functions (from cea library)
// ============================================================================

extern "C" {
    // ========== Version & Initialization ==========

    /// Initialize CEA library
    /// Must be called once before any other CEA functions
    pub fn cea_init() -> cea_err;

    /// Initialize CEA with explicit thermo database file
    pub fn cea_init_thermo(thermofile: cea_string) -> cea_err;

    /// Check if CEA is initialized
    pub fn cea_is_initialized(initialized: *mut cea_int) -> cea_err;

    // ========== Mixture Creation & Management ==========

    /// Create a mixture for given species
    ///
    /// # Arguments
    /// - `mix`: output mixture object
    /// - `nspecies`: number of species
    /// - `species`: array of species names
    pub fn cea_mixture_create(
        mix: *mut cea_mixture,
        nspecies: cea_int,
        species: *const cea_string,
    ) -> cea_err;

    /// Create mixture from reactant strings
    pub fn cea_mixture_create_from_reactants(
        mix: *mut cea_mixture,
        nreactants: cea_int,
        reactants: *const cea_string,
        nomit: cea_int,
        omit: *const cea_string,
    ) -> cea_err;

    /// Destroy a mixture object
    pub fn cea_mixture_destroy(mix: *mut cea_mixture) -> cea_err;

    /// Get number of species in mixture
    pub fn cea_mixture_get_num_species(mix: cea_mixture, num_species: *mut cea_int) -> cea_err;

    /// Calculate mixture property (e.g., enthalpy) for single temperature
    pub fn cea_mixture_calc_property(
        mix: cea_mixture,
        prop: cea_property_type,
        len_weights: cea_int,
        weights: cea_array,
        temperature: cea_real,
        value: *mut cea_real,
    ) -> cea_err;

    /// Calculate mixture property for multiple temperatures (one per reactant)
    pub fn cea_mixture_calc_property_multitemp(
        mix: cea_mixture,
        prop: cea_property_type,
        len_weights: cea_int,
        weights: cea_array,
        len_temperatures: cea_int,
        temperatures: cea_array,
        value: *mut cea_real,
    ) -> cea_err;

    // ========== Equilibrium Solver ==========

    /// Create equilibrium solver with given product specification
    pub fn cea_eqsolver_create(solver: *mut cea_eqsolver, products: cea_mixture) -> cea_err;

    /// Create equilibrium solver with reactants
    pub fn cea_eqsolver_create_with_reactants(
        solver: *mut cea_eqsolver,
        products: cea_mixture,
        reactants: cea_mixture,
    ) -> cea_err;

    /// Destroy equilibrium solver
    pub fn cea_eqsolver_destroy(solver: *mut cea_eqsolver) -> cea_err;

    /// Create solution object for equilibrium solver
    pub fn cea_eqsolution_create(soln: *mut cea_eqsolution, solver: cea_eqsolver) -> cea_err;

    /// Destroy solution object
    pub fn cea_eqsolution_destroy(soln: *mut cea_eqsolution) -> cea_err;

    /// Solve equilibrium problem
    ///
    /// # Arguments
    /// - `solver`: solver object
    /// - `eq_type`: type of equilibrium (TP, TV, HP, SP)
    /// - `state1`: first state variable (temperature or entropy)
    /// - `state2`: second state variable (pressure or volume)
    /// - `amounts`: array of reactant amounts (moles)
    /// - `soln`: output solution object
    pub fn cea_eqsolver_solve(
        solver: cea_eqsolver,
        eq_type: cea_equilibrium_type,
        state1: cea_real,
        state2: cea_real,
        amounts: cea_array,
        soln: cea_eqsolution,
    ) -> cea_err;

    /// Get property from equilibrium solution
    pub fn cea_eqsolution_get_property(
        soln: cea_eqsolution,
        prop: cea_property_type,
        value: *mut cea_real,
    ) -> cea_err;

    /// Get convergence status
    pub fn cea_eqsolution_get_converged(soln: cea_eqsolution, converged: *mut cea_int) -> cea_err;

    /// Get moles of products
    pub fn cea_eqsolution_get_moles(soln: cea_eqsolution, moles: *mut cea_real) -> cea_err;

    /// Get product compositions by weight
    pub fn cea_eqsolution_get_weights(
        soln: cea_eqsolution,
        np: cea_int,
        weights: *mut cea_real,
        log: bool,
    ) -> cea_err;

    /// Get species amounts (moles or mass) from solution
    pub fn cea_eqsolution_get_species_amounts(
        soln: cea_eqsolution,
        np: cea_int,
        amounts: *mut cea_real,
        mass: bool,
    ) -> cea_err;

    /// Get size information from equilibrium solver
    pub fn cea_eqsolver_get_size(
        solver: cea_eqsolver,
        eq_variable: cea_equilibrium_size,
        value: *mut cea_int,
    ) -> cea_err;

    // ========== Rocket Solver ==========

    /// Create rocket solver
    pub fn cea_rocket_solver_create(
        solver: *mut cea_rocket_solver,
        products: cea_mixture,
    ) -> cea_err;

    /// Create rocket solver with reactants
    pub fn cea_rocket_solver_create_with_reactants(
        solver: *mut cea_rocket_solver,
        products: cea_mixture,
        reactants: cea_mixture,
    ) -> cea_err;

    /// Destroy rocket solver
    pub fn cea_rocket_solver_destroy(solver: *mut cea_rocket_solver) -> cea_err;

    /// Create rocket solution object
    pub fn cea_rocket_solution_create(
        soln: *mut cea_rocket_solution,
        solver: cea_rocket_solver,
    ) -> cea_err;

    /// Destroy rocket solution
    pub fn cea_rocket_solution_destroy(soln: *mut cea_rocket_solution) -> cea_err;

    /// Solve rocket performance with shifting equilibrium
    /// (Simplified interface; actual NASA CEA has more parameters)
    pub fn cea_rocket_solver_solve_iac(
        solver: cea_rocket_solver,
        soln: cea_rocket_solution,
        weights: cea_array,
        pc: cea_real,
        pi_p: cea_array,
        n_pi_p: cea_int,
        subar: cea_array,
        nsubar: cea_int,
        supar: cea_array,
        nsupar: cea_int,
        n_frz: cea_int,
        hc_or_tc: cea_real,
        use_hc: bool,
        tc_est: cea_real,
        use_tc_est: bool,
    ) -> cea_err;

    /// Get rocket property from solution  
    /// For array properties (multiple stations), len must match number of points
    pub fn cea_rocket_solution_get_property(
        soln: cea_rocket_solution,
        prop: cea_rocket_property_type,
        len: cea_int,
        value: *mut cea_real,
    ) -> cea_err;

    /// Get number of solution points (stations) in rocket solution
    pub fn cea_rocket_solution_get_size(
        soln: cea_rocket_solution,
        num_pts: *mut cea_int,
    ) -> cea_err;

    /// Get species weights at a specific station
    pub fn cea_rocket_solution_get_weights(
        soln: cea_rocket_solution,
        np: cea_int,
        station: cea_int,
        weights: *mut cea_real,
        log: bool,
    ) -> cea_err;

    /// Get species amounts at a specific station
    pub fn cea_rocket_solution_get_species_amounts(
        soln: cea_rocket_solution,
        np: cea_int,
        station: cea_int,
        amounts: *mut cea_real,
        mass: bool,
    ) -> cea_err;

    /// Get convergence status
    pub fn cea_rocket_solution_get_converged(
        soln: cea_rocket_solution,
        converged: *mut cea_int,
    ) -> cea_err;

    // ========== Utility ==========

    /// Get last error message
    pub fn cea_strerror(err: cea_err) -> cea_string;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cea_ok_constant() {
        assert_eq!(CEA_OK, 0);
    }

    #[test]
    fn test_enum_values() {
        // Test equilibrium type enum matches CEA C API
        assert_eq!(cea_equilibrium_type::TP as i32, 0);
        assert_eq!(cea_equilibrium_type::HP as i32, 1);
        assert_eq!(cea_equilibrium_type::SP as i32, 2);
        assert_eq!(cea_equilibrium_type::TV as i32, 3);

        // Test property type enum
        assert_eq!(cea_property_type::Temperature as i32, 0);
        assert_eq!(cea_property_type::Pressure as i32, 1);
        assert_eq!(cea_property_type::Mw as i32, 5);
        assert_eq!(cea_property_type::GammaS as i32, 10);
    }
}
