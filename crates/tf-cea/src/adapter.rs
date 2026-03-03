use std::io::Write;
use std::process::{Command, Stdio};

use crate::backend::CeaBackend;
use crate::config::CeaBackendConfig;
use crate::error::CeaError;
use crate::model::{
    BackendProblem, BackendResult, EquilibriumProblem, EquilibriumResult, RocketProblem,
    RocketResult,
};

/// Process adapter that shells out to an existing CEA backend bridge.
///
/// The executable contract is simple JSON over stdio:
/// - stdin: [`BackendProblem`]
/// - stdout: [`BackendResult`]
pub struct CeaProcessAdapter {
    config: CeaBackendConfig,
}

impl CeaProcessAdapter {
    pub fn new(config: CeaBackendConfig) -> Self {
        Self { config }
    }

    fn run_problem(&self, problem: &BackendProblem) -> Result<BackendResult, CeaError> {
        let executable = self
            .config
            .executable
            .as_ref()
            .ok_or(CeaError::MissingExecutable)?;

        let payload = serde_json::to_vec(problem)?;
        let mut child = Command::new(&executable.path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        {
            let stdin = child
                .stdin
                .as_mut()
                .ok_or_else(|| CeaError::InvalidResponse("missing stdin handle".to_owned()))?;
            stdin.write_all(&payload)?;
        }

        let output = child.wait_with_output()?;
        if !output.status.success() {
            return Err(CeaError::ProcessFailure {
                status: output.status.code().unwrap_or(-1),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            });
        }

        let result: BackendResult = serde_json::from_slice(&output.stdout)?;
        Ok(result)
    }
}

impl CeaBackend for CeaProcessAdapter {
    fn run_equilibrium(&self, problem: &EquilibriumProblem) -> Result<EquilibriumResult, CeaError> {
        let result = self.run_problem(&BackendProblem::Equilibrium(problem.clone()))?;
        match result {
            BackendResult::Equilibrium(data) => Ok(data),
            BackendResult::Rocket(_) => Err(CeaError::InvalidResponse(
                "received rocket response for equilibrium request".to_owned(),
            )),
        }
    }

    fn run_rocket(&self, problem: &RocketProblem) -> Result<RocketResult, CeaError> {
        let result = self.run_problem(&BackendProblem::Rocket(problem.clone()))?;
        match result {
            BackendResult::Rocket(data) => Ok(data),
            BackendResult::Equilibrium(_) => Err(CeaError::InvalidResponse(
                "received equilibrium response for rocket request".to_owned(),
            )),
        }
    }
}
