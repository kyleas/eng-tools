use indexmap::IndexMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SolveConfig {
    pub default_target: Option<String>,
    #[serde(default)]
    pub explicit_forms: IndexMap<String, String>,
    #[serde(default)]
    pub numerical: NumericalConfig,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct NumericalConfig {
    #[serde(default)]
    pub unsupported_targets: Vec<String>,
    #[serde(default)]
    pub solver: SolverKind,
    pub initial_guess: Option<f64>,
    pub bracket: Option<[f64; 2]>,
    pub tolerance_abs: Option<f64>,
    pub tolerance_rel: Option<f64>,
    pub max_iter: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SolverKind {
    #[default]
    Bisection,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BranchDef {
    pub name: String,
    pub condition: String,
    #[serde(default)]
    pub preferred: bool,
}
