use tf_cea::RocketProblem;
use tf_cea::{
    CeaBackend, CeaError, CombustorModel as CeaCombustorModel, NozzleModel as CeaNozzleModel,
};

use crate::error::RpaError;
use crate::model::{
    AssumptionMetadata, CombustorModel, NozzleChemistryModel, NozzleConstraint,
    RocketAnalysisProblem, RocketAnalysisResult, StateSource, StateSummary,
};

const STANDARD_GRAVITY_M_PER_S2: f64 = 9.80665;

pub struct RocketAnalysisSolver<B: CeaBackend> {
    backend: B,
}

impl<B: CeaBackend> RocketAnalysisSolver<B> {
    pub fn new(backend: B) -> Self {
        Self { backend }
    }

    pub fn solve(&self, problem: &RocketAnalysisProblem) -> Result<RocketAnalysisResult, RpaError> {
        validate_problem(problem)?;

        let expansion_ratio = match problem.nozzle_constraint {
            NozzleConstraint::ExpansionRatio(eps) => eps,
            NozzleConstraint::ExitPressurePa(exit_pressure) => {
                return Err(RpaError::UnsupportedAssumption(format!(
                    "exit-pressure constrained nozzle solve is not yet supported by tf-cea adapter (requested pe={} Pa)",
                    exit_pressure
                )));
            }
        };

        let nozzle_model = map_nozzle_chemistry(problem.nozzle_chemistry_model.clone())?;
        let combustor_model = map_combustor(problem.combustor_model.clone());

        let cea_problem = RocketProblem {
            oxidizer: problem.oxidizer.clone(),
            fuel: problem.fuel.clone(),
            chamber_pressure_pa: problem.chamber_pressure_pa,
            mixture_ratio_oxidizer_to_fuel: problem.mixture_ratio_oxidizer_to_fuel,
            expansion_ratio,
            nozzle_model,
            combustor_model,
        };

        let rocket = self
            .backend
            .run_rocket(&cea_problem)
            .map_err(map_backend_error)?;

        let thrust_coefficient_amb = rocket.thrust_coefficient_vac
            - (problem.ambient_pressure_pa / problem.chamber_pressure_pa) * expansion_ratio;
        let specific_impulse_amb_s = thrust_coefficient_amb
            * rocket.characteristic_velocity_m_per_s
            / STANDARD_GRAVITY_M_PER_S2;

        let effective_exhaust_velocity_vac_m_per_s =
            rocket.specific_impulse_vac_s * STANDARD_GRAVITY_M_PER_S2;
        let effective_exhaust_velocity_amb_m_per_s =
            specific_impulse_amb_s * STANDARD_GRAVITY_M_PER_S2;
        let chamber_to_ambient_pressure_ratio =
            problem.chamber_pressure_pa / problem.ambient_pressure_pa;

        let notes = vec![
            "Chamber state, c*, vacuum Isp, and vacuum Cf are provided directly by the CEA backend."
                .to_owned(),
            "Throat and exit thermodynamic summaries are placeholder seams until tf-cea exposes station-level outputs."
                .to_owned(),
            "Ambient performance currently uses standard rocket relation: Cf_amb = Cf_vac - (pa/pc) * eps."
                .to_owned(),
        ];

        Ok(RocketAnalysisResult {
            chamber: StateSummary {
                temperature_k: Some(rocket.chamber_temperature_k),
                pressure_pa: Some(problem.chamber_pressure_pa),
                gamma: Some(rocket.chamber_gamma),
                molecular_weight_kg_per_kmol: Some(rocket.chamber_molecular_weight_kg_per_kmol),
                source: StateSource::CeaBackend,
            },
            throat: StateSummary {
                temperature_k: None,
                pressure_pa: None,
                gamma: None,
                molecular_weight_kg_per_kmol: None,
                source: StateSource::NotYetProvidedByBackend,
            },
            exit: StateSummary {
                temperature_k: None,
                pressure_pa: None,
                gamma: None,
                molecular_weight_kg_per_kmol: None,
                source: StateSource::NotYetProvidedByBackend,
            },
            characteristic_velocity_m_per_s: rocket.characteristic_velocity_m_per_s,
            thrust_coefficient_vac: rocket.thrust_coefficient_vac,
            thrust_coefficient_amb,
            specific_impulse_vac_s: rocket.specific_impulse_vac_s,
            specific_impulse_amb_s,
            effective_exhaust_velocity_vac_m_per_s,
            effective_exhaust_velocity_amb_m_per_s,
            expansion_ratio_used: expansion_ratio,
            ambient_pressure_pa: problem.ambient_pressure_pa,
            chamber_pressure_pa: problem.chamber_pressure_pa,
            chamber_to_ambient_pressure_ratio,
            assumptions: AssumptionMetadata {
                combustor_model: problem.combustor_model.clone(),
                nozzle_chemistry_model: problem.nozzle_chemistry_model.clone(),
                nozzle_constraint: problem.nozzle_constraint.clone(),
            },
            notes,
        })
    }
}

fn validate_problem(problem: &RocketAnalysisProblem) -> Result<(), RpaError> {
    for (name, value) in [
        ("chamber_pressure_pa", problem.chamber_pressure_pa),
        (
            "mixture_ratio_oxidizer_to_fuel",
            problem.mixture_ratio_oxidizer_to_fuel,
        ),
        ("ambient_pressure_pa", problem.ambient_pressure_pa),
    ] {
        if !value.is_finite() || value <= 0.0 {
            return Err(RpaError::InvalidInput(format!(
                "{name} must be finite and > 0"
            )));
        }
    }

    match problem.nozzle_constraint {
        NozzleConstraint::ExpansionRatio(eps) if eps.is_finite() && eps > 1.0 => {}
        NozzleConstraint::ExpansionRatio(_) => {
            return Err(RpaError::InvalidInput(
                "expansion ratio must be finite and > 1".to_owned(),
            ));
        }
        NozzleConstraint::ExitPressurePa(pe) if pe.is_finite() && pe > 0.0 => {}
        NozzleConstraint::ExitPressurePa(_) => {
            return Err(RpaError::InvalidInput(
                "exit pressure must be finite and > 0".to_owned(),
            ));
        }
    }

    if let CombustorModel::FiniteArea { contraction_ratio } = problem.combustor_model
        && (!contraction_ratio.is_finite() || contraction_ratio <= 1.0)
    {
        return Err(RpaError::InvalidInput(
            "finite-area contraction_ratio must be finite and > 1".to_owned(),
        ));
    }

    Ok(())
}

fn map_nozzle_chemistry(model: NozzleChemistryModel) -> Result<CeaNozzleModel, RpaError> {
    match model {
        NozzleChemistryModel::ShiftingEquilibrium => Ok(CeaNozzleModel::ShiftingEquilibrium),
        NozzleChemistryModel::FrozenAtChamber => Ok(CeaNozzleModel::Frozen),
        NozzleChemistryModel::FrozenAtThroat => Err(RpaError::UnsupportedAssumption(
            "frozen-at-throat is a planned seam and is not yet supported by tf-cea backend"
                .to_owned(),
        )),
    }
}

fn map_combustor(model: CombustorModel) -> CeaCombustorModel {
    match model {
        CombustorModel::InfiniteArea => CeaCombustorModel::InfiniteArea,
        CombustorModel::FiniteArea { contraction_ratio } => CeaCombustorModel::FiniteArea {
            area_ratio: contraction_ratio,
        },
    }
}

fn map_backend_error(error: CeaError) -> RpaError {
    match error {
        CeaError::MissingExecutable => {
            RpaError::Backend("CEA executable is not configured".to_owned())
        }
        other => RpaError::Backend(other.to_string()),
    }
}
