use serde::{Deserialize, Serialize};
use tf_cea::Reactant;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RocketAnalysisProblem {
    pub oxidizer: Reactant,
    pub fuel: Reactant,
    pub chamber_pressure_pa: f64,
    pub mixture_ratio_oxidizer_to_fuel: f64,
    pub nozzle_constraint: NozzleConstraint,
    pub combustor_model: CombustorModel,
    pub nozzle_chemistry_model: NozzleChemistryModel,
    pub ambient_pressure_pa: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NozzleConstraint {
    ExpansionRatio(f64),
    ExitPressurePa(f64),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CombustorModel {
    InfiniteArea,
    FiniteArea { contraction_ratio: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NozzleChemistryModel {
    ShiftingEquilibrium,
    FrozenAtChamber,
    FrozenAtThroat,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StateSummary {
    pub temperature_k: Option<f64>,
    pub pressure_pa: Option<f64>,
    pub gamma: Option<f64>,
    pub molecular_weight_kg_per_kmol: Option<f64>,
    pub source: StateSource,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StateSource {
    CeaBackend,
    NotYetProvidedByBackend,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RocketAnalysisResult {
    pub chamber: StateSummary,
    pub throat: StateSummary,
    pub exit: StateSummary,
    pub characteristic_velocity_m_per_s: f64,
    pub thrust_coefficient_vac: f64,
    pub thrust_coefficient_amb: f64,
    pub specific_impulse_vac_s: f64,
    pub specific_impulse_amb_s: f64,
    pub effective_exhaust_velocity_vac_m_per_s: f64,
    pub effective_exhaust_velocity_amb_m_per_s: f64,
    pub expansion_ratio_used: f64,
    pub ambient_pressure_pa: f64,
    pub chamber_pressure_pa: f64,
    pub chamber_to_ambient_pressure_ratio: f64,
    pub assumptions: AssumptionMetadata,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AssumptionMetadata {
    pub combustor_model: CombustorModel,
    pub nozzle_chemistry_model: NozzleChemistryModel,
    pub nozzle_constraint: NozzleConstraint,
}
