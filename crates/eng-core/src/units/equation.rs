use crate::units::typed::{DimensionSignature, ExprInput};
use crate::units::{error::UnitError, parser::split_value_and_unit};
use eng_unit_expr::{ExprError, Signature};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EvaluatedQuantity {
    pub value_si: f64,
    pub signature: DimensionSignature,
}

#[derive(Debug, Clone, Copy)]
struct ReducedUnit {
    factor_to_si: f64,
    sig: DimensionSignature,
}

pub fn convert_equation_value_to_si(
    dimension: &str,
    unit: &str,
    value: f64,
) -> Result<f64, UnitError> {
    let (expected_sig, quantity_name) = expected_signature_for_dimension(dimension)?;
    let trimmed_unit = unit.trim();
    let reduced = reduce_unit_expression(trimmed_unit)?;

    if reduced.sig != expected_sig {
        return Err(UnitError::UnknownUnit {
            unit: unit.to_string(),
            quantity: format!(
                "{} (dimension mismatch: expected {:?}, got {:?})",
                quantity_name, expected_sig, reduced.sig
            ),
        });
    }

    Ok(value * reduced.factor_to_si)
}

pub fn parse_equation_value_and_unit(text: &str) -> Result<(f64, String), UnitError> {
    split_value_and_unit(text)
}

pub fn parse_equation_quantity_to_si(dimension: &str, text: &str) -> Result<f64, UnitError> {
    let evaluated = evaluate_quantity_expression(text)?;
    ensure_signature_matches_dimension(evaluated.signature, dimension)?;
    Ok(evaluated.value_si)
}

pub fn parse_quantity_expression(text: &str) -> Result<ExprInput, UnitError> {
    let evaluated = evaluate_quantity_expression(text)?;
    Ok(ExprInput::new(evaluated.value_si, evaluated.signature))
}

pub fn signature_for_dimension(dimension: &str) -> Result<DimensionSignature, UnitError> {
    let (sig, _) = expected_signature_for_dimension(dimension)?;
    Ok(sig)
}

pub fn ensure_signature_matches_dimension(
    signature: DimensionSignature,
    dimension: &str,
) -> Result<(), UnitError> {
    let (expected_sig, quantity_name) = expected_signature_for_dimension(dimension)?;
    if signature != expected_sig {
        return Err(UnitError::UnknownUnit {
            unit: format!("{signature:?}"),
            quantity: format!(
                "{} (dimension mismatch: expected {:?}, got {:?})",
                quantity_name, expected_sig, signature
            ),
        });
    }
    Ok(())
}

pub fn convert_equation_value_from_si(
    dimension: &str,
    unit: &str,
    value_si: f64,
) -> Result<f64, UnitError> {
    let factor = convert_equation_value_to_si(dimension, unit, 1.0)?;
    Ok(value_si / factor)
}

pub fn default_unit_for_dimension(dimension: &str) -> Option<&'static str> {
    let dim = normalize_dimension(dimension);
    match dim.as_str() {
        "dimensionless" | "ratio" | "friction_factor" | "mach" => Some("1"),
        "pressure" | "stress" => Some("Pa"),
        "length" | "diameter" | "distance" | "roughness" => Some("m"),
        "area" => Some("m2"),
        "volume" => Some("m3"),
        "mass" => Some("kg"),
        "velocity" => Some("m/s"),
        "density" => Some("kg/m3"),
        "viscosity" | "dynamic_viscosity" => Some("Pa*s"),
        "mass_flow_rate" => Some("kg/s"),
        "mass_flux" => Some("kg/(m2*s)"),
        "temperature" => Some("K"),
        "force" => Some("N"),
        "moment" => Some("N*m"),
        "area_moment_of_inertia" | "polar_moment_of_inertia" => Some("m4"),
        "gas_constant" => Some("J/(kg*K)"),
        "universal_gas_constant" => Some("J/(mol*K)"),
        "acceleration" => Some("m/s2"),
        "specific_impulse" => Some("s"),
        "heat_rate" => Some("W"),
        "thermal_conductivity" => Some("W/(m*K)"),
        "heat_transfer_coefficient" => Some("W/(m2*K)"),
        "thermal_resistance" => Some("K/W"),
        "volumetric_flow_rate" => Some("m3/s"),
        "specific_heat_capacity" => Some("J/(kg*K)"),
        _ => None,
    }
}

fn expected_signature_for_dimension(
    dimension: &str,
) -> Result<(DimensionSignature, String), UnitError> {
    let dim = normalize_dimension(dimension);
    let out = match dim.as_str() {
        "dimensionless" | "ratio" | "friction_factor" | "mach" => {
            (DimensionSignature::dimless(), "dimensionless")
        }
        "pressure" | "stress" => (DimensionSignature::new(1, -1, -2, 0, 0), "pressure/stress"),
        "length" | "diameter" | "distance" | "roughness" => {
            (DimensionSignature::new(0, 1, 0, 0, 0), "length")
        }
        "area" => (DimensionSignature::new(0, 2, 0, 0, 0), "area"),
        "volume" => (DimensionSignature::new(0, 3, 0, 0, 0), "volume"),
        "mass" => (DimensionSignature::new(1, 0, 0, 0, 0), "mass"),
        "velocity" => (DimensionSignature::new(0, 1, -1, 0, 0), "velocity"),
        "density" => (DimensionSignature::new(1, -3, 0, 0, 0), "density"),
        "viscosity" | "dynamic_viscosity" => (
            DimensionSignature::new(1, -1, -1, 0, 0),
            "dynamic viscosity",
        ),
        "mass_flow_rate" => (DimensionSignature::new(1, 0, -1, 0, 0), "mass flow rate"),
        "mass_flux" => (DimensionSignature::new(1, -2, -1, 0, 0), "mass flux"),
        "temperature" => (DimensionSignature::new(0, 0, 0, 1, 0), "temperature"),
        "force" => (DimensionSignature::new(1, 1, -2, 0, 0), "force"),
        "moment" => (DimensionSignature::new(1, 2, -2, 0, 0), "moment"),
        "area_moment_of_inertia" | "polar_moment_of_inertia" => {
            (DimensionSignature::new(0, 4, 0, 0, 0), "second moment")
        }
        "gas_constant" | "specific_heat_capacity" => (
            DimensionSignature::new(0, 2, -2, -1, 0),
            "specific gas constant/cp",
        ),
        "universal_gas_constant" => (
            DimensionSignature::new(1, 2, -2, -1, -1),
            "universal gas constant",
        ),
        "acceleration" => (DimensionSignature::new(0, 1, -2, 0, 0), "acceleration"),
        "specific_impulse" => (DimensionSignature::new(0, 0, 1, 0, 0), "specific impulse"),
        "heat_rate" => (DimensionSignature::new(1, 2, -3, 0, 0), "power"),
        "thermal_conductivity" => (
            DimensionSignature::new(1, 1, -3, -1, 0),
            "thermal conductivity",
        ),
        "heat_transfer_coefficient" => (
            DimensionSignature::new(1, 0, -3, -1, 0),
            "heat transfer coefficient",
        ),
        "thermal_resistance" => (
            DimensionSignature::new(-1, -2, 3, 1, 0),
            "thermal resistance",
        ),
        "volumetric_flow_rate" => (
            DimensionSignature::new(0, 3, -1, 0, 0),
            "volumetric flow rate",
        ),
        _ => {
            return Err(UnitError::UnsupportedDimension {
                dimension: dimension.to_string(),
            });
        }
    };
    Ok((out.0, out.1.to_string()))
}

fn reduce_unit_expression(unit: &str) -> Result<ReducedUnit, UnitError> {
    let trimmed = unit.trim();
    if trimmed.is_empty() || trimmed == "1" || trimmed == "-" {
        return Ok(ReducedUnit {
            factor_to_si: 1.0,
            sig: DimensionSignature::dimless(),
        });
    }

    // Keep equation temperature strict: only absolute K for now.
    if trimmed.eq_ignore_ascii_case("c")
        || trimmed.eq_ignore_ascii_case("f")
        || trimmed.eq_ignore_ascii_case("degc")
        || trimmed.eq_ignore_ascii_case("degf")
    {
        return Err(UnitError::AmbiguousUnit {
            unit: trimmed.to_string(),
            reason:
                "Affine temperatures are not supported in equation conversions; use Kelvin ('K')"
                    .to_string(),
        });
    }

    let q = eng_unit_expr::evaluate(&format!("1 {}", trimmed)).map_err(map_expr_error)?;
    Ok(ReducedUnit {
        factor_to_si: q.value_si,
        sig: map_sig(q.signature),
    })
}

fn evaluate_quantity_expression(text: &str) -> Result<EvaluatedQuantity, UnitError> {
    let q = eng_unit_expr::evaluate(text).map_err(map_expr_error)?;
    Ok(EvaluatedQuantity {
        value_si: q.value_si,
        signature: map_sig(q.signature),
    })
}

fn map_sig(sig: Signature) -> DimensionSignature {
    DimensionSignature::new(sig.m, sig.l, sig.t, sig.th, sig.n)
}

fn map_expr_error(err: ExprError) -> UnitError {
    match err {
        ExprError::Parse(msg) => UnitError::ParseError(msg),
        ExprError::UnknownUnit(unit) => UnitError::UnknownUnit {
            unit,
            quantity: "unit expression".to_string(),
        },
        ExprError::AmbiguousUnit(reason) => UnitError::AmbiguousUnit {
            unit: "expression".to_string(),
            reason,
        },
    }
}

fn normalize_dimension(dimension: &str) -> String {
    dimension.trim().to_ascii_lowercase().replace(' ', "_")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_pressure() {
        let pa = convert_equation_value_to_si("pressure", "psi", 100.0).expect("convert");
        assert!((pa - 689_475.729_316_8).abs() < 1e-6);
    }

    #[test]
    fn parses_quantity() {
        let m = parse_equation_quantity_to_si("length", "12 in").expect("parse");
        assert!((m - 0.3048).abs() < 1e-12);
    }

    #[test]
    fn converts_pressure_from_si() {
        let psi =
            convert_equation_value_from_si("pressure", "psi", 689_475.729_316_8).expect("convert");
        assert!((psi - 100.0).abs() < 1e-9);
    }

    #[test]
    fn viscosity_equivalent_units_reduce() {
        let a = convert_equation_value_to_si("viscosity", "Pa*s", 1.0).unwrap();
        let b = convert_equation_value_to_si("viscosity", "kg/(m*s)", 1.0).unwrap();
        let c = convert_equation_value_to_si("viscosity", "N*s/m2", 1.0).unwrap();
        assert!((a - b).abs() < 1e-12);
        assert!((a - c).abs() < 1e-12);
    }

    #[test]
    fn volumetric_flow_equivalent_units_reduce() {
        let a = convert_equation_value_to_si("volumetric_flow_rate", "m3/s", 1.0).unwrap();
        let b = convert_equation_value_to_si("volumetric_flow_rate", "L/s", 1000.0).unwrap();
        let c = convert_equation_value_to_si("volumetric_flow_rate", "gal/min", 60.0).unwrap();
        assert!((a - b).abs() < 1e-12);
        assert!(c > 0.0);
    }

    #[test]
    fn pressure_quantity_expression_is_supported() {
        let v = parse_equation_quantity_to_si("pressure", "5 MPa + 12 psi").unwrap();
        let expected = 5.0e6 + 12.0 * 6_894.757_293_168;
        assert!((v - expected).abs() < 1e-6);
    }

    #[test]
    fn length_quantity_expression_is_supported() {
        let v = parse_equation_quantity_to_si("length", "3 ft + 2 in").unwrap();
        let expected = 3.0 * 0.3048 + 2.0 * 0.0254;
        assert!((v - expected).abs() < 1e-12);
    }

    #[test]
    fn viscosity_quantity_expression_is_supported() {
        let v = parse_equation_quantity_to_si("viscosity", "1 Pa*s + 2 cP").unwrap();
        assert!((v - 1.002).abs() < 1e-12);
    }

    #[test]
    fn volumetric_flow_expression_is_supported() {
        let v = parse_equation_quantity_to_si("volumetric_flow_rate", "2 gal/min + 1 L/s").unwrap();
        let expected = 2.0 * 0.003_785_411_784 / 60.0 + 1.0e-3;
        assert!((v - expected).abs() < 1e-12);
    }

    #[test]
    fn invalid_mixed_dimension_addition_is_rejected() {
        let err = parse_equation_quantity_to_si("pressure", "5 MPa + 3 m").unwrap_err();
        assert!(err.to_string().contains("differing dimensions"));
    }

    #[test]
    fn invalid_quantity_expression_syntax_is_rejected() {
        let err = parse_equation_quantity_to_si("pressure", "5 MPa + * 2 psi").unwrap_err();
        assert!(err.to_string().contains("invalid quantity expression"));
    }

    #[test]
    fn unknown_unit_in_expression_is_rejected() {
        let err = parse_equation_quantity_to_si("pressure", "5 blarg + 2 psi").unwrap_err();
        assert!(err.to_string().contains("Unknown unit"));
    }
}
