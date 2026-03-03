use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Reactant {
    pub name: String,
    pub amount_moles: f64,
    pub temperature_k: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ThermoState {
    PressureTemperature {
        pressure_pa: f64,
        temperature_k: f64,
    },
    PressureEnthalpy {
        pressure_pa: f64,
        enthalpy_j_per_kg: f64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EquilibriumProblem {
    pub reactants: Vec<Reactant>,
    pub state: ThermoState,
    pub include_condensed_species: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EquilibriumResult {
    pub pressure_pa: f64,
    pub temperature_k: f64,
    pub mean_molecular_weight_kg_per_kmol: f64,
    pub gamma: f64,
    pub species_mole_fractions: Vec<SpeciesFraction>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpeciesFraction {
    pub species: String,
    pub mole_fraction: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RocketProblem {
    pub oxidizer: Reactant,
    pub fuel: Reactant,
    pub chamber_pressure_pa: f64,
    pub mixture_ratio_oxidizer_to_fuel: f64,
    pub expansion_ratio: f64,
    pub nozzle_model: NozzleModel,
    pub combustor_model: CombustorModel,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NozzleModel {
    ShiftingEquilibrium,
    Frozen,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CombustorModel {
    InfiniteArea,
    FiniteArea { area_ratio: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RocketResult {
    pub chamber_temperature_k: f64,
    pub chamber_gamma: f64,
    pub chamber_molecular_weight_kg_per_kmol: f64,
    pub characteristic_velocity_m_per_s: f64,
    pub specific_impulse_vac_s: f64,
    pub thrust_coefficient_vac: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "problem_type", content = "payload", rename_all = "snake_case")]
pub enum BackendProblem {
    Equilibrium(EquilibriumProblem),
    Rocket(RocketProblem),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "result_type", content = "payload", rename_all = "snake_case")]
pub enum BackendResult {
    Equilibrium(EquilibriumResult),
    Rocket(RocketResult),
}
