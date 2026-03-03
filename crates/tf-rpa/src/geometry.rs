use serde::{Deserialize, Serialize};

use crate::{NozzleConstraint, RocketAnalysisProblem, RpaError};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum GeometrySizingMode {
    #[default]
    GivenThroatDiameter,
    GivenThroatArea,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum NozzleContourStyle {
    #[default]
    Conical,
    BellParabolic,
    TruncatedIdeal,
}

impl NozzleContourStyle {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Conical => "Conical",
            Self::BellParabolic => "Bell (parabolic)",
            Self::TruncatedIdeal => "Truncated ideal contour",
        }
    }
}

impl GeometrySizingMode {
    pub fn label(&self) -> &'static str {
        match self {
            Self::GivenThroatDiameter => "Given throat diameter",
            Self::GivenThroatArea => "Given throat area",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RocketGeometryProblem {
    pub base_problem: RocketAnalysisProblem,
    pub sizing_mode: GeometrySizingMode,
    pub throat_input_value: f64,
    pub chamber_contraction_ratio: f64,
    pub characteristic_length_m: f64,
    pub nozzle_half_angle_deg: f64,
    pub nozzle_contour_style: NozzleContourStyle,
    pub nozzle_truncation_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RocketGeometryResult {
    pub throat_area_m2: f64,
    pub throat_diameter_m: f64,
    pub exit_area_m2: f64,
    pub exit_diameter_m: f64,
    pub expansion_ratio: f64,
    pub chamber_area_m2_estimate: f64,
    pub chamber_diameter_m_estimate: f64,
    pub chamber_length_m_estimate: f64,
    pub nozzle_length_m_estimate: f64,
    pub chamber_pressure_pa_reference: f64,
    pub canonical_model: EngineGeometryModel,
    pub assumptions: GeometryAssumptions,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EngineGeometryModel {
    pub chamber_length_m: f64,
    pub converging_length_m: f64,
    pub diverging_length_m: f64,
    pub throat_axial_m: f64,
    pub exit_axial_m: f64,
    pub chamber_diameter_m: f64,
    pub throat_diameter_m: f64,
    pub exit_diameter_m: f64,
    pub wall_contour_upper: Vec<[f64; 2]>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeometryAssumptions {
    pub sizing_mode: GeometrySizingMode,
    pub chamber_contraction_ratio: f64,
    pub characteristic_length_m: f64,
    pub nozzle_half_angle_deg: f64,
    pub nozzle_contour_style: NozzleContourStyle,
    pub nozzle_truncation_ratio: f64,
    pub combustor_model: crate::CombustorModel,
    pub nozzle_chemistry_model: crate::NozzleChemistryModel,
}

pub fn compute_geometry(problem: &RocketGeometryProblem) -> Result<RocketGeometryResult, RpaError> {
    validate_geometry_problem(problem)?;

    let expansion_ratio = match problem.base_problem.nozzle_constraint {
        NozzleConstraint::ExpansionRatio(value) => value,
        NozzleConstraint::ExitPressurePa(_) => {
            return Err(RpaError::UnsupportedAssumption(
                "geometry sizing currently requires ExpansionRatio nozzle constraint".to_owned(),
            ));
        }
    };

    let throat_area_m2 = match problem.sizing_mode {
        GeometrySizingMode::GivenThroatDiameter => {
            std::f64::consts::PI * (problem.throat_input_value * 0.5).powi(2)
        }
        GeometrySizingMode::GivenThroatArea => problem.throat_input_value,
    };

    let throat_diameter_m = (4.0 * throat_area_m2 / std::f64::consts::PI).sqrt();
    let exit_area_m2 = expansion_ratio * throat_area_m2;
    let exit_diameter_m = (4.0 * exit_area_m2 / std::f64::consts::PI).sqrt();

    let chamber_area_m2_estimate = problem.chamber_contraction_ratio * throat_area_m2;
    let chamber_diameter_m_estimate =
        (4.0 * chamber_area_m2_estimate / std::f64::consts::PI).sqrt();
    let chamber_volume_m3 = problem.characteristic_length_m * throat_area_m2;
    let chamber_length_m_estimate = chamber_volume_m3 / chamber_area_m2_estimate;

    let throat_radius = 0.5 * throat_diameter_m;
    let exit_radius = 0.5 * exit_diameter_m;
    let nozzle_half_angle_rad = problem.nozzle_half_angle_deg.to_radians();
    let base_diverging_length = (exit_radius - throat_radius) / nozzle_half_angle_rad.tan();
    let nozzle_length_factor = match problem.nozzle_contour_style {
        NozzleContourStyle::Conical => 1.0,
        NozzleContourStyle::BellParabolic => 0.82,
        NozzleContourStyle::TruncatedIdeal => 0.70,
    };
    let nozzle_length_m_estimate =
        base_diverging_length * nozzle_length_factor * problem.nozzle_truncation_ratio;

    let converging_length_m = 0.5 * (chamber_diameter_m_estimate - throat_diameter_m).max(0.0);
    let throat_axial_m = chamber_length_m_estimate + converging_length_m;
    let exit_axial_m = throat_axial_m + nozzle_length_m_estimate;
    let canonical_model = build_canonical_geometry_model(
        chamber_length_m_estimate,
        converging_length_m,
        nozzle_length_m_estimate,
        chamber_diameter_m_estimate,
        throat_diameter_m,
        exit_diameter_m,
        problem.nozzle_contour_style,
    );

    Ok(RocketGeometryResult {
        throat_area_m2,
        throat_diameter_m,
        exit_area_m2,
        exit_diameter_m,
        expansion_ratio,
        chamber_area_m2_estimate,
        chamber_diameter_m_estimate,
        chamber_length_m_estimate,
        nozzle_length_m_estimate,
        chamber_pressure_pa_reference: problem.base_problem.chamber_pressure_pa,
        canonical_model,
        assumptions: GeometryAssumptions {
            sizing_mode: problem.sizing_mode,
            chamber_contraction_ratio: problem.chamber_contraction_ratio,
            characteristic_length_m: problem.characteristic_length_m,
            nozzle_half_angle_deg: problem.nozzle_half_angle_deg,
            nozzle_contour_style: problem.nozzle_contour_style,
            nozzle_truncation_ratio: problem.nozzle_truncation_ratio,
            combustor_model: problem.base_problem.combustor_model.clone(),
            nozzle_chemistry_model: problem.base_problem.nozzle_chemistry_model.clone(),
        },
        notes: vec![
            "First-pass geometry sizing only; no contour optimization or CAD export.".to_owned(),
            "Chamber length uses characteristic length L* and assumed chamber contraction ratio."
                .to_owned(),
            "Nozzle contour is first-pass and style-driven (conical/bell/truncated ideal)."
                .to_owned(),
            format!(
                "Canonical geometry stations: throat @ {:.4} m, exit @ {:.4} m.",
                throat_axial_m, exit_axial_m
            ),
        ],
    })
}

fn build_canonical_geometry_model(
    chamber_length_m: f64,
    converging_length_m: f64,
    diverging_length_m: f64,
    chamber_diameter_m: f64,
    throat_diameter_m: f64,
    exit_diameter_m: f64,
    style: NozzleContourStyle,
) -> EngineGeometryModel {
    let throat_axial_m = chamber_length_m + converging_length_m;
    let exit_axial_m = throat_axial_m + diverging_length_m;

    let mut wall_contour_upper = Vec::new();
    let chamber_r = 0.5 * chamber_diameter_m;
    let throat_r = 0.5 * throat_diameter_m;
    let exit_r = 0.5 * exit_diameter_m;
    wall_contour_upper.push([0.0, chamber_r]);
    wall_contour_upper.push([chamber_length_m, chamber_r]);

    // Smooth converging contour to avoid a hard chamber-to-throat corner.
    let n_conv = 20usize;
    for i in 1..=n_conv {
        let f = (i as f64) / (n_conv as f64);
        let s = smoothstep01(f);
        let x = chamber_length_m + f * converging_length_m;
        let r = chamber_r + s * (throat_r - chamber_r);
        wall_contour_upper.push([x, r]);
    }

    // Diverging section with a short throat blend region for cleaner curvature.
    let n_div = 40usize;
    let throat_blend_fraction = 0.15;
    for i in 1..=n_div {
        let f = (i as f64) / (n_div as f64);
        let x = throat_axial_m + f * diverging_length_m;
        let style_shape = match style {
            NozzleContourStyle::Conical => f,
            NozzleContourStyle::BellParabolic => smoothstep01(f),
            NozzleContourStyle::TruncatedIdeal => f.powf(0.65),
        };
        let blend = smoothstep01((f / throat_blend_fraction).min(1.0));
        let shape = style_shape * blend;
        let radius = throat_r + shape * (exit_r - throat_r);
        wall_contour_upper.push([x, radius]);
    }

    EngineGeometryModel {
        chamber_length_m,
        converging_length_m,
        diverging_length_m,
        throat_axial_m,
        exit_axial_m,
        chamber_diameter_m,
        throat_diameter_m,
        exit_diameter_m,
        wall_contour_upper,
    }
}

fn smoothstep01(f: f64) -> f64 {
    let t = f.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn validate_geometry_problem(problem: &RocketGeometryProblem) -> Result<(), RpaError> {
    if !problem.throat_input_value.is_finite() || problem.throat_input_value <= 0.0 {
        return Err(RpaError::InvalidInput(
            "geometry throat input must be finite and > 0".to_owned(),
        ));
    }

    if !problem.chamber_contraction_ratio.is_finite() || problem.chamber_contraction_ratio <= 1.0 {
        return Err(RpaError::InvalidInput(
            "chamber contraction ratio must be finite and > 1".to_owned(),
        ));
    }

    if !problem.characteristic_length_m.is_finite() || problem.characteristic_length_m <= 0.0 {
        return Err(RpaError::InvalidInput(
            "characteristic length L* must be finite and > 0".to_owned(),
        ));
    }

    if !problem.nozzle_half_angle_deg.is_finite()
        || problem.nozzle_half_angle_deg <= 1.0
        || problem.nozzle_half_angle_deg >= 45.0
    {
        return Err(RpaError::InvalidInput(
            "nozzle half-angle must be finite and between 1 and 45 degrees".to_owned(),
        ));
    }

    if matches!(problem.sizing_mode, GeometrySizingMode::GivenThroatArea)
        && problem.throat_input_value < 1.0e-8
    {
        return Err(RpaError::InvalidInput(
            "throat area input is too small for robust preview scaling".to_owned(),
        ));
    }

    if !problem.nozzle_truncation_ratio.is_finite()
        || problem.nozzle_truncation_ratio <= 0.4
        || problem.nozzle_truncation_ratio > 1.0
    {
        return Err(RpaError::InvalidInput(
            "nozzle truncation ratio must be within (0.4, 1.0]".to_owned(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use tf_cea::Reactant;

    use super::*;

    fn base_problem() -> RocketAnalysisProblem {
        RocketAnalysisProblem {
            oxidizer: Reactant {
                name: "LOX".to_owned(),
                amount_moles: 1.0,
                temperature_k: Some(90.0),
            },
            fuel: Reactant {
                name: "RP-1".to_owned(),
                amount_moles: 1.0,
                temperature_k: Some(293.0),
            },
            chamber_pressure_pa: 7.0e6,
            mixture_ratio_oxidizer_to_fuel: 2.6,
            nozzle_constraint: NozzleConstraint::ExpansionRatio(20.0),
            combustor_model: crate::CombustorModel::InfiniteArea,
            nozzle_chemistry_model: crate::NozzleChemistryModel::ShiftingEquilibrium,
            ambient_pressure_pa: 101_325.0,
        }
    }

    #[test]
    fn computes_geometry_from_throat_diameter() {
        let problem = RocketGeometryProblem {
            base_problem: base_problem(),
            sizing_mode: GeometrySizingMode::GivenThroatDiameter,
            throat_input_value: 0.1,
            chamber_contraction_ratio: 3.0,
            characteristic_length_m: 1.2,
            nozzle_half_angle_deg: 15.0,
            nozzle_contour_style: NozzleContourStyle::Conical,
            nozzle_truncation_ratio: 1.0,
        };

        let result = compute_geometry(&problem).expect("geometry");
        assert!(result.throat_area_m2 > 0.0);
        assert!(result.exit_area_m2 > result.throat_area_m2);
        assert!(result.exit_diameter_m > result.throat_diameter_m);
        assert!(result.nozzle_length_m_estimate > 0.0);
        assert!(!result.canonical_model.wall_contour_upper.is_empty());
    }

    #[test]
    fn rejects_exit_pressure_mode_for_now() {
        let mut base = base_problem();
        base.nozzle_constraint = NozzleConstraint::ExitPressurePa(50_000.0);

        let problem = RocketGeometryProblem {
            base_problem: base,
            sizing_mode: GeometrySizingMode::GivenThroatArea,
            throat_input_value: 0.01,
            chamber_contraction_ratio: 3.0,
            characteristic_length_m: 1.2,
            nozzle_half_angle_deg: 15.0,
            nozzle_contour_style: NozzleContourStyle::Conical,
            nozzle_truncation_ratio: 1.0,
        };

        let err = compute_geometry(&problem).expect_err("must fail");
        assert!(err.to_string().contains("ExpansionRatio"));
    }
    #[test]
    fn contour_style_and_truncation_change_nozzle_length() {
        let mut conical = RocketGeometryProblem {
            base_problem: base_problem(),
            sizing_mode: GeometrySizingMode::GivenThroatDiameter,
            throat_input_value: 0.1,
            chamber_contraction_ratio: 3.0,
            characteristic_length_m: 1.2,
            nozzle_half_angle_deg: 15.0,
            nozzle_contour_style: NozzleContourStyle::Conical,
            nozzle_truncation_ratio: 1.0,
        };

        let conical_result = compute_geometry(&conical).expect("conical");
        conical.nozzle_contour_style = NozzleContourStyle::BellParabolic;
        let bell_result = compute_geometry(&conical).expect("bell");
        assert!(bell_result.nozzle_length_m_estimate < conical_result.nozzle_length_m_estimate);

        conical.nozzle_contour_style = NozzleContourStyle::TruncatedIdeal;
        conical.nozzle_truncation_ratio = 0.7;
        let truncated_result = compute_geometry(&conical).expect("truncated");
        assert!(truncated_result.nozzle_length_m_estimate < bell_result.nozzle_length_m_estimate);
    }

    #[test]
    fn rejects_invalid_truncation_ratio() {
        let problem = RocketGeometryProblem {
            base_problem: base_problem(),
            sizing_mode: GeometrySizingMode::GivenThroatDiameter,
            throat_input_value: 0.1,
            chamber_contraction_ratio: 3.0,
            characteristic_length_m: 1.2,
            nozzle_half_angle_deg: 15.0,
            nozzle_contour_style: NozzleContourStyle::Conical,
            nozzle_truncation_ratio: 0.3,
        };

        let err = compute_geometry(&problem).expect_err("must fail");
        assert!(err.to_string().contains("truncation"));
    }
}
