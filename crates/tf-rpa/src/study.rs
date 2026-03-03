use serde::{Deserialize, Serialize};
use tf_cea::CeaBackend;

use crate::{RocketAnalysisProblem, RocketAnalysisResult, RocketAnalysisSolver, RpaError};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RocketStudyProblem {
    pub base_problem: RocketAnalysisProblem,
    pub variable: StudyVariable,
    pub range: StudyRange,
    pub outputs: Vec<StudyOutputMetric>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StudyVariable {
    ChamberPressurePa,
    MixtureRatio,
    AmbientPressurePa,
    ExpansionRatio,
}

impl StudyVariable {
    pub fn label(&self) -> &'static str {
        match self {
            Self::ChamberPressurePa => "Chamber Pressure [Pa]",
            Self::MixtureRatio => "O/F Mixture Ratio",
            Self::AmbientPressurePa => "Ambient Pressure [Pa]",
            Self::ExpansionRatio => "Nozzle Expansion Ratio [-]",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StudyRange {
    pub min: f64,
    pub max: f64,
    pub point_count: usize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StudyOutputMetric {
    ChamberTemperatureK,
    ChamberGamma,
    ChamberMolecularWeightKgPerKmol,
    CharacteristicVelocityMPerS,
    ThrustCoefficientVac,
    SpecificImpulseVacS,
    SpecificImpulseAmbS,
    EffectiveExhaustVelocityVacMPerS,
    EffectiveExhaustVelocityAmbMPerS,
    ChamberToAmbientPressureRatio,
}

impl StudyOutputMetric {
    pub fn label(&self) -> &'static str {
        match self {
            Self::ChamberTemperatureK => "Chamber Temperature [K]",
            Self::ChamberGamma => "Chamber Gamma [-]",
            Self::ChamberMolecularWeightKgPerKmol => "Chamber MW [kg/kmol]",
            Self::CharacteristicVelocityMPerS => "c* [m/s]",
            Self::ThrustCoefficientVac => "Cf,vac [-]",
            Self::SpecificImpulseVacS => "Isp,vac [s]",
            Self::SpecificImpulseAmbS => "Isp,amb [s]",
            Self::EffectiveExhaustVelocityVacMPerS => "c_eff,vac [m/s]",
            Self::EffectiveExhaustVelocityAmbMPerS => "c_eff,amb [m/s]",
            Self::ChamberToAmbientPressureRatio => "Pc/Pa [-]",
        }
    }

    pub fn extract(&self, result: &RocketAnalysisResult) -> f64 {
        match self {
            Self::ChamberTemperatureK => result.chamber.temperature_k.unwrap_or(f64::NAN),
            Self::ChamberGamma => result.chamber.gamma.unwrap_or(f64::NAN),
            Self::ChamberMolecularWeightKgPerKmol => result
                .chamber
                .molecular_weight_kg_per_kmol
                .unwrap_or(f64::NAN),
            Self::CharacteristicVelocityMPerS => result.characteristic_velocity_m_per_s,
            Self::ThrustCoefficientVac => result.thrust_coefficient_vac,
            Self::SpecificImpulseVacS => result.specific_impulse_vac_s,
            Self::SpecificImpulseAmbS => result.specific_impulse_amb_s,
            Self::EffectiveExhaustVelocityVacMPerS => result.effective_exhaust_velocity_vac_m_per_s,
            Self::EffectiveExhaustVelocityAmbMPerS => result.effective_exhaust_velocity_amb_m_per_s,
            Self::ChamberToAmbientPressureRatio => result.chamber_to_ambient_pressure_ratio,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RocketStudyPoint {
    pub sweep_value: f64,
    pub outputs: Vec<MetricValue>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetricValue {
    pub metric: StudyOutputMetric,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RocketStudyResult {
    pub variable: StudyVariable,
    pub outputs: Vec<StudyOutputMetric>,
    pub points: Vec<RocketStudyPoint>,
    pub warning_messages: Vec<String>,
}

pub fn run_single_variable_study<B: CeaBackend>(
    solver: &RocketAnalysisSolver<B>,
    problem: &RocketStudyProblem,
) -> Result<RocketStudyResult, RpaError> {
    validate_study_problem(problem)?;

    let sweep = build_linear_sweep(&problem.range);
    let mut points = Vec::with_capacity(sweep.len());
    let mut warnings = Vec::new();

    for value in sweep {
        let mut case = problem.base_problem.clone();
        if let Err(err) = apply_study_variable(&mut case, problem.variable, value) {
            points.push(RocketStudyPoint {
                sweep_value: value,
                outputs: Vec::new(),
                error: Some(err.to_string()),
            });
            continue;
        }

        match solver.solve(&case) {
            Ok(result) => {
                let outputs = problem
                    .outputs
                    .iter()
                    .map(|metric| MetricValue {
                        metric: *metric,
                        value: metric.extract(&result),
                    })
                    .collect();
                points.push(RocketStudyPoint {
                    sweep_value: value,
                    outputs,
                    error: None,
                });
            }
            Err(err) => {
                warnings.push(format!(
                    "study point {}={} failed: {}",
                    problem.variable.label(),
                    value,
                    err
                ));
                points.push(RocketStudyPoint {
                    sweep_value: value,
                    outputs: Vec::new(),
                    error: Some(err.to_string()),
                });
            }
        }
    }

    Ok(RocketStudyResult {
        variable: problem.variable,
        outputs: problem.outputs.clone(),
        points,
        warning_messages: warnings,
    })
}

fn validate_study_problem(problem: &RocketStudyProblem) -> Result<(), RpaError> {
    if problem.outputs.is_empty() {
        return Err(RpaError::InvalidInput(
            "study outputs must include at least one metric".to_owned(),
        ));
    }

    if !problem.range.min.is_finite() || !problem.range.max.is_finite() {
        return Err(RpaError::InvalidInput(
            "study range bounds must be finite".to_owned(),
        ));
    }

    if problem.range.max <= problem.range.min {
        return Err(RpaError::InvalidInput(
            "study range max must be greater than min".to_owned(),
        ));
    }

    if problem.range.point_count < 2 {
        return Err(RpaError::InvalidInput(
            "study point_count must be >= 2".to_owned(),
        ));
    }

    if matches!(problem.variable, StudyVariable::ExpansionRatio)
        && !matches!(
            problem.base_problem.nozzle_constraint,
            crate::NozzleConstraint::ExpansionRatio(_)
        )
    {
        return Err(RpaError::UnsupportedAssumption(
            "expansion-ratio sweep requires base nozzle constraint to be ExpansionRatio".to_owned(),
        ));
    }

    Ok(())
}

fn build_linear_sweep(range: &StudyRange) -> Vec<f64> {
    let step = (range.max - range.min) / ((range.point_count - 1) as f64);
    (0..range.point_count)
        .map(|i| range.min + step * (i as f64))
        .collect()
}

fn apply_study_variable(
    problem: &mut RocketAnalysisProblem,
    variable: StudyVariable,
    value: f64,
) -> Result<(), RpaError> {
    if !value.is_finite() {
        return Err(RpaError::InvalidInput(
            "study sweep value must be finite".to_owned(),
        ));
    }

    match variable {
        StudyVariable::ChamberPressurePa => problem.chamber_pressure_pa = value,
        StudyVariable::MixtureRatio => problem.mixture_ratio_oxidizer_to_fuel = value,
        StudyVariable::AmbientPressurePa => problem.ambient_pressure_pa = value,
        StudyVariable::ExpansionRatio => match &mut problem.nozzle_constraint {
            crate::NozzleConstraint::ExpansionRatio(expansion_ratio) => *expansion_ratio = value,
            crate::NozzleConstraint::ExitPressurePa(_) => {
                return Err(RpaError::UnsupportedAssumption(
                    "cannot apply expansion ratio sweep to exit-pressure constrained base case"
                        .to_owned(),
                ));
            }
        },
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use tf_cea::{
        CeaBackend, CeaError, EquilibriumProblem, EquilibriumResult, Reactant, RocketProblem,
        RocketResult,
    };

    use super::*;

    #[derive(Clone, Default)]
    struct FakeBackend;

    impl CeaBackend for FakeBackend {
        fn run_equilibrium(
            &self,
            _problem: &EquilibriumProblem,
        ) -> Result<EquilibriumResult, CeaError> {
            Err(CeaError::InvalidResponse("unused".to_owned()))
        }

        fn run_rocket(&self, problem: &RocketProblem) -> Result<RocketResult, CeaError> {
            let chamber_temperature_k = 3000.0 + problem.mixture_ratio_oxidizer_to_fuel * 50.0;
            Ok(RocketResult {
                chamber_temperature_k,
                chamber_gamma: 1.2,
                chamber_molecular_weight_kg_per_kmol: 22.0,
                characteristic_velocity_m_per_s: 1800.0,
                specific_impulse_vac_s: 320.0,
                thrust_coefficient_vac: 1.8,
            })
        }
    }

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
            nozzle_constraint: crate::NozzleConstraint::ExpansionRatio(40.0),
            combustor_model: crate::CombustorModel::InfiniteArea,
            nozzle_chemistry_model: crate::NozzleChemistryModel::ShiftingEquilibrium,
            ambient_pressure_pa: 101_325.0,
        }
    }

    #[test]
    fn runs_single_variable_study() {
        let solver = RocketAnalysisSolver::new(FakeBackend);
        let study = RocketStudyProblem {
            base_problem: base_problem(),
            variable: StudyVariable::MixtureRatio,
            range: StudyRange {
                min: 2.0,
                max: 3.0,
                point_count: 3,
            },
            outputs: vec![
                StudyOutputMetric::ChamberTemperatureK,
                StudyOutputMetric::SpecificImpulseVacS,
            ],
        };

        let result = run_single_variable_study(&solver, &study).expect("study");
        assert_eq!(result.points.len(), 3);
        assert!(result.points.iter().all(|p| p.error.is_none()));
        assert_eq!(result.points[0].outputs.len(), 2);
    }

    #[test]
    fn rejects_empty_outputs() {
        let solver = RocketAnalysisSolver::new(FakeBackend);
        let study = RocketStudyProblem {
            base_problem: base_problem(),
            variable: StudyVariable::MixtureRatio,
            range: StudyRange {
                min: 2.0,
                max: 3.0,
                point_count: 3,
            },
            outputs: vec![],
        };

        let err = run_single_variable_study(&solver, &study).expect_err("expected invalid");
        assert!(err.to_string().contains("at least one metric"));
    }
}
