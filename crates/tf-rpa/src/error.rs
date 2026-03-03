use thiserror::Error;

#[derive(Debug, Error)]
pub enum RpaError {
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("unsupported assumption: {0}")]
    UnsupportedAssumption(String),
    #[error("thermochemistry backend failure: {0}")]
    Backend(String),
}
