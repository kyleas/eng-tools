use crate::error::CeaError;
use crate::model::{EquilibriumProblem, EquilibriumResult, RocketProblem, RocketResult};

pub trait CeaBackend {
    fn run_equilibrium(&self, problem: &EquilibriumProblem) -> Result<EquilibriumResult, CeaError>;
    fn run_rocket(&self, problem: &RocketProblem) -> Result<RocketResult, CeaError>;
}
