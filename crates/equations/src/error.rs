use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum EquationError {
    #[error("io error at {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("yaml parse error in {path}: {source}")]
    Yaml {
        path: PathBuf,
        #[source]
        source: serde_yaml::Error,
    },
    #[error("json serialization error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("schema generation error: {0}")]
    Schema(String),
    #[error("registry validation error: {0}")]
    Validation(String),
    #[error("expression parse error in '{expression}': {message}")]
    ExpressionParse { expression: String, message: String },
    #[error("expression evaluation error in '{expression}': {message}")]
    ExpressionEval { expression: String, message: String },
    #[error("unknown symbol '{symbol}' in expression '{expression}'")]
    UnknownSymbol { expression: String, symbol: String },
    #[error(
        "invalid solve target '{target}' for equation '{equation_key}'. valid targets: {valid_targets}"
    )]
    InvalidSolveTarget {
        equation_key: String,
        target: String,
        valid_targets: String,
    },
    #[error("unknown equation id '{id}' (lookup supports key, path_id, or alias){suggestion}")]
    UnknownEquationId { id: String, suggestion: String },
    #[error(
        "unsupported branch '{branch}' for equation '{equation_key}'. valid branches: {valid_branches}{suggestion}"
    )]
    InvalidBranch {
        equation_key: String,
        branch: String,
        valid_branches: String,
        suggestion: String,
    },
    #[error("numerical solver failed for equation '{equation_key}' target '{target}': {reason}")]
    NumericalSolve {
        equation_key: String,
        target: String,
        reason: String,
    },
    #[error("unit parse/conversion error for variable '{variable}': {message}")]
    Unit { variable: String, message: String },
    #[error("constraint violation for variable '{variable}': {message}")]
    Constraint { variable: String, message: String },
    #[error("equation test failed: {0}")]
    TestFailure(String),
}

pub type Result<T> = std::result::Result<T, EquationError>;
