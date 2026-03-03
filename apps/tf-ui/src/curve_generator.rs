//! Curve generation from backend models.
//!
//! Generates CurveData from various source types by querying
//! backend component models and simulating actuator responses.

use crate::curve_source::{CurveData, CurveSource, FluidSweepParameters, ValveCharacteristicKind};
use tf_components::valve::{Valve, ValveLaw};
use tf_fluids::{CoolPropModel, Quantity, Species, SweepDefinition, SweepType, parse_quantity};
use tf_project::schema::{ComponentDef, ComponentKind, ValveLawDef};
use tf_sim::{ActuatorState, FirstOrderActuator};
use uom::si::f64::Area;

/// Generate curve data from a curve source and project context.
pub fn generate_curve_data(
    source: &CurveSource,
    project: Option<&tf_project::schema::Project>,
) -> Option<CurveData> {
    match source {
        CurveSource::ValveCharacteristic {
            component_id,
            characteristic,
            sample_count,
        } => generate_valve_characteristic(component_id, *characteristic, *sample_count, project),
        CurveSource::ActuatorResponse {
            tau_s,
            rate_limit_per_s,
            initial_position,
            command,
            duration_s,
            sample_count,
        } => Some(generate_actuator_response(
            *tau_s,
            *rate_limit_per_s,
            *initial_position,
            *command,
            *duration_s,
            *sample_count,
        )),
        CurveSource::FluidPropertySweep {
            x_property,
            y_property,
            parameters,
        } => generate_fluid_property_sweep(x_property, y_property, parameters),
    }
}

/// Generate valve characteristic curve data.
fn generate_valve_characteristic(
    component_id: &str,
    characteristic: ValveCharacteristicKind,
    sample_count: usize,
    project: Option<&tf_project::schema::Project>,
) -> Option<CurveData> {
    // Find the valve component in the project
    let valve_def = project
        .and_then(|p| p.systems.first())
        .and_then(|sys| sys.components.iter().find(|comp| comp.id == component_id))?;

    // Extract valve parameters
    let (cd, area_max, law) = extract_valve_parameters(valve_def)?;

    // Generate sample points
    let mut x_values = Vec::with_capacity(sample_count);
    let mut y_values = Vec::with_capacity(sample_count);

    for i in 0..sample_count {
        let position = (i as f64) / ((sample_count - 1) as f64);
        x_values.push(position);

        let y_value = match characteristic {
            ValveCharacteristicKind::EffectiveArea => {
                // Create temporary valve and get effective area
                let mut valve =
                    Valve::new("temp".to_string(), cd, area_max, position).with_law(law);
                valve.set_position(position);
                compute_valve_effective_area(&valve)
            }
            ValveCharacteristicKind::OpeningFactor => {
                // Just compute the opening factor based on law
                compute_opening_factor(position, law)
            }
        };

        y_values.push(y_value);
    }

    Some(CurveData {
        x_values,
        y_values,
        label: match characteristic {
            ValveCharacteristicKind::EffectiveArea => {
                format!("{} CdA", component_id)
            }
            ValveCharacteristicKind::OpeningFactor => {
                format!("{} Opening", component_id)
            }
        },
    })
}

/// Extract valve parameters from component definition.
fn extract_valve_parameters(comp_def: &ComponentDef) -> Option<(f64, Area, ValveLaw)> {
    if let ComponentKind::Valve {
        cd,
        area_max_m2,
        law,
        ..
    } = &comp_def.kind
    {
        use uom::si::area::square_meter;

        let area_max = Area::new::<square_meter>(*area_max_m2);
        let valve_law = match law {
            ValveLawDef::Linear => ValveLaw::Linear,
            ValveLawDef::Quadratic => ValveLaw::Quadratic,
            ValveLawDef::QuickOpening => {
                // QuickOpening not yet supported in backend
                return None;
            }
        };
        Some((*cd, area_max, valve_law))
    } else {
        None
    }
}

/// Compute effective area for a valve at given position.
fn compute_valve_effective_area(valve: &Valve) -> f64 {
    // Use the valve's internal effective_area calculation
    // This requires accessing the private method, so we replicate the logic
    let min_area_factor = 1e-4;
    let factor = match valve.law {
        ValveLaw::Linear => valve.position,
        ValveLaw::Quadratic => valve.position * valve.position,
    };
    let effective_factor = factor.max(min_area_factor);
    valve.area_max.value * effective_factor * valve.cd
}

/// Compute opening factor based on valve law.
fn compute_opening_factor(position: f64, law: ValveLaw) -> f64 {
    match law {
        ValveLaw::Linear => position,
        ValveLaw::Quadratic => position * position,
    }
}

/// Generate actuator step response curve.
fn generate_actuator_response(
    tau_s: f64,
    rate_limit_per_s: f64,
    initial_position: f64,
    command: f64,
    duration_s: f64,
    sample_count: usize,
) -> CurveData {
    let actuator = FirstOrderActuator::new(tau_s, rate_limit_per_s).ok();

    let mut x_values = Vec::with_capacity(sample_count);
    let mut y_values = Vec::with_capacity(sample_count);

    if let Some(act) = actuator {
        let dt = duration_s / (sample_count - 1) as f64;
        let mut state = ActuatorState {
            position: initial_position.clamp(0.0, 1.0),
        };

        for i in 0..sample_count {
            let time = (i as f64) * dt;
            x_values.push(time);
            y_values.push(state.position);

            if i < sample_count - 1 {
                state = act.step(&state, dt, command);
            }
        }
    } else {
        // Fallback if actuator creation fails
        for i in 0..sample_count {
            let time = (i as f64) * duration_s / (sample_count - 1) as f64;
            x_values.push(time);
            y_values.push(initial_position);
        }
    }

    CurveData {
        x_values,
        y_values,
        label: format!("Actuator Response (τ={:.2}s)", tau_s),
    }
}

/// Generate fluid property sweep curve data.
fn generate_fluid_property_sweep(
    x_property: &str,
    y_property: &str,
    params: &FluidSweepParameters,
) -> Option<CurveData> {
    // Parse species
    let species = params.species.parse::<Species>().ok()?;

    // Determine sweep quantity based on sweep_variable
    let sweep_quantity = match params.sweep_variable.to_lowercase().as_str() {
        "temperature" => Quantity::Temperature,
        "pressure" => Quantity::Pressure,
        "enthalpy" | "h" => Quantity::SpecificEnthalpy,
        "entropy" | "s" => Quantity::SpecificEntropy,
        "density" | "rho" => Quantity::Density,
        _ => return None,
    };

    // Parse start and end values with units
    let start_si = parse_quantity(&params.start_value, sweep_quantity).ok()?;
    let end_si = parse_quantity(&params.end_value, sweep_quantity).ok()?;

    // Determine sweep type
    let sweep_type = match params.sweep_type.to_lowercase().as_str() {
        "logarithmic" | "log" => SweepType::Logarithmic,
        _ => SweepType::Linear,
    };

    // Create sweep definition
    let sweep_def = SweepDefinition {
        start_si,
        start_raw: params.start_value.clone(),
        end_si,
        end_raw: params.end_value.clone(),
        quantity: sweep_quantity,
        num_points: params.num_points,
        sweep_type,
    };

    // Parse fixed property if provided
    if let (Some(fixed_name), Some(fixed_val)) =
        (&params.fixed_property_name, &params.fixed_property_value)
    {
        let fixed_quantity = match fixed_name.to_lowercase().as_str() {
            "temperature" => Quantity::Temperature,
            "pressure" => Quantity::Pressure,
            "enthalpy" | "h" => Quantity::SpecificEnthalpy,
            "entropy" | "s" => Quantity::SpecificEntropy,
            "density" | "rho" => Quantity::Density,
            _ => return None,
        };

        let _fixed_si = parse_quantity(fixed_val, fixed_quantity).ok()?;

        // Execute sweep based on which properties are primary/fixed
        // This is a simplified version - a full implementation would support all combinations
        match (sweep_quantity, fixed_quantity) {
            (Quantity::Temperature, Quantity::Pressure) => {
                // Temperature sweep at fixed pressure
                let model = CoolPropModel::new();
                let result = tf_fluids::execute_temperature_sweep_at_pressure(
                    &model, species, &sweep_def, _fixed_si,
                )
                .ok()?;

                // Extract x and y properties from result
                extract_curve_data_from_sweep(&result, x_property, y_property, &params.species)
            }
            (Quantity::Pressure, Quantity::Temperature) => {
                // Pressure sweep at fixed temperature
                let model = CoolPropModel::new();
                let result = tf_fluids::execute_pressure_sweep_at_temperature(
                    &model, species, &sweep_def, _fixed_si,
                )
                .ok()?;

                extract_curve_data_from_sweep(&result, x_property, y_property, &params.species)
            }
            _ => None, // Other combinations not yet supported
        }
    } else {
        None // Require fixed property for now
    }
}

/// Extract curve data from a sweep result based on requested properties.
fn extract_curve_data_from_sweep(
    result: &tf_fluids::SweepResult,
    x_property: &str,
    y_property: &str,
    species_name: &str,
) -> Option<CurveData> {
    let x_values = match x_property.to_lowercase().as_str() {
        "temperature" | "t" => result.temperature_k(),
        "pressure" | "p" => result.pressure_pa(),
        "density" | "rho" | "ρ" => result.density_kg_m3(),
        "enthalpy" | "h" => result.enthalpy_j_per_kg(),
        "entropy" | "s" => result.entropy_j_per_kg_k(),
        "cp" => result.cp_j_per_kg_k(),
        "cv" => result.cv_j_per_kg_k(),
        "gamma" | "κ" => result.gamma(),
        "speed_of_sound" | "a" | "c" => result.speed_of_sound_m_s(),
        _ => return None,
    };

    let y_values = match y_property.to_lowercase().as_str() {
        "temperature" | "t" => result.temperature_k(),
        "pressure" | "p" => result.pressure_pa(),
        "density" | "rho" | "ρ" => result.density_kg_m3(),
        "enthalpy" | "h" => result.enthalpy_j_per_kg(),
        "entropy" | "s" => result.entropy_j_per_kg_k(),
        "cp" => result.cp_j_per_kg_k(),
        "cv" => result.cv_j_per_kg_k(),
        "gamma" | "κ" => result.gamma(),
        "speed_of_sound" | "a" | "c" => result.speed_of_sound_m_s(),
        _ => return None,
    };

    if x_values.len() != y_values.len() || x_values.is_empty() {
        return None;
    }

    let label = format!("{} vs {} ({})", y_property, x_property, species_name);

    Some(CurveData {
        x_values,
        y_values,
        label,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tf_project::schema::{ComponentKind, Project, SystemDef, ValveLawDef};

    #[test]
    fn actuator_response_generation() {
        let curve = generate_actuator_response(0.5, 2.0, 0.0, 1.0, 5.0, 100);
        assert_eq!(curve.x_values.len(), 100);
        assert_eq!(curve.y_values.len(), 100);
        assert_eq!(curve.x_values[0], 0.0);
        assert!(curve.x_values[99] <= 5.0);
        assert_eq!(curve.y_values[0], 0.0);
        // Should approach command value
        assert!(curve.y_values[99] > 0.8);
    }

    #[test]
    fn opening_factor_linear() {
        let factor = compute_opening_factor(0.5, ValveLaw::Linear);
        assert!((factor - 0.5).abs() < 1e-10);
    }

    #[test]
    fn opening_factor_quadratic() {
        let factor = compute_opening_factor(0.5, ValveLaw::Quadratic);
        assert!((factor - 0.25).abs() < 1e-10);
    }

    #[test]
    fn valve_characteristic_generation_with_project() {
        use crate::curve_source::ValveCharacteristicKind;

        // Create a minimal project with a valve
        let mut project = Project {
            version: tf_project::LATEST_VERSION,
            name: "Test Project".to_string(),
            systems: vec![],
            modules: vec![],
            layouts: vec![],
            runs: tf_project::schema::RunLibraryDef::default(),
            plotting_workspace: None,
            fluid_workspace: None,
            rocket_workspace: None,
        };

        let mut system = SystemDef {
            id: "sys1".to_string(),
            name: "Test System".to_string(),
            fluid: tf_project::schema::FluidDef {
                composition: tf_project::schema::CompositionDef::Pure {
                    species: "Nitrogen".to_string(),
                },
            },
            nodes: vec![],
            components: vec![],
            boundaries: vec![],
            schedules: vec![],
            controls: None,
        };

        // Add a valve component
        system.components.push(tf_project::schema::ComponentDef {
            id: "valve1".to_string(),
            name: "Test Valve".to_string(),
            from_node_id: "n1".to_string(),
            to_node_id: "n2".to_string(),
            kind: ComponentKind::Valve {
                cd: 0.8,
                area_max_m2: 0.01,
                position: 0.5,
                law: ValveLawDef::Linear,
                treat_as_gas: false,
            },
        });

        project.systems.push(system);

        // Generate valve characteristic curve
        let curve = generate_valve_characteristic(
            "valve1",
            ValveCharacteristicKind::EffectiveArea,
            50,
            Some(&project),
        );

        assert!(curve.is_some());
        let curve_data = curve.unwrap();
        assert_eq!(curve_data.x_values.len(), 50);
        assert_eq!(curve_data.y_values.len(), 50);
        assert!(curve_data.label.contains("valve1"));
    }

    #[test]
    fn valve_characteristic_nonexistent_component() {
        use crate::curve_source::ValveCharacteristicKind;

        let project = Project {
            version: tf_project::LATEST_VERSION,
            name: "Test Project".to_string(),
            systems: vec![],
            modules: vec![],
            layouts: vec![],
            runs: tf_project::schema::RunLibraryDef::default(),
            plotting_workspace: None,
            fluid_workspace: None,
            rocket_workspace: None,
        };

        // Try to generate curve for nonexistent valve
        let curve = generate_valve_characteristic(
            "nonexistent",
            ValveCharacteristicKind::EffectiveArea,
            50,
            Some(&project),
        );

        // Should return None gracefully
        assert!(curve.is_none());
    }
}
