use crate::model::{CaseTolerance, MethodKind, VariableConstraint};

pub const GLOBAL_RELATION_TOL_ABS: f64 = 1e-10;
pub const GLOBAL_RELATION_TOL_REL: f64 = 1e-8;
pub const GLOBAL_SOLVE_TOL_ABS: f64 = 1e-10;
pub const GLOBAL_SOLVE_TOL_REL: f64 = 1e-8;
pub const GLOBAL_NUMERICAL_MAX_ITER: u32 = 300;

#[derive(Debug, Clone, Copy)]
pub struct NumericalDimensionDefaults {
    pub initial_guess: f64,
    pub bracket: (f64, f64),
    pub max_iter: u32,
}

pub fn numerical_defaults_for_dimension(dimension: &str) -> NumericalDimensionDefaults {
    let dim = dimension.trim().to_ascii_lowercase();
    let (initial_guess, bracket) = match dim.as_str() {
        "pressure" | "stress" => (1.0e5, (1.0, 1.0e9)),
        "length" | "diameter" | "distance" | "roughness" => (1.0, (1.0e-9, 1.0e4)),
        "area" => (1.0, (1.0e-12, 1.0e6)),
        "velocity" => (10.0, (1.0e-9, 1.0e5)),
        "density" => (1.0, (1.0e-9, 1.0e6)),
        "viscosity" => (1.0e-3, (1.0e-12, 1.0e2)),
        "mass_flow_rate" | "mass_flux" => (1.0, (1.0e-12, 1.0e8)),
        "temperature" => (300.0, (1.0e-9, 1.0e5)),
        "force" => (1.0e3, (1.0e-9, 1.0e10)),
        "moment" => (1.0e3, (-1.0e10, 1.0e10)),
        "area_moment_of_inertia" | "polar_moment_of_inertia" => (1.0e-6, (1.0e-18, 1.0e4)),
        "gas_constant" => (300.0, (1.0e-9, 1.0e6)),
        "acceleration" => (9.81, (1.0e-9, 1.0e5)),
        "specific_impulse" => (250.0, (1.0e-9, 1.0e6)),
        "heat_rate" => (1000.0, (-1.0e9, 1.0e9)),
        "thermal_conductivity" => (1.0, (1.0e-9, 1.0e6)),
        "heat_transfer_coefficient" => (10.0, (1.0e-9, 1.0e8)),
        "thermal_resistance" => (0.1, (1.0e-12, 1.0e9)),
        "friction_factor" => (0.02, (1.0e-6, 1.0)),
        "mach" => (0.5, (1.0e-6, 50.0)),
        "ratio" | "dimensionless" => (1.0, (1.0e-9, 1.0e6)),
        "angle" | "position" => (0.0, (-1.0e6, 1.0e6)),
        _ => (1.0, (-1.0e6, 1.0e6)),
    };
    NumericalDimensionDefaults {
        initial_guess,
        bracket,
        max_iter: GLOBAL_NUMERICAL_MAX_ITER,
    }
}

pub fn default_methods_for_target(has_explicit: bool, has_numerical: bool) -> Vec<MethodKind> {
    let mut methods = Vec::new();
    if has_explicit {
        methods.push(MethodKind::Explicit);
    }
    if has_numerical {
        methods.push(MethodKind::Numerical);
    }
    methods
}

pub fn merge_tolerance(
    equation_level: Option<&CaseTolerance>,
    case_level: Option<&CaseTolerance>,
    target_level: Option<&CaseTolerance>,
) -> (f64, f64) {
    let abs = target_level
        .and_then(|t| t.abs)
        .or_else(|| case_level.and_then(|t| t.abs))
        .or_else(|| equation_level.and_then(|t| t.abs))
        .unwrap_or(GLOBAL_SOLVE_TOL_ABS);
    let rel = target_level
        .and_then(|t| t.rel)
        .or_else(|| case_level.and_then(|t| t.rel))
        .or_else(|| equation_level.and_then(|t| t.rel))
        .unwrap_or(GLOBAL_SOLVE_TOL_REL);
    (abs, rel)
}

pub fn constraint_defaults_for_dimension(dimension: &str) -> VariableConstraint {
    let dim = dimension.trim().to_ascii_lowercase();
    match dim.as_str() {
        // Most physical engineering quantities are strictly positive in normal use.
        "pressure"
        | "stress"
        | "length"
        | "diameter"
        | "distance"
        | "roughness"
        | "friction_factor"
        | "mach"
        | "temperature"
        | "density"
        | "viscosity"
        | "area"
        | "mass_flow_rate"
        | "mass_flux"
        | "force"
        | "area_moment_of_inertia"
        | "polar_moment_of_inertia"
        | "gas_constant"
        | "acceleration"
        | "specific_impulse"
        | "heat_rate"
        | "thermal_conductivity"
        | "heat_transfer_coefficient"
        | "thermal_resistance" => VariableConstraint {
            positive: true,
            ..VariableConstraint::default()
        },
        _ => VariableConstraint::default(),
    }
}
