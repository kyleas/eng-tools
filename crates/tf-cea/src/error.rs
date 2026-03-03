use thiserror::Error;

#[derive(Debug, Error)]
pub enum CeaError {
    #[error("backend executable not configured")]
    MissingExecutable,
    #[error("backend process failed with status {status}: {stderr}")]
    ProcessFailure { status: i32, stderr: String },
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("invalid backend response: {0}")]
    InvalidResponse(String),
    #[error("backend error: {0}")]
    BackendError(String),
}
