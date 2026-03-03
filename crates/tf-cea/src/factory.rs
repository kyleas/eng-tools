use crate::adapter::CeaProcessAdapter;
use crate::backend::CeaBackend;
use crate::config::{CeaBackendConfig, CeaBackendMode};
use crate::error::CeaError;
use crate::model::{EquilibriumProblem, EquilibriumResult, RocketProblem, RocketResult};
use crate::native::NativeCeaBackend;

pub enum SelectedCeaBackend {
    Process(CeaProcessAdapter),
    Native(NativeCeaBackend),
}

impl CeaBackend for SelectedCeaBackend {
    fn run_equilibrium(&self, problem: &EquilibriumProblem) -> Result<EquilibriumResult, CeaError> {
        match self {
            Self::Process(backend) => backend.run_equilibrium(problem),
            Self::Native(backend) => backend.run_equilibrium(problem),
        }
    }

    fn run_rocket(&self, problem: &RocketProblem) -> Result<RocketResult, CeaError> {
        match self {
            Self::Process(backend) => backend.run_rocket(problem),
            Self::Native(backend) => backend.run_rocket(problem),
        }
    }
}

pub fn create_backend(config: CeaBackendConfig) -> Result<SelectedCeaBackend, CeaError> {
    match config.mode {
        CeaBackendMode::Process => {
            if config.executable.is_none() {
                return Err(CeaError::MissingExecutable);
            }
            Ok(SelectedCeaBackend::Process(CeaProcessAdapter::new(config)))
        }
        CeaBackendMode::InProcess => Ok(SelectedCeaBackend::Native(NativeCeaBackend::new())),
    }
}
