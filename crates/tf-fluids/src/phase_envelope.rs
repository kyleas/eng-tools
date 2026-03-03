//! Phase envelope generation for plotting saturation curves.

use crate::calculator::{
    EquilibriumState, compute_saturated_liquid_at_pressure,
    compute_saturated_liquid_at_temperature, compute_saturated_vapor_at_pressure,
    compute_saturated_vapor_at_temperature,
};
use crate::error::FluidResult;
use crate::model::FluidModel;
use crate::species::Species;

/// Phase envelope data for a pure fluid.
#[derive(Debug, Clone)]
pub struct PhaseEnvelope {
    /// Saturated liquid states
    pub liquid_states: Vec<EquilibriumState>,
    /// Saturated vapor states
    pub vapor_states: Vec<EquilibriumState>,
}

/// Generate phase envelope by sweeping temperature.
///
/// Computes saturated liquid and vapor properties across a temperature range.
/// Useful for generating T-based diagrams (T-ρ, T-h, T-s).
pub fn generate_phase_envelope_by_temperature(
    model: &dyn FluidModel,
    species: Species,
    t_start_k: f64,
    t_end_k: f64,
    num_points: usize,
) -> FluidResult<PhaseEnvelope> {
    if num_points < 2 {
        return Ok(PhaseEnvelope {
            liquid_states: Vec::new(),
            vapor_states: Vec::new(),
        });
    }

    let mut liquid_states = Vec::with_capacity(num_points);
    let mut vapor_states = Vec::with_capacity(num_points);

    for i in 0..num_points {
        let t = t_start_k + (t_end_k - t_start_k) * (i as f64) / ((num_points - 1) as f64);

        // Try to compute saturated states at this temperature
        if let Ok(liquid) = compute_saturated_liquid_at_temperature(model, species, t) {
            liquid_states.push(liquid);
        }
        if let Ok(vapor) = compute_saturated_vapor_at_temperature(model, species, t) {
            vapor_states.push(vapor);
        }
    }

    Ok(PhaseEnvelope {
        liquid_states,
        vapor_states,
    })
}

/// Generate phase envelope by sweeping pressure.
///
/// Computes saturated liquid and vapor properties across a pressure range.
/// Useful for generating P-based diagrams (P-ρ, P-h, P-T).
pub fn generate_phase_envelope_by_pressure(
    model: &dyn FluidModel,
    species: Species,
    p_start_pa: f64,
    p_end_pa: f64,
    num_points: usize,
    use_log_spacing: bool,
) -> FluidResult<PhaseEnvelope> {
    if num_points < 2 {
        return Ok(PhaseEnvelope {
            liquid_states: Vec::new(),
            vapor_states: Vec::new(),
        });
    }

    let mut liquid_states = Vec::with_capacity(num_points);
    let mut vapor_states = Vec::with_capacity(num_points);

    for i in 0..num_points {
        let p = if use_log_spacing {
            // Logarithmic spacing
            let log_start = p_start_pa.ln();
            let log_end = p_end_pa.ln();
            (log_start + (log_end - log_start) * (i as f64) / ((num_points - 1) as f64)).exp()
        } else {
            // Linear spacing
            p_start_pa + (p_end_pa - p_start_pa) * (i as f64) / ((num_points - 1) as f64)
        };

        // Try to compute saturated states at this pressure
        if let Ok(liquid) = compute_saturated_liquid_at_pressure(model, species, p) {
            liquid_states.push(liquid);
        }
        if let Ok(vapor) = compute_saturated_vapor_at_pressure(model, species, p) {
            vapor_states.push(vapor);
        }
    }

    Ok(PhaseEnvelope {
        liquid_states,
        vapor_states,
    })
}

/// Extract property values from phase envelope states.
pub fn extract_property(states: &[EquilibriumState], property: &str) -> Vec<f64> {
    states
        .iter()
        .map(|state| match property.to_lowercase().as_str() {
            "temperature" | "t" => state.temperature_k(),
            "pressure" | "p" => state.pressure_pa(),
            "density" | "rho" | "ρ" => state.density_kg_m3(),
            "enthalpy" | "h" => state.enthalpy_j_per_kg,
            "entropy" | "s" => state.entropy_j_per_kg_k,
            "cp" => state.cp_j_per_kg_k,
            "cv" => state.cv_j_per_kg_k,
            "gamma" | "κ" => state.gamma,
            "speed_of_sound" | "a" | "c" => state.speed_of_sound_m_s(),
            _ => f64::NAN,
        })
        .collect()
}
